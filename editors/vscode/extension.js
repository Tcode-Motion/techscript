const vscode = require('vscode');
const { LanguageClient } = require('vscode-languageclient/node');

let client;

function activate(context) {
    let config = vscode.workspace.getConfiguration('techscript');
    let lspPath = config.get('lsp.path') || 'techscript-lsp';

    let serverOptions = {
        run: { command: lspPath },
        debug: { command: lspPath }
    };

    let clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'techscript' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.txs')
        }
    };

    client = new LanguageClient(
        'techscriptLanguageServer',
        'TechScript Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

module.exports = {
    activate,
    deactivate
};
