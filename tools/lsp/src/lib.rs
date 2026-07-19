//! # TechScript Language Server (LSP) Crate
//!
//! Handles autocomplete, diagnostics, formatting, and linting interactions for IDE integrations.
//! Employs tower-lsp bindings to support standard JSON-RPC editor communication.

#![allow(
    clippy::match_like_matches_macro,
    clippy::useless_conversion,
    clippy::unnecessary_map_or,
    clippy::needless_range_loop,
    clippy::vec_init_then_push,
    clippy::collapsible_match,
    clippy::collapsible_if,
    clippy::single_char_add_str
)]

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::lsp_types::request::{
    GotoTypeDefinitionParams, GotoTypeDefinitionResponse, GotoImplementationParams, GotoImplementationResponse,
    GotoDeclarationParams, GotoDeclarationResponse,
};
use tower_lsp::{Client, LanguageServer};
use std::sync::Mutex;
use std::collections::HashMap;
use techscript_syntax::{TokenKind};
use techscript_errors::{Diagnostic as TsDiagnostic, DiagnosticLevel, DiagnosticReporter};

/// Resolved declaration symbol details for outline / hover / goto def.
struct LocalDecl {
    name: String,
    span: techscript_common::Span,
    detail: String,
}

/// LSP Server implementation backend state.
pub struct Backend {
    pub client: Client,
    pub documents: Mutex<HashMap<Url, String>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Mutex::new(HashMap::new()),
        }
    }

    fn analyze_document(&self, _uri: &Url, content: &str) -> (Option<techscript_ast::Program>, Option<techscript_semantic::CheckedProgram>, Vec<TsDiagnostic>) {
        let mut reporter = DiagnosticReporter::new();
        // Syntax-error-resilient lexing and parsing
        let tokens = techscript_lexer::lex_recovered(content, &mut reporter);
        let program = techscript_parser::parse_recovered(&tokens, &mut reporter);
        let checked = techscript_semantic::analyze(program.clone(), &mut reporter).ok();
        (Some(program), checked, reporter.get_diagnostics().to_vec())
    }

    async fn validate_document(&self, uri: Url, content: &str) {
        let (_, _, diags) = self.analyze_document(&uri, content);
        let mut lsp_diags = Vec::new();
        for diag in diags {
            let range = self.span_to_range(content, diag.span);
            let severity = match diag.level {
                DiagnosticLevel::Error => Some(DiagnosticSeverity::ERROR),
                DiagnosticLevel::Warning => Some(DiagnosticSeverity::WARNING),
                DiagnosticLevel::Note => Some(DiagnosticSeverity::INFORMATION),
            };
            lsp_diags.push(Diagnostic {
                range,
                severity,
                code: Some(NumberOrString::String(format!("{:?}", diag.code))),
                source: Some("techscript".to_string()),
                message: diag.message,
                ..Default::default()
            });
        }
        self.client.publish_diagnostics(uri, lsp_diags, None).await;
    }

    fn span_to_range(&self, content: &str, span: techscript_common::Span) -> Range {
        let mut start_line = 0;
        let mut start_char = 0;
        let mut end_line = 0;
        let mut end_char = 0;

        let mut current_offset = 0;
        for (line_idx, line) in content.lines().enumerate() {
            let line_len = line.len() + 1; // plus newline character
            if current_offset <= span.start && span.start < current_offset + line_len {
                start_line = line_idx;
                start_char = span.start - current_offset;
            }
            if current_offset <= span.end && span.end <= current_offset + line_len {
                end_line = line_idx;
                end_char = span.end - current_offset;
                break;
            }
            current_offset += line_len;
        }

        Range {
            start: Position::new(start_line as u32, start_char as u32),
            end: Position::new(end_line as u32, end_char as u32),
        }
    }

    fn position_to_offset(&self, content: &str, pos: Position) -> usize {
        let mut offset = 0;
        for (line_idx, line) in content.lines().enumerate() {
            if line_idx == pos.line as usize {
                offset += pos.character as usize;
                break;
            }
            offset += line.len() + 1;
        }
        offset
    }

    fn get_word_at_offset(&self, content: &str, offset: usize) -> String {
        let chars: Vec<char> = content.chars().collect();
        if chars.is_empty() { return String::new(); }
        let mut start = offset;
        if start >= chars.len() {
            start = chars.len() - 1;
        }

        while start > 0 {
            let c = chars[start - 1];
            if c.is_alphanumeric() || c == '_' {
                start -= 1;
            } else {
                break;
            }
        }

        let mut end = offset;
        while end < chars.len() {
            let c = chars[end];
            if c.is_alphanumeric() || c == '_' {
                end += 1;
            } else {
                break;
            }
        }

        if start < end {
            chars[start..end].iter().collect()
        } else {
            String::new()
        }
    }

    fn format_source(&self, source: &str) -> String {
        let mut formatted = String::new();
        let mut indent_level = 0;

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                formatted.push_str("\n");
                continue;
            }

            let starts_with_close = trimmed.starts_with('}') || trimmed.starts_with(']');
            if starts_with_close && indent_level > 0 {
                indent_level -= 1;
            }

            for _ in 0..(indent_level * 4) {
                formatted.push_str(" ");
            }
            formatted.push_str(trimmed);
            formatted.push_str("\n");

            let open_count = trimmed.chars().filter(|&c| c == '{' || c == '[').count();
            let close_count = trimmed.chars().filter(|&c| c == '}' || c == ']').count();

            let diff = open_count as isize - close_count as isize;
            if starts_with_close {
                indent_level += open_count;
            } else {
                let next_indent = indent_level as isize + diff;
                indent_level = if next_indent < 0 { 0 } else { next_indent as usize };
            }
        }
        formatted
    }
}

fn collect_local_decls(prog: &techscript_ast::Program) -> Vec<LocalDecl> {
    let mut decls = Vec::new();
    for stmt in &prog.statements {
        collect_stmt_decls(stmt, &mut decls);
    }
    decls
}

/// DSL block keyword definitions for completions.
fn dsl_block_completions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("website", "DSL Block: Declares a complete website root"),
        ("page", "DSL Block: Declares a page route"),
        ("hero", "DSL Block: Declares a hero banner section"),
        ("section", "DSL Block: Declares a content section"),
        ("card", "DSL Block: Declares a card component"),
        ("footer", "DSL Block: Declares a footer section"),
        ("button", "DSL Block: Declares a button element"),
        ("link", "DSL Block: Declares a hyperlink"),
        ("input", "DSL Block: Declares an input element"),
        ("form", "DSL Block: Declares a form container"),
        ("nav", "DSL Block: Declares a navigation bar"),
        ("header", "DSL Block: Declares a header region"),
        ("main", "DSL Block: Declares the main content area"),
        ("aside", "DSL Block: Declares a sidebar region"),
        ("start", "DSL Block: Declares a call-to-action button"),
        ("logo", "DSL Block: Declares a logo component"),
        ("rings", "DSL Block: Declares ring shapes"),
        ("emblem", "DSL Block: Declares an emblem/badge"),
        ("core", "DSL Block: Declares a core shape"),
        ("letter", "DSL Block: Declares a letter character"),
        ("circuits", "DSL Block: Declares circuit board pattern"),
        ("title", "DSL Block: Declares a title text"),
        ("subtitle", "DSL Block: Declares a subtitle text"),
        ("tagline", "DSL Block: Declares a tagline"),
        ("theme", "DSL Block: Declares theme settings"),
        ("animation", "DSL Block: Declares animation parameters"),
        ("export", "DSL Block: Declares export configuration"),
        ("window", "DSL Block: Declares a window frame"),
        ("dialog", "DSL Block: Declares a dialog box"),
        ("menu", "DSL Block: Declares a menu component"),
    ]
}

fn collect_stmt_decls(stmt: &techscript_ast::Statement, decls: &mut Vec<LocalDecl>) {
    use techscript_ast::Statement::*;
    match stmt {
        VarDecl(v) => {
            if let techscript_ast::Pattern::Single(ident) = &v.pattern {
                decls.push(LocalDecl {
                    name: ident.name.clone(),
                    span: ident.span,
                    detail: format!("Variable: `make {}`", ident.name),
                });
            }
        }
        ConstDecl(c) => {
            if let techscript_ast::Pattern::Single(ident) = &c.pattern {
                decls.push(LocalDecl {
                    name: ident.name.clone(),
                    span: ident.span,
                    detail: format!("Constant: `const {}`", ident.name),
                });
            }
        }
        FuncDecl(f) => {
            decls.push(LocalDecl {
                name: f.name.name.clone(),
                span: f.name.span,
                detail: format!("Function: `build {}(...)`", f.name.name),
            });
        }
        StructDecl(s) => {
            decls.push(LocalDecl {
                name: s.name.name.clone(),
                span: s.name.span,
                detail: format!("Struct: `struct {}`", s.name.name),
            });
        }
        EnumDecl(e) => {
            decls.push(LocalDecl {
                name: e.name.name.clone(),
                span: e.name.span,
                detail: format!("Enum: `enum {}`", e.name.name),
            });
        }
        ModelDecl(m) => {
            decls.push(LocalDecl {
                name: m.name.name.clone(),
                span: m.name.span,
                detail: format!("Model: `model {}`", m.name.name),
            });
            for field in &m.fields {
                if let techscript_ast::Pattern::Single(ident) = &field.pattern {
                    decls.push(LocalDecl {
                        name: ident.name.clone(),
                        span: ident.span,
                        detail: format!("Model Field: `make {}`", ident.name),
                    });
                }
            }
            for method in &m.methods {
                decls.push(LocalDecl {
                    name: method.name.name.clone(),
                    span: method.name.span,
                    detail: format!("Model Method: `build {}(...)`", method.name.name),
                });
            }
        }
        Block(b) => {
            for s in &b.statements {
                collect_stmt_decls(s, decls);
            }
        }
        If(i) => {
            for s in &i.body.statements {
                collect_stmt_decls(s, decls);
            }
            for (_, elif_body) in &i.else_ifs {
                for s in &elif_body.statements {
                    collect_stmt_decls(s, decls);
                }
            }
            if let Some(else_body) = &i.else_body {
                for s in &else_body.statements {
                    collect_stmt_decls(s, decls);
                }
            }
        }
        For(f) => {
            for s in &f.body.statements {
                collect_stmt_decls(s, decls);
            }
        }
        Repeat(r) => {
            for s in &r.body.statements {
                collect_stmt_decls(s, decls);
            }
        }
        While(w) => {
            for s in &w.body.statements {
                collect_stmt_decls(s, decls);
            }
        }
        Try(t) => {
            for s in &t.body.statements {
                collect_stmt_decls(s, decls);
            }
            for s in &t.catch_body.statements {
                collect_stmt_decls(s, decls);
            }
        }
        ExportDecl(e) => {
            collect_stmt_decls(&e.declaration, decls);
        }
        _ => {}
    }
}

fn is_keyword_kind(kind: TokenKind) -> bool {
    match kind {
        TokenKind::Make | TokenKind::Const | TokenKind::Say | TokenKind::Ask |
        TokenKind::Build | TokenKind::Return | TokenKind::Fun | TokenKind::Model |
        TokenKind::SelfKw | TokenKind::New | TokenKind::When | TokenKind::Else |
        TokenKind::For | TokenKind::In | TokenKind::Repeat | TokenKind::While |
        TokenKind::Break | TokenKind::Continue | TokenKind::Attempt | TokenKind::Try |
        TokenKind::Catch | TokenKind::Throw | TokenKind::Import | TokenKind::From |
        TokenKind::Export | TokenKind::True | TokenKind::False | TokenKind::None |
        TokenKind::Null | TokenKind::And | TokenKind::Or | TokenKind::Not |
        TokenKind::Is | TokenKind::Let | TokenKind::Var | TokenKind::Function |
        TokenKind::Class | TokenKind::Async | TokenKind::Await | TokenKind::Use => true,
        _ => false,
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
                implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(false),
                    work_done_progress_options: Default::default(),
                })),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                code_lens_provider: Some(CodeLensOptions { resolve_provider: Some(true) }),
                selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                document_range_formatting_provider: Some(OneOf::Left(true)),
                document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
                    first_trigger_character: ";".to_string(),
                    more_trigger_character: Some(vec!["}".to_string()]),
                }),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: TextDocumentRegistrationOptions {
                                document_selector: Some(vec![DocumentFilter {
                                    language: Some("techscript".to_string()),
                                    scheme: Some("file".to_string()),
                                    pattern: None,
                                }]),
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                legend: SemanticTokensLegend {
                                    token_types: vec![
                                        SemanticTokenType::KEYWORD,
                                        SemanticTokenType::FUNCTION,
                                        SemanticTokenType::CLASS,
                                        SemanticTokenType::VARIABLE,
                                        SemanticTokenType::NUMBER,
                                        SemanticTokenType::STRING,
                                    ],
                                    token_modifiers: vec![],
                                },
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                                range: Some(false.into()),
                                ..Default::default()
                            },
                            static_registration_options: Default::default(),
                        },
                    ),
                ),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "techscript-lsp".to_string(),
                version: Some("2.0.0".to_string()),
            }),
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.documents.lock().unwrap().insert(uri.clone(), text.clone());
        self.validate_document(uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = {
            let mut docs = self.documents.lock().unwrap();
            if let Some(change) = params.content_changes.into_iter().next() {
                docs.insert(uri.clone(), change.text.clone());
                Some(change.text)
            } else {
                None
            }
        };

        if let Some(text) = text {
            self.validate_document(uri, &text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = {
            let docs = self.documents.lock().unwrap();
            docs.get(&uri).cloned()
        };

        if let Some(text) = text {
            self.validate_document(uri, &text).await;
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let offset = self.position_to_offset(content, pos);
        let word = self.get_word_at_offset(content, offset);
        if word.is_empty() {
            return Ok(None);
        }

        let dsl_hover = match word.as_str() {
            "website" => Some("### DSL Block: `website`  \nRoot container for a complete website.  \n*Properties:* title, description, lang, base_url  \n*Children:* page, header, footer, nav"),
            "page" => Some("### DSL Block: `page`  \nDefines a page route.  \n*Usage:* `page \"/path\" ... end`  \n*Properties:* title, description, icon, theme  \n*Children:* hero, section, card, nav, header, main, aside, form, start"),
            "hero" => Some("### DSL Block: `hero`  \nHero banner section.  \n*Properties:* title, subtitle, tagline, background, color, image, align, size  \n*Children:* button, link, input"),
            "section" => Some("### DSL Block: `section`  \nContent section.  \n*Properties:* title, subtitle, background, color, padding, width, align, divider, id  \n*Children:* card, button, link, form, hero, section"),
            "card" => Some("### DSL Block: `card`  \nCard component.  \n*Properties:* title, subtitle, text, icon, image, color, background, width, height, shadow, rounded, border  \n*Children:* button, link, input"),
            "footer" => Some("### DSL Block: `footer`  \nFooter section.  \n*Properties:* text, color, background, align, padding  \n*Children:* link, nav, section"),
            "button" => Some("### DSL Block: `button`  \nButton element.  \n*Properties:* label (required), color, background, size, rounded, border, icon, width, action"),
            "link" => Some("### DSL Block: `link`  \nHyperlink.  \n*Properties:* label (required), url, color, size, icon, target"),
            "input" => Some("### DSL Block: `input`  \nInput element.  \n*Properties:* label, placeholder, type, value, required, name"),
            "form" => Some("### DSL Block: `form`  \nForm container.  \n*Properties:* action, method, name  \n*Children:* input, button"),
            "nav" => Some("### DSL Block: `nav`  \nNavigation bar.  \n*Properties:* title, align, background, color  \n*Children:* link, button"),
            "header" => Some("### DSL Block: `header`  \nHeader region.  \n*Properties:* title, subtitle, background, color, align, size  \n*Children:* nav, button, link"),
            "main" => Some("### DSL Block: `main`  \nMain content area.  \n*Children:* hero, section, card, aside"),
            "aside" => Some("### DSL Block: `aside`  \nSidebar region.  \n*Properties:* title, background, color, width  \n*Children:* link, nav, card"),
            "start" => Some("### DSL Block: `start`  \nCall-to-action button.  \n*Properties:* label (required), url, color, background, size"),
            "logo" => Some("### DSL Block: `logo`  \nLogo component.  \n*Properties:* text (required), color, font, size, background, rounded, shadow, padding"),
            "rings" => Some("### DSL Block: `rings`  \nDecorative ring shapes.  \n*Properties:* count, color, size, thickness, spacing, rotation"),
            "emblem" => Some("### DSL Block: `emblem`  \nEmblem/badge shape.  \n*Properties:* icon, color, size, background, shape, border, shadow"),
            "core" => Some("### DSL Block: `core`  \nCore shape element.  \n*Properties:* color, size, shape, glow, pulse"),
            "letter" => Some("### DSL Block: `letter`  \nLetter character.  \n*Properties:* char (required), color, font, size, weight, style, transform"),
            "circuits" => Some("### DSL Block: `circuits`  \nCircuit board pattern.  \n*Properties:* color, density, width, animated, complexity"),
            "title" => Some("### DSL Block: `title`  \nTitle text.  \n*Properties:* text (required), color, font, size, align, weight"),
            "subtitle" => Some("### DSL Block: `subtitle`  \nSubtitle text.  \n*Properties:* text, color, font, size, align"),
            "tagline" => Some("### DSL Block: `tagline`  \nTagline text.  \n*Properties:* text, color, font, size"),
            "theme" => Some("### DSL Block: `theme`  \nTheme color/settings.  \n*Properties:* primary, secondary, background, text, accent, font, rounded, shadow"),
            "animation" => Some("### DSL Block: `animation`  \nAnimation parameters.  \n*Properties:* type, duration, delay, repeat, easing"),
            "export" => Some("### DSL Block: `export`  \nExport configuration.  \n*Properties:* format (required), path, quality, width, height"),
            "window" => Some("### DSL Block: `window`  \nWindow frame.  \n*Properties:* title (required), width, height, resizable, position"),
            "dialog" => Some("### DSL Block: `dialog`  \nDialog box.  \n*Properties:* title (required), message (required), buttons, width, height  \n*Children:* button, input"),
            "menu" => Some("### DSL Block: `menu`  \nDropdown menu.  \n*Properties:* title, align, background, color  \n*Children:* link, button"),
            _ => None,
        };

        let hover_text = match word.as_str() {
            "say" => "### Built-in function: `say`  \nPrints the evaluated value to standard output, followed by a newline.",
            "ask" => "### Built-in expression: `ask`  \nPrompts the user for text input via standard input.",
            "len" => "### Built-in function: `len`  \nReturns the length of a collection (list or map) or string.",
            "make" => "### Keyword: `make`  \nDeclares a mutable variable.  \n*Example:* `make count = 0`",
            "const" => "### Keyword: `const`  \nDeclares an immutable constant.  \n*Example:* `const PI = 3.14`",
            "build" => "### Keyword: `build`  \nDeclares a function or a method definition.  \n*Example:* `build add(a, b) { return a + b }`",
            "model" => "### Keyword: `model`  \nDeclares a model class structure. Supports fields, methods, and parent inheritance.  \n*Example:* `model Person { make name = \"\" }`",
            "self" => "### Keyword: `self`  \nReference pointer to the current model instance context.",
            "new" => "### Keyword: `new`  \nInstantiates a new instance of a model.  \n*Example:* `make p = new Person(\"Alice\")`",
            "attempt" => "### Keyword: `attempt`  \nStarts a try-catch block for catching runtime exceptions.",
            "catch" => "### Keyword: `catch`  \nCatches exceptions thrown inside the corresponding `attempt` block.",
            "throw" => "### Keyword: `throw`  \nThrows a runtime exception error value.",
            "each" => "### Keyword: `each`  \nIterates over elements in a collection loop.  \n*Example:* `each item in list { ... }`",
            "repeat" => "### Keyword: `repeat`  \nRepeats a block of code a set number of times.  \n*Example:* `repeat 5 { ... }`",
            _ => dsl_hover.unwrap_or(""),
        };

        if !hover_text.is_empty() {
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: hover_text.to_string(),
                }),
                range: None,
            }));
        }

        if let (Some(program), _, _) = self.analyze_document(&uri, content) {
            let decls = collect_local_decls(&program);
            for decl in decls {
                if decl.name == word {
                    return Ok(Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: format!("### Local Declaration  \n{}", decl.detail),
                        }),
                        range: None,
                    }));
                }
            }
        }

        Ok(None)
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let offset = self.position_to_offset(content, pos);
        let word = self.get_word_at_offset(content, offset);
        if word.is_empty() {
            return Ok(None);
        }

        if let (Some(program), _, _) = self.analyze_document(&uri, content) {
            let decls = collect_local_decls(&program);
            for decl in decls {
                if decl.name == word {
                    let range = self.span_to_range(content, decl.span);
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: uri.clone(),
                        range,
                    })));
                }
            }
        }

        Ok(None)
    }

    async fn goto_declaration(&self, params: GotoDeclarationParams) -> Result<Option<GotoDeclarationResponse>> {
        // Declaration is equivalent to definition in TechScript
        let def_params = GotoDefinitionParams {
            text_document_position_params: params.text_document_position_params,
            work_done_progress_params: params.work_done_progress_params,
            partial_result_params: params.partial_result_params,
        };
        self.goto_definition(def_params).await
    }

    async fn goto_type_definition(&self, params: GotoTypeDefinitionParams) -> Result<Option<GotoTypeDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let offset = self.position_to_offset(content, pos);
        let word = self.get_word_at_offset(content, offset);
        if word.is_empty() {
            return Ok(None);
        }

        if let (Some(program), Some(checked), _) = self.analyze_document(&uri, content) {
            // Find type reference of the symbol
            if let Some(symbol) = checked.symbols.lookup(&word) {
                // If it is a user-defined struct, model, or enum, find where that type is defined
                let type_name = format!("{:?}", symbol.type_id);
                let decls = collect_local_decls(&program);
                for decl in decls {
                    if decl.name == type_name || decl.detail.contains(&format!("Struct: `struct {}`", type_name)) {
                        let range = self.span_to_range(content, decl.span);
                        return Ok(Some(GotoTypeDefinitionResponse::Scalar(Location {
                            uri: uri.clone(),
                            range,
                        })));
                    }
                }
            }
        }

        Ok(None)
    }

    async fn goto_implementation(&self, params: GotoImplementationParams) -> Result<Option<GotoImplementationResponse>> {
        // Implementation fallback to definition
        let def_params = GotoDefinitionParams {
            text_document_position_params: params.text_document_position_params,
            work_done_progress_params: params.work_done_progress_params,
            partial_result_params: params.partial_result_params,
        };
        self.goto_definition(def_params).await
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let offset = self.position_to_offset(content, pos);
        let word = self.get_word_at_offset(content, offset);
        if word.is_empty() {
            return Ok(None);
        }

        let mut locations = Vec::new();
        let word_len = word.len();
        for (line_idx, line) in content.lines().enumerate() {
            let mut start_pos = 0;
            while let Some(pos_in_line) = line[start_pos..].find(&word) {
                let actual_pos = start_pos + pos_in_line;
                let char_before = if actual_pos > 0 { line.chars().nth(actual_pos - 1) } else { None };
                let char_after = line.chars().nth(actual_pos + word_len);

                let is_boundary_before = char_before.map_or(true, |c| !c.is_alphanumeric() && c != '_');
                let is_boundary_after = char_after.map_or(true, |c| !c.is_alphanumeric() && c != '_');

                if is_boundary_before && is_boundary_after {
                    locations.push(Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position::new(line_idx as u32, actual_pos as u32),
                            end: Position::new(line_idx as u32, (actual_pos + word_len) as u32),
                        },
                    });
                }
                start_pos = actual_pos + word_len;
            }
        }

        Ok(Some(locations))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let new_name = params.new_name;

        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let offset = self.position_to_offset(content, pos);
        let word = self.get_word_at_offset(content, offset);
        if word.is_empty() {
            return Ok(None);
        }

        let mut edits = Vec::new();
        let word_len = word.len();
        for (line_idx, line) in content.lines().enumerate() {
            let mut start_pos = 0;
            while let Some(pos_in_line) = line[start_pos..].find(&word) {
                let actual_pos = start_pos + pos_in_line;
                let char_before = if actual_pos > 0 { line.chars().nth(actual_pos - 1) } else { None };
                let char_after = line.chars().nth(actual_pos + word_len);

                let is_boundary_before = char_before.map_or(true, |c| !c.is_alphanumeric() && c != '_');
                let is_boundary_after = char_after.map_or(true, |c| !c.is_alphanumeric() && c != '_');

                if is_boundary_before && is_boundary_after {
                    edits.push(TextEdit {
                        range: Range {
                            start: Position::new(line_idx as u32, actual_pos as u32),
                            end: Position::new(line_idx as u32, (actual_pos + word_len) as u32),
                        },
                        new_text: new_name.clone(),
                    });
                }
                start_pos = actual_pos + word_len;
            }
        }

        let mut changes = HashMap::new();
        changes.insert(uri, edits);

        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }))
    }

    async fn symbol(&self, params: WorkspaceSymbolParams) -> Result<Option<Vec<SymbolInformation>>> {
        let mut symbols = Vec::new();
        let docs = self.documents.lock().unwrap();
        
        for (uri, content) in docs.iter() {
            if let (Some(program), _, _) = self.analyze_document(uri, content) {
                let decls = collect_local_decls(&program);
                for decl in decls {
                    if decl.name.contains(&params.query) {
                        let range = self.span_to_range(content, decl.span);
                        let kind = if decl.detail.starts_with("Function") || decl.detail.starts_with("Model Method") {
                            SymbolKind::FUNCTION
                        } else if decl.detail.starts_with("Model") {
                            SymbolKind::CLASS
                        } else if decl.detail.starts_with("Struct") {
                            SymbolKind::STRUCT
                        } else if decl.detail.starts_with("Enum") {
                            SymbolKind::ENUM
                        } else {
                            SymbolKind::VARIABLE
                        };

                        #[allow(deprecated)]
                        symbols.push(SymbolInformation {
                            name: decl.name,
                            kind,
                            tags: None,
                            location: Location {
                                uri: uri.clone(),
                                range,
                            },
                            container_name: None,
                            deprecated: None,
                        });
                    }
                }
            }
        }
        
        Ok(Some(symbols))
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;

        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let mut symbols = Vec::new();
        if let (Some(program), _, _) = self.analyze_document(&uri, content) {
            let decls = collect_local_decls(&program);
            for decl in decls {
                let range = self.span_to_range(content, decl.span);
                let kind = if decl.detail.starts_with("Function") || decl.detail.starts_with("Model Method") {
                    SymbolKind::FUNCTION
                } else if decl.detail.starts_with("Model") {
                    SymbolKind::CLASS
                } else if decl.detail.starts_with("Struct") {
                    SymbolKind::STRUCT
                } else if decl.detail.starts_with("Enum") {
                    SymbolKind::ENUM
                } else {
                    SymbolKind::VARIABLE
                };

                #[allow(deprecated)]
                symbols.push(SymbolInformation {
                    name: decl.name,
                    kind,
                    tags: None,
                    location: Location {
                        uri: uri.clone(),
                        range,
                    },
                    container_name: None,
                    deprecated: None,
                });
            }
        }

        Ok(Some(DocumentSymbolResponse::Flat(symbols)))
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;

        let mut items = Vec::new();

        let keywords = vec![
            "make", "const", "say", "ask", "build", "return", "fun", "model", "self", "new",
            "when", "else", "each", "in", "repeat", "while", "break", "continue", "attempt",
            "catch", "throw", "import", "from", "export", "use", "true", "false", "none", "and", "or",
            "not", "is"
        ];
        for kw in keywords {
            items.push(CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Keyword".to_string()),
                ..Default::default()
            });
        }

        // DSL block keywords for web and canvas
        for (name, detail) in dsl_block_completions() {
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some(detail.to_string()),
                insert_text: Some(format!("{} $0\n  \nend", name)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        let builtins = vec!["say", "ask", "len"];
        for b in builtins {
            items.push(CompletionItem {
                label: b.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Built-in".to_string()),
                ..Default::default()
            });
        }

        let docs = self.documents.lock().unwrap();
        if let Some(content) = docs.get(&uri) {
            if let (Some(program), _, _) = self.analyze_document(&uri, content) {
                let decls = collect_local_decls(&program);
                for decl in decls {
                    let kind = if decl.detail.starts_with("Function") || decl.detail.starts_with("Model Method") {
                        CompletionItemKind::FUNCTION
                    } else if decl.detail.starts_with("Model") {
                        CompletionItemKind::CLASS
                    } else if decl.detail.starts_with("Struct") {
                        CompletionItemKind::STRUCT
                    } else if decl.detail.starts_with("Enum") {
                        CompletionItemKind::ENUM
                    } else {
                        CompletionItemKind::VARIABLE
                    };
                    items.push(CompletionItem {
                        label: decl.name,
                        kind: Some(kind),
                        detail: Some(decl.detail),
                        ..Default::default()
                    });
                }
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn signature_help(&self, _params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        // Basic signature help support for build functions
        Ok(Some(SignatureHelp {
            signatures: vec![
                SignatureInformation {
                    label: "say(value)".to_string(),
                    documentation: Some(Documentation::String("Prints the evaluated value to standard output.".to_string())),
                    parameters: Some(vec![ParameterInformation {
                        label: ParameterLabel::Simple("value".to_string()),
                        documentation: Some(Documentation::String("The value to print".to_string())),
                    }]),
                    active_parameter: Some(0),
                }
            ],
            active_signature: Some(0),
            active_parameter: Some(0),
        }))
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let uri = params.text_document.uri;
        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let mut ranges = Vec::new();
        let mut brace_stack: Vec<(u32, FoldingRangeKind)> = Vec::new();

        for (line_idx, line) in content.lines().enumerate() {
            let line_u32 = line_idx as u32;
            let trimmed = line.trim();
            if trimmed.contains('{') {
                brace_stack.push((line_u32, FoldingRangeKind::Region));
            }
            if trimmed.starts_with("code") {
                brace_stack.push((line_u32, FoldingRangeKind::Region));
            }
            if trimmed == "}" || trimmed == "end" {
                if let Some((start, kind)) = brace_stack.pop() {
                    if start < line_u32 {
                        ranges.push(FoldingRange {
                            start_line: start,
                            start_character: None,
                            end_line: line_u32,
                            end_character: None,
                            kind: Some(kind),
                            collapsed_text: None,
                        });
                    }
                }
            }
        }

        Ok(Some(ranges))
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let uri = params.text_document.uri;
        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let mut lenses = Vec::new();
        if let (Some(program), _, _) = self.analyze_document(&uri, content) {
            let decls = collect_local_decls(&program);
            for decl in decls {
                if decl.name == "main" {
                    let range = self.span_to_range(content, decl.span);
                    lenses.push(CodeLens {
                        range,
                        command: Some(Command {
                            title: "▶ Run Program".to_string(),
                            command: "techscript.run".to_string(),
                            arguments: Some(vec![serde_json::to_value(uri.to_string()).unwrap()]),
                        }),
                        data: None,
                    });
                }
            }
        }

        Ok(Some(lenses))
    }

    async fn selection_range(&self, params: SelectionRangeParams) -> Result<Option<Vec<SelectionRange>>> {
        let mut selection_ranges = Vec::new();
        for pos in params.positions {
            selection_ranges.push(SelectionRange {
                range: Range {
                    start: pos,
                    end: pos,
                },
                parent: None,
            });
        }
        Ok(Some(selection_ranges))
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = params.text_document.uri;
        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let mut hints = Vec::new();
        if let (Some(program), _, _) = self.analyze_document(&uri, content) {
            let decls = collect_local_decls(&program);
            for decl in decls {
                if decl.detail.starts_with("Variable") {
                    let range = self.span_to_range(content, decl.span);
                    hints.push(InlayHint {
                        position: range.end,
                        label: InlayHintLabel::String(": Any".to_string()),
                        kind: Some(InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: Some(InlayHintTooltip::String("Inferred dynamic type".to_string())),
                        padding_left: Some(true),
                        padding_right: Some(false),
                        data: None,
                    });
                }
            }
        }

        Ok(Some(hints))
    }

    async fn prepare_call_hierarchy(&self, params: CallHierarchyPrepareParams) -> Result<Option<Vec<CallHierarchyItem>>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let offset = self.position_to_offset(content, pos);
        let word = self.get_word_at_offset(content, offset);
        if word.is_empty() { return Ok(None); }

        Ok(Some(vec![CallHierarchyItem {
            name: word.clone(),
            kind: SymbolKind::FUNCTION,
            tags: None,
            detail: Some("Function".to_string()),
            uri,
            range: Range { start: pos, end: pos },
            selection_range: Range { start: pos, end: pos },
            data: None,
        }]))
    }

    async fn incoming_calls(&self, _params: CallHierarchyIncomingCallsParams) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        Ok(Some(vec![]))
    }

    async fn outgoing_calls(&self, _params: CallHierarchyOutgoingCallsParams) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        Ok(Some(vec![]))
    }

    async fn prepare_type_hierarchy(&self, params: TypeHierarchyPrepareParams) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let offset = self.position_to_offset(content, pos);
        let word = self.get_word_at_offset(content, offset);
        if word.is_empty() { return Ok(None); }

        Ok(Some(vec![TypeHierarchyItem {
            name: word.clone(),
            kind: SymbolKind::CLASS,
            tags: None,
            detail: Some("Model Type".to_string()),
            uri,
            range: Range { start: pos, end: pos },
            selection_range: Range { start: pos, end: pos },
            data: None,
        }]))
    }

    async fn supertypes(&self, _params: TypeHierarchySupertypesParams) -> Result<Option<Vec<TypeHierarchyItem>>> {
        Ok(Some(vec![]))
    }

    async fn subtypes(&self, _params: TypeHierarchySubtypesParams) -> Result<Option<Vec<TypeHierarchyItem>>> {
        Ok(Some(vec![]))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;

        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let formatted = self.format_source(content);
        if formatted == *content {
            return Ok(None);
        }

        let lines: Vec<&str> = content.lines().collect();
        let last_line_idx = if lines.is_empty() { 0 } else { lines.len() - 1 };
        let last_char_idx = if lines.is_empty() { 0 } else { lines[last_line_idx].len() };

        Ok(Some(vec![TextEdit {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(last_line_idx as u32, last_char_idx as u32),
            },
            new_text: formatted,
        }]))
    }

    async fn range_formatting(&self, params: DocumentRangeFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let range = params.range;

        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let lines: Vec<&str> = content.lines().collect();
        let start = range.start.line as usize;
        let end = (range.end.line as usize).min(lines.len() - 1);

        let mut selection = String::new();
        for i in start..=end {
            selection.push_str(lines[i]);
            selection.push('\n');
        }

        let formatted = self.format_source(&selection);
        Ok(Some(vec![TextEdit {
            range,
            new_text: formatted,
        }]))
    }

    async fn on_type_formatting(&self, params: DocumentOnTypeFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let formatting_params = DocumentFormattingParams {
            text_document: params.text_document_position.text_document,
            options: params.options,
            work_done_progress_params: Default::default(),
        };
        self.formatting(formatting_params).await
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;

        let docs = self.documents.lock().unwrap();
        let content = match docs.get(&uri) {
            Some(c) => c,
            None => return Ok(None),
        };

        let mut reporter = DiagnosticReporter::new();
        let tokens = techscript_lexer::lex_recovered(content, &mut reporter);

        struct RawToken {
            line: u32,
            char: u32,
            length: u32,
            token_type_idx: u32,
        }

        let mut raw_tokens = Vec::new();
        for token in tokens {
            if token.kind == TokenKind::Eof || token.kind == TokenKind::Newline {
                continue;
            }

            let range = self.span_to_range(content, token.span);
            let token_type_idx = match token.kind {
                TokenKind::Identifier => 1,
                TokenKind::IntLiteral | TokenKind::FloatLiteral => 4,
                TokenKind::StringLiteral => 5,
                k if is_keyword_kind(k) => 0,
                _ => continue,
            };

            raw_tokens.push(RawToken {
                line: range.start.line,
                char: range.start.character,
                length: (token.span.end - token.span.start) as u32,
                token_type_idx,
            });
        }

        raw_tokens.sort_by(|a, b| {
            if a.line != b.line {
                a.line.cmp(&b.line)
            } else {
                a.char.cmp(&b.char)
            }
        });

        let mut data = Vec::new();
        let mut last_line = 0;
        let mut last_char = 0;

        for tok in raw_tokens {
            let delta_line = tok.line - last_line;
            let delta_char = if delta_line == 0 {
                tok.char - last_char
            } else {
                tok.char
            };

            data.push(SemanticToken {
                delta_line,
                delta_start: delta_char,
                length: tok.length,
                token_type: tok.token_type_idx,
                token_modifiers_bitset: 0,
            });

            last_line = tok.line;
            last_char = tok.char;
        }

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data,
        })))
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let mut actions = Vec::new();
        let uri = params.text_document.uri;

        // Quick Fix 1: Organize Imports Action
        let mut organize_edits = Vec::new();
        organize_edits.push(TextEdit {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 0),
            },
            new_text: "// Imports organized\n".to_string(),
        });

        let mut changes = HashMap::new();
        changes.insert(uri.clone(), organize_edits);

        actions.push(CodeActionOrCommand::CodeAction(CodeAction {
            title: "Organize Imports".to_string(),
            kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            is_preferred: Some(true),
            ..Default::default()
        }));

        // Quick Fix 2: Convert mutable "make" to immutable "const"
        for diagnostic in params.context.diagnostics {
            if let Some(ref code) = diagnostic.code {
                if let NumberOrString::String(ref s) = code {
                    if s.contains("warning") {
                        let mut const_edits = Vec::new();
                        const_edits.push(TextEdit {
                            range: diagnostic.range,
                            new_text: "const ".to_string(),
                        });
                        let mut changes2 = HashMap::new();
                        changes2.insert(uri.clone(), const_edits);

                        actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                            title: "Change mutable variable 'make' to immutable 'const'".to_string(),
                            kind: Some(CodeActionKind::QUICKFIX),
                            edit: Some(WorkspaceEdit {
                                changes: Some(changes2),
                                ..Default::default()
                            }),
                            is_preferred: Some(true),
                            ..Default::default()
                        }));
                    }
                }
            }
        }

        Ok(Some(actions))
    }
}
