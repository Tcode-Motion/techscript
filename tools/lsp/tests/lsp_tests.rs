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
