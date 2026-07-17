# Language Server Protocol (LSP) Guide — TechScript 2.0

The TechScript Language Server (`techscript-lsp`) facilitates semantic analysis and code editing features for IDEs using standard JSON-RPC communication.

## Supported Capabilities

The server advertises and supports the following LSP capabilities:

- `textDocument/didOpen` / `textDocument/didChange` / `textDocument/didSave` (full synchronization)
- `textDocument/completion` (Intelligent Auto Completion for variables, functions, structs, models, and keywords)
- `textDocument/hover` (Built-in standard library function help and local declaration signatures)
- `textDocument/definition` (Go To Definition)
- `textDocument/declaration` (Go To Declaration)
- `textDocument/typeDefinition` (Go To Type Definition)
- `textDocument/implementation` (Go To Implementation)
- `textDocument/references` (Find All References)
- `textDocument/rename` (Rename Symbol)
- `textDocument/documentSymbol` (Document outline view symbols)
- `workspace/symbol` (Workspace search symbols)
- `textDocument/foldingRange` (Code blocks folding)
- `textDocument/codeLens` (Inline Run/Build triggers)
- `textDocument/selectionRange` (Expand/shrink selection)
- `textDocument/inlayHint` (Inlay Type annotations)
- `textDocument/callHierarchy` / `textDocument/typeHierarchy`
- `textDocument/formatting` / `textDocument/rangeFormatting` / `textDocument/onTypeFormatting`
- `textDocument/codeAction` (Quick Fixes and organize imports)
- `textDocument/semanticTokens` (Semantic coloring)

## Error Recovery & Resilience

The language server employs syntax-error-resilient lexing and parsing. Even in the presence of lexical errors or parse exceptions, the compiler recovers the statement block bounds and successfully builds a partial AST, ensuring intellisense features remain active.
