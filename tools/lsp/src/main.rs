//! # TechScript Language Server Binary (techscript-lsp)
//!
//! Entry point orchestrating stdin/stdout tower-lsp stream serving.

use techscript_lsp::Backend;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
