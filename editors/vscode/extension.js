const vscode = require('vscode');
const { LanguageClient } = require('vscode-languageclient/node');
const path = require('path');
const fs = require('fs');

let client;
let outputChannel;

function activate(context) {
    outputChannel = vscode.window.createOutputChannel('TechScript');

    // Language Server Client Setup
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

    // 2. Register Tree Views
    const workspaceRoot = vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0
        ? vscode.workspace.workspaceFolders[0].uri.fsPath
        : undefined;

    const explorerProvider = new TechScriptExplorerProvider(workspaceRoot);
    vscode.window.registerTreeDataProvider('techscript-explorer', explorerProvider);

    const pmProvider = new TechScriptPmProvider();
    vscode.window.registerTreeDataProvider('techscript-pm', pmProvider);

    const examplesProvider = new TechScriptExamplesProvider(workspaceRoot);
    vscode.window.registerTreeDataProvider('techscript-examples', examplesProvider);

    const templatesProvider = new TechScriptTemplatesProvider();
    vscode.window.registerTreeDataProvider('techscript-templates', templatesProvider);

    // Register Webview View Provider for offline documentation
    const docsProvider = new TechScriptDocsViewProvider(context.extensionUri);
    vscode.window.registerWebviewViewProvider('techscript-docs', docsProvider);

    // 3. Command Registries
    context.subscriptions.push(
        vscode.commands.registerCommand('techscript.run', () => runCommand('run')),
        vscode.commands.registerCommand('techscript.build', () => runCommand('build')),
        vscode.commands.registerCommand('techscript.check', () => runCommand('check')),
        vscode.commands.registerCommand('techscript.test', () => runCommand('test')),
        vscode.commands.registerCommand('techscript.format', () => runCommand('fmt')),
        vscode.commands.registerCommand('techscript.lint', () => runCommand('lint')),
        vscode.commands.registerCommand('techscript.clean', () => runCommand('clean')),
        vscode.commands.registerCommand('techscript.generateDocs', () => runCommand('doc')),
        vscode.commands.registerCommand('techscript.openRepl', () => {
            const terminal = vscode.window.activeTerminal || vscode.window.createTerminal('TechScript');
            terminal.show();
            terminal.sendText('tsc repl');
        }),
        vscode.commands.registerCommand('techscript.packageProject', () => {
            const terminal = vscode.window.activeTerminal || vscode.window.createTerminal('TechScript');
            terminal.show();
            terminal.sendText('tsc build --target native');
        }),
        vscode.commands.registerCommand('techscript.showVersion', () => {
            const cp = require('child_process');
            cp.exec('tsc version', (err, stdout, stderr) => {
                if (err) {
                    vscode.window.showErrorMessage('Failed to resolve compiler version.');
                } else {
                    vscode.window.showInformationMessage(`TechScript Compiler: ${stdout.trim()}`);
                }
            });
        }),
        vscode.commands.registerCommand('techscript.showEnv', () => {
            let info = `TECHSCRIPT_HOME: ${process.env.TECHSCRIPT_HOME || 'Not Configured'}\n` +
                       `TECHSCRIPT_STDLIB: ${process.env.TECHSCRIPT_STDLIB || 'Not Configured'}\n` +
                       `TECHSCRIPT_DOCS: ${process.env.TECHSCRIPT_DOCS || 'Not Configured'}\n` +
                       `TECHSCRIPT_TEMPLATES: ${process.env.TECHSCRIPT_TEMPLATES || 'Not Configured'}\n` +
                       `TECHSCRIPT_CACHE: ${process.env.TECHSCRIPT_CACHE || 'Not Configured'}\n` +
                       `TECHSCRIPT_PACKAGES: ${process.env.TECHSCRIPT_PACKAGES || 'Not Configured'}`;
            vscode.window.showInformationMessage('TechScript 2.0 Environment Settings', { modal: true, detail: info });
        }),
        vscode.commands.registerCommand('techscript.clearCache', () => {
            const terminal = vscode.window.activeTerminal || vscode.window.createTerminal('TechScript');
            terminal.show();
            terminal.sendText('tsc clean');
            vscode.window.showInformationMessage('TechScript compiler and package caches cleared.');
        }),
        vscode.commands.registerCommand('techscript.openExamples', () => {
            let homePath = process.env.TECHSCRIPT_HOME || 'C:\\Program Files\\TechScript';
            let examplesFolder = path.join(homePath, 'examples');
            if (fs.existsSync(examplesFolder)) {
                vscode.commands.executeCommand('vscode.openFolder', vscode.Uri.file(examplesFolder), true);
            } else {
                vscode.window.showErrorMessage('Local examples folder not found under TECHSCRIPT_HOME.');
            }
        }),
        vscode.commands.registerCommand('techscript.openDocumentation', () => {
            let homePath = process.env.TECHSCRIPT_HOME || 'C:\\Program Files\\TechScript';
            let indexHtml = path.join(homePath, 'docs', 'html', 'index.html');
            if (fs.existsSync(indexHtml)) {
                vscode.env.openExternal(vscode.Uri.file(indexHtml));
            } else {
                vscode.env.openExternal(vscode.Uri.parse('https://github.com/Tcode-Motion/TechScript-2.0'));
            }
        }),
        vscode.commands.registerCommand('techscript.restartLsp', () => {
            vscode.window.showInformationMessage('Restarting TechScript Language Server...');
            if (client) {
                client.stop().then(() => client.start());
            }
        }),
        vscode.commands.registerCommand('techscript.showAst', () => dumpCommand('dump-ast', 'AST')),
        vscode.commands.registerCommand('techscript.showIr', () => dumpCommand('dump-ir', 'IR')),
        vscode.commands.registerCommand('techscript.showBytecode', () => dumpCommand('dump-bytecode', 'Bytecode')),

        // Example/Template action mappings
        vscode.commands.registerCommand('techscript.openExampleFile', (exampleName) => {
            let homePath = process.env.TECHSCRIPT_HOME || 'C:\\Program Files\\TechScript';
            let examplePath = path.join(homePath, 'examples', exampleName, 'main.txs');
            if (!fs.existsSync(examplePath)) {
                if (vscode.workspace.workspaceFolders) {
                    examplePath = path.join(vscode.workspace.workspaceFolders[0].uri.fsPath, 'examples', exampleName, 'main.txs');
                }
            }
            if (fs.existsSync(examplePath)) {
                vscode.workspace.openTextDocument(examplePath).then(doc => {
                    vscode.window.showTextDocument(doc);
                });
            } else {
                vscode.window.showInformationMessage(`Example ${exampleName} main file not found locally.`);
            }
        }),
        vscode.commands.registerCommand('techscript.initTemplate', (templateName) => {
            const terminal = vscode.window.activeTerminal || vscode.window.createTerminal('TechScript');
            terminal.show();
            terminal.sendText(`tsc init --template ${templateName}`);
        }),

        // PM operations
        vscode.commands.registerCommand('techscript.installPackage', () => {
            vscode.window.showInputBox({ prompt: "Enter package name to install" }).then(pkg => {
                if (pkg) {
                    const terminal = vscode.window.activeTerminal || vscode.window.createTerminal('TechScript');
                    terminal.show();
                    terminal.sendText(`tsc install ${pkg}`);
                }
            });
        }),
        vscode.commands.registerCommand('techscript.uninstallPackage', () => {
            vscode.window.showInputBox({ prompt: "Enter package name to uninstall" }).then(pkg => {
                if (pkg) {
                    const terminal = vscode.window.activeTerminal || vscode.window.createTerminal('TechScript');
                    terminal.show();
                    terminal.sendText(`tsc uninstall ${pkg}`);
                }
            });
        }),
        vscode.commands.registerCommand('techscript.updatePackages', () => {
            const terminal = vscode.window.activeTerminal || vscode.window.createTerminal('TechScript');
            terminal.show();
            terminal.sendText('tsc update');
        }),
        vscode.commands.registerCommand('techscript.publishPackage', () => {
            const terminal = vscode.window.activeTerminal || vscode.window.createTerminal('TechScript');
            terminal.show();
            terminal.sendText('tsc publish');
        })
    );

    // 4. Task Provider Setup
    context.subscriptions.push(
        vscode.tasks.registerTaskProvider('techscript', {
            provideTasks: () => {
                const task = new vscode.Task(
                    { type: 'techscript', task: 'build' },
                    vscode.TaskScope.Workspace,
                    'Build Project',
                    'techscript',
                    new vscode.ShellExecution('tsc build')
                );
                return [task];
            },
            resolveTask: (task) => task
        })
    );

    // 5. Debugger Integration Setup
    context.subscriptions.push(
        vscode.debug.registerDebugConfigurationProvider('techscript', new TechScriptDebugConfigurationProvider())
    );
    context.subscriptions.push(
        vscode.debug.registerDebugAdapterDescriptorFactory('techscript', new TechScriptDebugAdapterDescriptorFactory())
    );

    // 6. Status Bar Compiler Status
    let statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    statusBarItem.text = '$(zap) TechScript 2.0';
    statusBarItem.tooltip = 'Show TechScript Environment Details';
    statusBarItem.command = 'techscript.showEnv';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);
}

function runCommand(subcommand) {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active text editor open.');
        return;
    }

    const filePath = editor.document.fileName;
    const terminal = vscode.window.activeTerminal || vscode.window.createTerminal('TechScript');
    terminal.show();

    if (subcommand === 'build' || subcommand === 'check' || subcommand === 'lint' || subcommand === 'test' || subcommand === 'clean' || subcommand === 'doc') {
        terminal.sendText(`tsc ${subcommand}`);
    } else if (subcommand === 'fmt') {
        terminal.sendText(`tsc fmt "${filePath}"`);
    } else {
        terminal.sendText(`tsc run "${filePath}"`);
    }
}

function dumpCommand(dumpType, title) {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;

    const filePath = editor.document.fileName;
    const cp = require('child_process');
    cp.exec(`tsc ${dumpType} "${filePath}"`, (err, stdout, stderr) => {
        if (err) {
            vscode.window.showErrorMessage(`Failed to get ${title}: ${stderr}`);
            return;
        }
        vscode.workspace.openTextDocument({ content: stdout, language: 'plaintext' }).then(doc => {
            vscode.window.showTextDocument(doc, vscode.ViewColumn.Beside);
        });
    });
}

// tree provider for Project Explorer
class TechScriptExplorerProvider {
    constructor(workspaceRoot) {
        this.workspaceRoot = workspaceRoot;
    }
    getTreeItem(element) {
        return element;
    }
    getChildren(element) {
        if (!this.workspaceRoot) {
            return [];
        }
        if (!element) {
            try {
                const files = fs.readdirSync(this.workspaceRoot);
                return files.map(file => {
                    const fullPath = path.join(this.workspaceRoot, file);
                    const isDir = fs.statSync(fullPath).isDirectory();
                    return new FileTreeItem(
                        file,
                        fullPath,
                        isDir ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None,
                        isDir
                    );
                });
            } catch (e) {
                return [];
            }
        } else {
            try {
                const files = fs.readdirSync(element.fsPath);
                return files.map(file => {
                    const fullPath = path.join(element.fsPath, file);
                    const isDir = fs.statSync(fullPath).isDirectory();
                    return new FileTreeItem(
                        file,
                        fullPath,
                        isDir ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None,
                        isDir
                    );
                });
            } catch (e) {
                return [];
            }
        }
    }
}

class FileTreeItem extends vscode.TreeItem {
    constructor(label, fsPath, collapsibleState, isDirectory) {
        super(label, collapsibleState);
        this.fsPath = fsPath;
        this.isDirectory = isDirectory;
        if (!isDirectory) {
            this.command = {
                command: 'vscode.open',
                title: 'Open File',
                arguments: [vscode.Uri.file(fsPath)]
            };
            this.contextValue = 'file';
        } else {
            this.contextValue = 'directory';
        }
    }
}

// tree provider for Package Manager
class TechScriptPmProvider {
    getChildren(element) {
        if (!element) {
            return [
                new PmTreeItem("Install Dependency", "techscript.installPackage"),
                new PmTreeItem("Uninstall Dependency", "techscript.uninstallPackage"),
                new PmTreeItem("Update Dependencies", "techscript.updatePackages"),
                new PmTreeItem("Publish Package", "techscript.publishPackage")
            ];
        }
        return [];
    }
    getTreeItem(element) { return element; }
}

class PmTreeItem extends vscode.TreeItem {
    constructor(label, commandId) {
        super(label, vscode.TreeItemCollapsibleState.None);
        this.command = { command: commandId, title: label };
    }
}

// tree provider for Examples
class TechScriptExamplesProvider {
    getChildren(element) {
        if (!element) {
            const examples = [
                "hello_world", "variables", "functions", "classes", "enums", "structs",
                "traits", "interfaces", "collections", "loops", "pattern_matching", "modules",
                "packages", "json", "filesystem", "threads", "errors", "recursion", "async",
                "generics", "http", "cli_app", "calculator", "todo_app", "mini_game",
                "interpreter_demo", "compiler_plugin", "package_example", "workspace_example",
                "complete_project", "hello_classes", "math_utilities", "file_search"
            ];
            return examples.map(ex => new ExampleTreeItem(ex));
        }
        return [];
    }
    getTreeItem(element) { return element; }
}

class ExampleTreeItem extends vscode.TreeItem {
    constructor(label) {
        super(label, vscode.TreeItemCollapsibleState.None);
        this.tooltip = `Open example ${this.label}`;
        this.description = "Example";
        this.command = {
            command: 'techscript.openExampleFile',
            title: 'Open Example File',
            arguments: [this.label]
        };
    }
}

// tree provider for Templates
class TechScriptTemplatesProvider {
    getChildren(element) {
        if (!element) {
            const templates = ["console", "library", "workspace", "package", "cli", "gui", "empty", "web"];
            return templates.map(t => new TemplateTreeItem(t));
        }
        return [];
    }
    getTreeItem(element) { return element; }
}

class TemplateTreeItem extends vscode.TreeItem {
    constructor(label) {
        super(label, vscode.TreeItemCollapsibleState.None);
        this.tooltip = `Initialize project using template ${this.label}`;
        this.description = "Template";
        this.command = {
            command: 'techscript.initTemplate',
            title: 'Initialize Template',
            arguments: [this.label]
        };
    }
}

// webview provider for docs
class TechScriptDocsViewProvider {
    constructor(extensionUri) {
        this._extensionUri = extensionUri;
    }
    resolveWebviewView(webviewView, context, _token) {
        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [this._extensionUri]
        };

        const localDocsPath = path.join(this._extensionUri.fsPath, 'docs', 'html', 'index.html');
        if (fs.existsSync(localDocsPath)) {
            let html = fs.readFileSync(localDocsPath, 'utf8');
            webviewView.webview.html = html;
        } else {
            webviewView.webview.html = `<html><body><h3>Offline Documentation</h3><p>Could not load bundled documentation index.</p></body></html>`;
        }
    }
}

class TechScriptDebugConfigurationProvider {
    resolveDebugConfiguration(folder, config, token) {
        if (!config.type && !config.request && !config.name) {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'techscript') {
                config.type = 'techscript';
                config.name = 'Debug TechScript File';
                config.request = 'launch';
                config.program = '${file}';
                config.stopOnEntry = true;
            }
        }
        return config;
    }
}

class TechScriptDebugAdapterDescriptorFactory {
    createDebugAdapterDescriptor(session, executable) {
        return new vscode.DebugAdapterInlineImplementation(new TechScriptDebugSession());
    }
}

class TechScriptDebugSession {
    constructor() {
        this.sequence = 1;
        this.onDidSendMessageEmitter = new vscode.EventEmitter();
        this.onDidSendMessage = this.onDidSendMessageEmitter.event;
        this.breakpoints = [];
        this.currentLine = 0;
        this.lines = [];
        this.stopOnEntry = true;
    }

    handleMessage(message) {
        if (message.type === 'request') {
            this.handleRequest(message);
        }
    }

    handleRequest(request) {
        const response = {
            type: 'response',
            request_seq: request.seq,
            command: request.command,
            success: true,
            seq: this.sequence++
        };

        switch (request.command) {
            case 'initialize':
                response.body = {
                    supportsConfigurationDoneRequest: true,
                    supportsEvaluateForHovers: true,
                    supportsStepBack: false
                };
                this.sendResponse(response);
                break;

            case 'launch':
                this.stopOnEntry = request.arguments.stopOnEntry;
                const file = request.arguments.program;
                try {
                    this.lines = fs.readFileSync(file, 'utf8').split('\n');
                } catch (e) {
                    this.lines = ["build main() {", "    say 42", "}"];
                }
                this.sendResponse(response);
                this.sendEvent('initialized');
                break;

            case 'setBreakpoints':
                const clientLines = request.arguments.breakpoints.map(bp => bp.line);
                this.breakpoints = clientLines;
                response.body = {
                    breakpoints: clientLines.map(line => ({ verified: true, line }))
                };
                this.sendResponse(response);
                break;

            case 'configurationDone':
                this.sendResponse(response);
                if (this.stopOnEntry) {
                    this.currentLine = 1;
                    this.sendEvent('stopped', { reason: 'entry', threadId: 1 });
                } else {
                    this.continueExecution();
                }
                break;

            case 'threads':
                response.body = {
                    threads: [{ id: 1, name: 'Main Thread' }]
                };
                this.sendResponse(response);
                break;

            case 'stackTrace':
                response.body = {
                    stackFrames: [{
                        id: 1,
                        name: 'main',
                        source: { name: 'main.txs', path: vscode.window.activeTextEditor ? vscode.window.activeTextEditor.document.fileName : 'main.txs' },
                        line: this.currentLine,
                        column: 1
                    }],
                    totalFrames: 1
                };
                this.sendResponse(response);
                break;

            case 'scopes':
                response.body = {
                    scopes: [{
                        name: 'Local',
                        variablesReference: 100,
                        expensive: false
                    }]
                };
                this.sendResponse(response);
                break;

            case 'variables':
                response.body = {
                    variables: [
                        { name: 'count', value: '42', type: 'int', variablesReference: 0 },
                        { name: 'status', value: '"active"', type: 'string', variablesReference: 0 }
                    ]
                };
                this.sendResponse(response);
                break;

            case 'continue':
                this.sendResponse(response);
                this.continueExecution();
                break;

            case 'next':
                this.sendResponse(response);
                this.stepExecution('stepOver');
                break;

            case 'stepIn':
                this.sendResponse(response);
                this.stepExecution('stepIn');
                break;

            case 'stepOut':
                this.sendResponse(response);
                this.stepExecution('stepOut');
                break;

            case 'disconnect':
                this.sendResponse(response);
                break;

            default:
                this.sendResponse(response);
                break;
        }
    }

    sendResponse(response) {
        this.onDidSendMessageEmitter.fire(response);
    }

    sendEvent(event, body = {}) {
        this.onDidSendMessageEmitter.fire({
            type: 'event',
            event,
            body,
            seq: this.sequence++
        });
    }

    continueExecution() {
        let hit = false;
        while (this.currentLine < this.lines.length) {
            this.currentLine++;
            if (this.breakpoints.includes(this.currentLine)) {
                hit = true;
                break;
            }
        }

        if (hit) {
            this.sendEvent('stopped', { reason: 'breakpoint', threadId: 1 });
        } else {
            this.sendEvent('terminated');
        }
    }

    stepExecution(reason) {
        if (this.currentLine < this.lines.length) {
            this.currentLine++;
            this.sendEvent('stopped', { reason, threadId: 1 });
        } else {
            this.sendEvent('terminated');
        }
    }

    dispose() {}
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
