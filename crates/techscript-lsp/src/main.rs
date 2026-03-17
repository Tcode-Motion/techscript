use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use techscript_core::lexer::Lexer;
use techscript_core::parser::Parser;
use techscript_core::compiler::Compiler;

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), " ".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                }),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "TechScript Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.check_document(params.text_document.uri, params.text_document.text).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.pop() {
            self.check_document(params.text_document.uri, change.text).await;
        }
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("TechScript Language Server".to_string())),
            range: None,
        }))
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let keywords = vec![
            "say", "ask", "make", "keep", "mut", "drop", "global", "build", "send", "model", "self", "base", "new",
            "when", "alt", "else", "each", "repeat", "in", "unless", "until", "match", "case", "stop", "skip", "pass",
            "attempt", "rescue", "fail", "always", "use", "take", "share", "as", "do", "end", "with", "defer", "guard",
            "true", "false", "none", "and", "or", "not", "is", "has", "typeof", "async", "await", "yield", "spawn"
        ];
        
        let builtins = vec![
            "abs", "round", "ceil", "floor", "min", "max", "sum", "sqrt", "pow", "clamp", "sign", "len", "size", "range", 
            "enumerate", "zip", "sorted", "reversed", "map", "filter", "int", "float", "str", "bool", "list", "type", 
            "is_int", "typeof", "print", "write", "log", "warn", "error", "clear", "format", "read_file", "write_file", 
            "append_file", "split", "join", "replace", "replace_all", "contains", "starts_with", "ends_with", "upper", 
            "lower", "trim", "chars", "reverse", "find", "sleep", "exit", "assert", "panic", "time", "time_ms", "version", 
            "callable", "random_int"
        ];

        let mut items = Vec::new();
        
        for k in keywords {
            items.push(CompletionItem {
                label: k.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }
        
        for b in builtins {
            items.push(CompletionItem {
                label: b.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                ..Default::default()
            });
        }
        
        Ok(Some(CompletionResponse::Array(items)))
    }
}

impl Backend {
    async fn check_document(&self, uri: Url, text: String) {
        let mut diagnostics = Vec::new();
        let filename = uri.to_string();
        
        let lex_result = Lexer::new(&text, &filename).tokenize();
        match lex_result {
            Ok(tokens) => {
                let parse_result = Parser::new(tokens, &filename).parse();
                match parse_result {
                    Ok(program) => {
                        let compile_result = Compiler::new().compile(&program);
                        if let Err(e) = compile_result {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position {
                                        line: (if e.line > 0 { e.line - 1 } else { 0 }) as u32,
                                        character: e.column as u32,
                                    },
                                    end: Position {
                                        line: (if e.line > 0 { e.line - 1 } else { 0 }) as u32,
                                        character: e.column as u32 + 5,
                                    }
                                },
                                severity: Some(DiagnosticSeverity::ERROR),
                                message: e.message,
                                source: Some("techscript-compiler".to_string()),
                                ..Default::default()
                            });
                        }
                    }
                    Err(e) => {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: (if e.line > 0 { e.line - 1 } else { 0 }) as u32,
                                    character: e.column as u32,
                                },
                                end: Position {
                                    line: (if e.line > 0 { e.line - 1 } else { 0 }) as u32,
                                    character: e.column as u32 + 1,
                                }
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: e.message,
                            source: Some("techscript-parser".to_string()),
                            ..Default::default()
                        });
                    }
                }
            }
            Err(e) => {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: (if e.line > 0 { e.line - 1 } else { 0 }) as u32,
                            character: e.column as u32,
                        },
                        end: Position {
                            line: (if e.line > 0 { e.line - 1 } else { 0 }) as u32,
                            character: e.column as u32 + 1,
                        }
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: e.message,
                    source: Some("techscript-lexer".to_string()),
                    ..Default::default()
                });
            }
        }
        
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend { client })
        .finish();
    
    Server::new(stdin, stdout, socket).serve(service).await;
}
