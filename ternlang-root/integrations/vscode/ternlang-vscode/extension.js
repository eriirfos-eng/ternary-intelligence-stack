// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang VS Code Extension — RFI-IRFOS
// v0.3.3 — run, build, check, exec, REPL, check-on-save diagnostics

'use strict';

const vscode = require('vscode');
const path   = require('path');
const cp     = require('child_process');

// ── Diagnostics collection (check-on-save) ───────────────────────────────────
let diagnosticCollection;

function activate(context) {
    diagnosticCollection = vscode.languages.createDiagnosticCollection('ternlang');
    context.subscriptions.push(diagnosticCollection);

    // ── Commands ──────────────────────────────────────────────────────────────

    context.subscriptions.push(
        vscode.commands.registerCommand('ternlang.run', () => runInTerminal('run')),
        vscode.commands.registerCommand('ternlang.runDebug', () => runInTerminal('run', ['--debug'])),
        vscode.commands.registerCommand('ternlang.build', () => runBuild()),
        vscode.commands.registerCommand('ternlang.check', () => runCheckCommand()),
        vscode.commands.registerCommand('ternlang.repl', () => openRepl()),
    );

    // ── Check-on-save ─────────────────────────────────────────────────────────
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(doc => {
            if (doc.languageId === 'ternlang') {
                const cfg = vscode.workspace.getConfiguration('ternlang');
                if (cfg.get('checkOnSave')) {
                    runCheckDiagnostics(doc);
                }
            }
        })
    );

    // ── Check open .tern files on startup ─────────────────────────────────────
    vscode.workspace.textDocuments.forEach(doc => {
        if (doc.languageId === 'ternlang') runCheckDiagnostics(doc);
    });
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(doc => {
            if (doc.languageId === 'ternlang') runCheckDiagnostics(doc);
        })
    );
}

// ── Helpers ───────────────────────────────────────────────────────────────────

function ternlangBin() {
    return vscode.workspace.getConfiguration('ternlang').get('executablePath') || 'ternlang';
}

function activeFilePath() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('Ternlang: no active .tern file.');
        return null;
    }
    if (editor.document.languageId !== 'ternlang') {
        vscode.window.showWarningMessage('Ternlang: active file is not a .tern file.');
        return null;
    }
    return editor.document.fileName;
}

function getOrCreateTerminal() {
    const existing = vscode.window.terminals.find(t => t.name === 'Ternlang');
    return existing || vscode.window.createTerminal({ name: 'Ternlang' });
}

function runInTerminal(subcommand, extraArgs = []) {
    const file = activeFilePath();
    if (!file) return;
    const bin  = ternlangBin();
    const args = [subcommand, `"${file}"`, ...extraArgs].join(' ');
    const terminal = getOrCreateTerminal();
    terminal.show(true);
    terminal.sendText(`${bin} ${args}`);
}

function runBuild() {
    const file = activeFilePath();
    if (!file) return;
    const tbc  = file.replace(/\.tern$/, '.tbc');
    const bin  = ternlangBin();
    const terminal = getOrCreateTerminal();
    terminal.show(true);
    terminal.sendText(`${bin} build "${file}" -o "${tbc}"`);
}

function runCheckCommand() {
    const file = activeFilePath();
    if (!file) return;
    const bin  = ternlangBin();
    const terminal = getOrCreateTerminal();
    terminal.show(true);
    terminal.sendText(`${bin} check "${file}"`);
}

function openRepl() {
    const bin = ternlangBin();
    const terminal = getOrCreateTerminal();
    terminal.show(true);
    terminal.sendText(`${bin} repl`);
}

// ── Check-on-save diagnostics ─────────────────────────────────────────────────
// Runs `ternlang check <file>` and parses any error output into VS Code diagnostics.
// Error format from the CLI: "  error  ExpectedToken(...)" on the same line as the filename.

function runCheckDiagnostics(doc) {
    const bin  = ternlangBin();
    const file = doc.fileName;

    cp.execFile(bin, ['check', file], { timeout: 5000 }, (err, stdout, stderr) => {
        diagnosticCollection.delete(doc.uri);
        if (!err) return; // no errors

        const output = (stdout + stderr);
        const diagnostics = [];

        // The CLI outputs:  "  <filepath>  error  <message>"
        // Since we can't get line numbers from a parse error without LSP,
        // we surface the error message at line 1 until ternlang-lsp is active.
        for (const line of output.split('\n')) {
            const m = line.match(/error\s+(.+)$/);
            if (m) {
                const msg = m[1].trim();
                const range = new vscode.Range(0, 0, 0, 0);
                diagnostics.push(new vscode.Diagnostic(range, `ternlang: ${msg}`, vscode.DiagnosticSeverity.Error));
            }
        }

        if (diagnostics.length > 0) {
            diagnosticCollection.set(doc.uri, diagnostics);
        }
    });
}

function deactivate() {
    if (diagnosticCollection) diagnosticCollection.dispose();
}

module.exports = { activate, deactivate };
