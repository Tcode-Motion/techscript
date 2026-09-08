use techscript_lsp::Backend;
use tower_lsp::lsp_types::*;
use tower_lsp::LanguageServer;
use tower_lsp::LspService;

#[test]
fn test_lsp_backend_capabilities() {
    let (service, _) = LspService::new(Backend::new);
    let _ = service;
}

#[tokio::test]
async fn test_lsp_completion() {
    let (service, _) = LspService::new(Backend::new);

    let doc_uri = Url::parse("file:///main.txs").unwrap();
    service
        .inner()
        .did_open(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: doc_uri.clone(),
                language_id: "techscript".to_string(),
                version: 1,
                text: "build main() {\n    make x = 10\n}".to_string(),
            },
        })
        .await;

    let res = service
        .inner()
        .completion(CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: doc_uri },
                position: Position::new(1, 4),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        })
        .await
        .unwrap();

    let items = match res.unwrap() {
        CompletionResponse::Array(arr) => arr,
        _ => panic!("Expected Array response"),
    };

    assert!(items.iter().any(|item| item.label == "make"));
    assert!(items.iter().any(|item| item.label == "say"));
}

#[tokio::test]
async fn test_lsp_hover() {
    let (service, _) = LspService::new(Backend::new);

    let doc_uri = Url::parse("file:///main.txs").unwrap();
    service
        .inner()
        .did_open(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: doc_uri.clone(),
                language_id: "techscript".to_string(),
                version: 1,
                text: "build main() {\n    say 42\n}".to_string(),
            },
        })
        .await;

    let res = service
        .inner()
        .hover(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: doc_uri },
                position: Position::new(1, 4),
            },
            work_done_progress_params: Default::default(),
        })
        .await
        .unwrap()
        .unwrap();

    let contents = match res.contents {
        HoverContents::Markup(m) => m.value,
        _ => panic!("Expected Markup hover"),
    };

    assert!(contents.contains("say"));
}

#[tokio::test]
async fn test_lsp_formatting() {
    let (service, _) = LspService::new(Backend::new);

    let doc_uri = Url::parse("file:///main.txs").unwrap();
    service
        .inner()
        .did_open(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: doc_uri.clone(),
                language_id: "techscript".to_string(),
                version: 1,
                text: "build main() {\nsay 42\n}".to_string(),
            },
        })
        .await;

    let res = service
        .inner()
        .formatting(DocumentFormattingParams {
            text_document: TextDocumentIdentifier { uri: doc_uri },
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                ..Default::default()
            },
            work_done_progress_params: Default::default(),
        })
        .await
        .unwrap()
        .unwrap();

    assert_eq!(res.len(), 1);
    let new_text = &res[0].new_text;
    assert!(new_text.contains("    say 42"));
}

#[tokio::test]
async fn test_lsp_code_action_make_to_const() {
    let (service, _) = LspService::new(Backend::new);

    let doc_uri = Url::parse("file:///main.txs").unwrap();
    service
        .inner()
        .did_open(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: doc_uri.clone(),
                language_id: "techscript".to_string(),
                version: 1,
                text: "build main() {\n    make x = 10\n}".to_string(),
            },
        })
        .await;

    let res = service
        .inner()
        .code_action(CodeActionParams {
            text_document: TextDocumentIdentifier {
                uri: doc_uri.clone(),
            },
            range: Range::default(),
            context: CodeActionContext {
                diagnostics: vec![Diagnostic {
                    range: Range {
                        start: Position::new(1, 4),
                        end: Position::new(1, 8),
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("warning".to_string())),
                    code_description: None,
                    source: Some("techscript".to_string()),
                    message: "Variable is never mutated. Consider using `const`".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                }],
                only: None,
                trigger_kind: None,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
        .await
        .unwrap();

    println!("{:#?}", res);

    let actions = res.unwrap();

    // Find the quick fix action
    let mut found = false;
    for action in actions {
        if let CodeActionOrCommand::CodeAction(a) = action {
            if a.title.contains("immutable 'const'") {
                found = true;

                let edits = a.edit.unwrap().changes.unwrap();
                let edits_for_uri = edits.get(&doc_uri).unwrap();

                assert_eq!(edits_for_uri.len(), 1);
                assert_eq!(edits_for_uri[0].new_text, "const ");
                assert_eq!(edits_for_uri[0].range.start.line, 1);
                assert_eq!(edits_for_uri[0].range.start.character, 4);
                assert_eq!(edits_for_uri[0].range.end.character, 8);
            }
        }
    }

    assert!(found, "Did not find the 'Change to const' quick fix");
}
