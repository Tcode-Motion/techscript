use techscript_lsp::Backend;
use tower_lsp::LspService;

#[test]
fn test_lsp_backend_capabilities() {
    let (service, _) = LspService::new(Backend::new);
    let _ = service; // compilation check
}
