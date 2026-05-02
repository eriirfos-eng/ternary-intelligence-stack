// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang VS Code Extension — RFI-IRFOS
// v1.0.2 — run, build, check, exec, REPL, check-on-save diagnostics, TernAudit selection

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
        vscode.commands.registerCommand('ternlang.auditSelection', () => auditSelection(context)),
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

// ── TernAudit Selection ───────────────────────────────────────────────────────

// Active decoration type — disposed when a new audit runs or after timeout.
let _auditDecoration = null;

function auditSelection(context) {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('TernAudit: no active editor.');
        return;
    }
    const selection = editor.selection;
    if (selection.isEmpty) {
        vscode.window.showWarningMessage('TernAudit: select a block of text first.');
        return;
    }

    const selectedText = editor.document.getText(selection);
    const result = analyzeAuditText(selectedText);

    // Clear any previous decoration
    if (_auditDecoration) { _auditDecoration.dispose(); _auditDecoration = null; }

    // Inline annotation after the last line of the selection
    const tritLabel = result.trit === 1 ? '+1 AFFIRM' : result.trit === -1 ? '-1 REJECT' : '0 HOLD';
    const tritColor = result.trit === 1 ? '#4ec9b0' : result.trit === -1 ? '#f44747' : '#dcdcaa';
    _auditDecoration = vscode.window.createTextEditorDecorationType({
        after: {
            contentText: `  ← TernAudit [trit: ${tritLabel} | conf: ${(result.confidence * 100).toFixed(0)}%]`,
            color: tritColor,
            fontStyle: 'italic',
        }
    });
    editor.setDecorations(_auditDecoration, [selection]);
    setTimeout(() => { if (_auditDecoration) { _auditDecoration.dispose(); _auditDecoration = null; } }, 12000);

    // Webview panel with full report
    const panel = vscode.window.createWebviewPanel(
        'ternAuditReport', 'TernAudit Report', vscode.ViewColumn.Beside, {}
    );
    panel.webview.html = buildAuditHtml(result, selectedText);
}

function analyzeAuditText(text) {
    const lower = text.toLowerCase();

    const positiveWords = ['affirm', 'approve', 'proceed', 'allow', 'valid', 'accept', 'pass', 'safe', 'compliant', 'authorized', 'permit', 'granted'];
    const negativeWords = ['reject', 'deny', 'block', 'invalid', 'error', 'unsafe', 'violation', 'unauthorized', 'fail', 'forbidden', 'prohibited', 'revoke'];
    const holdWords    = ['hold', 'pending', 'uncertain', 'unknown', 'review', 'escalate', 'defer', 'inconclusive', 'abstain'];

    let pos = 0, neg = 0, hld = 0;

    // Explicit trit values in text (e.g. "+1", "-1", "trit: 0")
    for (const m of (text.match(/\btrit\s*[:=]\s*([+-]?\d)\b/gi) || [])) {
        const v = parseInt(m.replace(/\btrit\s*[:=]\s*/i, ''));
        if (v === 1) pos += 2; else if (v === -1) neg += 2; else hld += 2;
    }
    for (const m of (text.match(/\b[+-]1\b/g) || [])) {
        if (m === '+1' || m === '1') pos++; else neg++;
    }

    for (const w of positiveWords) pos += (lower.match(new RegExp('\\b' + w + '\\w*', 'g')) || []).length;
    for (const w of negativeWords) neg += (lower.match(new RegExp('\\b' + w + '\\w*', 'g')) || []).length;
    for (const w of holdWords)     hld += (lower.match(new RegExp('\\b' + w + '\\w*', 'g')) || []).length;

    const total = pos + neg + hld;
    let trit = 0, confidence = 0.0;
    if (total > 0) {
        if (pos > neg && pos > hld)      { trit =  1; confidence = pos / total; }
        else if (neg > pos && neg > hld) { trit = -1; confidence = neg / total; }
        else                             { trit =  0; confidence = hld > 0 ? hld / total : 0.33; }
    }

    // EU AI Act article flags
    const flags = [];
    if (neg > 0) flags.push('Article 13 — transparency obligation may apply');
    if (text.length > 500) flags.push('Article 14 — human oversight recommended for long decision chains');
    if (/(personal|biometric|health|financial)/i.test(text)) flags.push('Article 15 — accuracy & robustness check required for sensitive data');

    return {
        trit,
        confidence: Math.round(confidence * 100) / 100,
        positiveCount: pos,
        negativeCount: neg,
        holdCount: hld,
        totalSignals: total,
        flags,
        charCount: text.length,
        wordCount: text.trim().split(/\s+/).length,
    };
}

function buildAuditHtml(result, selectedText) {
    const tritLabel = result.trit === 1 ? '+1 AFFIRM' : result.trit === -1 ? '-1 REJECT' : '0 HOLD';
    const tritColor = result.trit === 1 ? '#4ec9b0'   : result.trit === -1 ? '#f44747'   : '#dcdcaa';
    const flagsHtml = result.flags.length
        ? result.flags.map(f => `<li>⚠ ${f}</li>`).join('')
        : '<li style="color:#4ec9b0">✓ No EU AI Act flags detected</li>';
    const preview = selectedText.length > 300 ? selectedText.slice(0, 300) + '…' : selectedText;

    return `<!DOCTYPE html><html><head><meta charset="UTF-8">
<style>
  body { font-family: 'Segoe UI', sans-serif; background: #1e1e1e; color: #d4d4d4; padding: 20px; }
  h1   { color: #569cd6; font-size: 1.2em; margin-bottom: 4px; }
  .verdict { font-size: 2em; font-weight: bold; color: ${tritColor}; margin: 12px 0; }
  .conf    { color: #9cdcfe; font-size: 0.95em; }
  .signals { display: flex; gap: 20px; margin: 12px 0; }
  .sig     { text-align: center; }
  .sig .n  { font-size: 1.4em; font-weight: bold; }
  .sig.pos .n { color: #4ec9b0; }
  .sig.neg .n { color: #f44747; }
  .sig.hld .n { color: #dcdcaa; }
  .sig .l  { font-size: 0.75em; color: #808080; }
  .flags   { background: #252526; border-radius: 4px; padding: 10px 14px; margin: 12px 0; }
  .flags ul { margin: 4px 0; padding-left: 18px; }
  .flags li { margin: 4px 0; font-size: 0.88em; }
  .preview { background: #2d2d2d; border-radius: 4px; padding: 10px 14px; font-size: 0.82em;
             font-family: monospace; white-space: pre-wrap; word-break: break-word; color: #b5cea8; }
  .meta    { font-size: 0.8em; color: #6a9955; margin-top: 16px; }
</style></head><body>
<h1>🔍 TernAudit Guard — Decision Analysis</h1>
<div class="verdict">${tritLabel}</div>
<div class="conf">Confidence: ${(result.confidence * 100).toFixed(1)}% &nbsp;|&nbsp; Signals: ${result.totalSignals} &nbsp;|&nbsp; ${result.wordCount} words</div>
<div class="signals">
  <div class="sig pos"><div class="n">${result.positiveCount}</div><div class="l">AFFIRM</div></div>
  <div class="sig neg"><div class="n">${result.negativeCount}</div><div class="l">REJECT</div></div>
  <div class="sig hld"><div class="n">${result.holdCount}</div><div class="l">HOLD</div></div>
</div>
<div class="flags"><strong>EU AI Act Compliance</strong><ul>${flagsHtml}</ul></div>
<div class="preview">${preview.replace(/</g,'&lt;').replace(/>/g,'&gt;')}</div>
<div class="meta">RFI-IRFOS TernAudit Guard v1.0.2 · ZVR: 1015608684 · Patent A50296/2026</div>
</body></html>`;
}

function deactivate() {
    if (diagnosticCollection) diagnosticCollection.dispose();
    if (_auditDecoration) _auditDecoration.dispose();
}

module.exports = { activate, deactivate };
