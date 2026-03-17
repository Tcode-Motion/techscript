import * as path from "path";
import * as fs from "fs";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

function platformKey(): string {
  // VS Code uses Node's process.platform / process.arch.
  // We map into a stable folder name for prebuilt binaries.
  const plat = process.platform;
  const arch = process.arch;
  if (plat === "win32" && arch === "x64") return "win32-x64";
  if (plat === "darwin" && arch === "x64") return "darwin-x64";
  if (plat === "darwin" && arch === "arm64") return "darwin-arm64";
  if (plat === "linux" && arch === "x64") return "linux-x64";
  if (plat === "linux" && arch === "arm64") return "linux-arm64";
  return `${plat}-${arch}`;
}

function bundledServerPath(context: vscode.ExtensionContext): string {
  const exe = process.platform === "win32" ? "techscript-lsp.exe" : "techscript-lsp";
  return context.asAbsolutePath(path.join("server", platformKey(), exe));
}

function resolveServerCommand(context: vscode.ExtensionContext): { command: string; args: string[] } {
  const bundled = bundledServerPath(context);
  if (fs.existsSync(bundled)) {
    return { command: bundled, args: [] };
  }

  // Fallback: require techscript-lsp on PATH.
  // This keeps the extension functional even if prebuilt binaries are not shipped yet.
  const cmd = process.platform === "win32" ? "techscript-lsp.exe" : "techscript-lsp";
  return { command: cmd, args: [] };
}

export async function activate(context: vscode.ExtensionContext) {
  const output = vscode.window.createOutputChannel("TechScript");
  output.appendLine("TechScript extension activating…");

  const { command, args } = resolveServerCommand(context);

  const serverOptions: ServerOptions = {
    command,
    args,
    options: {
      env: process.env,
    },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "techscript" }],
    outputChannel: output,
  };

  client = new LanguageClient("techscript", "TechScript Language Server", serverOptions, clientOptions);

  try {
    await client.start();
    output.appendLine(`TechScript LSP started: ${command}`);
  } catch (e) {
    output.appendLine(`Failed to start TechScript LSP: ${String(e)}`);
    void vscode.window.showWarningMessage(
      "TechScript: could not start the Language Server. If you don't have a bundled LSP yet, install/build `techscript-lsp` and ensure it's on PATH."
    );
  }
}

export async function deactivate() {
  if (client) {
    await client.stop();
    client = undefined;
  }
}

