const vscode = require('vscode');
const cp = require('child_process');

function activate(context) {
    // 1. TechScript: Run Current File
    let runCommand = vscode.commands.registerCommand('techscript.run', function () {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            vscode.window.showErrorMessage('No active text editor found. Please open a .txs file first!');
            return;
        }
        
        const filePath = activeEditor.document.fileName;
        if (!filePath.endsWith('.txs') && !filePath.endsWith('.tx')) {
            vscode.window.showWarningMessage('The current file is not a TechScript (.txs) file.');
        }

        // Save active editor before running
        activeEditor.document.save().then(() => {
            let terminal = vscode.window.terminals.find(t => t.name === "TechScript Terminal");
            if (!terminal) {
                terminal = vscode.window.createTerminal("TechScript Terminal");
            }
            terminal.show();
            // Send the tech run command with path enclosed in double quotes for safety
            terminal.sendText(`tech run "${filePath}"`);
        });
    });

    // 2. TechScript: Open Interactive REPL
    let replCommand = vscode.commands.registerCommand('techscript.repl', function () {
        let terminal = vscode.window.terminals.find(t => t.name === "TechScript REPL");
        if (!terminal) {
            terminal = vscode.window.createTerminal("TechScript REPL");
        }
        terminal.show();
        terminal.sendText("tech repl");
    });

    // 3. TechScript: Launch Studio IDE
    let studioCommand = vscode.commands.registerCommand('techscript.studio', function () {
        cp.exec('tech studio', (error) => {
            if (error) {
                vscode.window.showErrorMessage('Failed to launch TechScript Studio. Make sure tech is in your PATH.');
            }
        });
        vscode.window.showInformationMessage('Launching TechScript Studio Visual IDE...');
    });

    context.subscriptions.push(runCommand);
    context.subscriptions.push(replCommand);
    context.subscriptions.push(studioCommand);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
