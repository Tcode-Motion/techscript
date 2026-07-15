use tower_lsp::LspService;
use techscript_lsp::Backend;

#[test]
fn test_lsp_backend_capabilities() {
    let (service, _) = LspService::new(Backend::new);
    let _ = service; // compilation check
}
