// ternlang VS Code extension
// Tier 1 (Free): syntax, snippets, run command, status bar, graceful LSP
// Tier 2 (Pro):  cloud LSP + inline trit hints  — requires tern_t2_... API key
// Tier 3 (Ind):  BET VM debugger + tensor view   — requires tern_t3_... API key
// Tier 4 (Ent):  cluster panel + agent monitor   — requires tern_t4_... API key

const vscode = require('vscode');
const path   = require('path');
const fs     = require('fs');
const cp     = require('child_process');
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');

let client;
let statusBar;
const OUTPUT = vscode.window.createOutputChannel('Ternlang');

// ── Tier detection ──────────────────────────────────────────────────────────

function getTier(apiKey) {
    if (!apiKey) return 1;
    if (apiKey.startsWith('tern_t4_')) return 4;
    if (apiKey.startsWith('tern_t3_')) return 3;
    if (apiKey.startsWith('tern_t2_')) return 2;
    return 1;
}

function tierLabel(t) {
    return ['', 'Free', 'Pro', 'Industrial', 'Enterprise'][t] || 'Free';
}

// ── Status bar ──────────────────────────────────────────────────────────────

function updateStatusBar(text, tooltip) {
    if (!statusBar) return;
    statusBar.text    = `$(symbol-boolean) Ternlang ${text}`;
    statusBar.tooltip = tooltip || 'Ternlang — Balanced Ternary Language';
}

// ── Run command (Tier 1) ────────────────────────────────────────────────────

async function runTernFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'ternlang') {
        vscode.window.showWarningMessage('Open a .tern file first.');
        return;
    }
    await editor.document.save();
    const file = editor.document.fileName;

    // Locate the CLI: prefer setting, then PATH, then default build path
    const conf    = vscode.workspace.getConfiguration('ternlang');
    const cliPath = conf.get('cliPath') || 'ternlang-cli';

    OUTPUT.clear();
    OUTPUT.show(true);
    OUTPUT.appendLine(`▶  ternlang-cli run "${path.basename(file)}"\n`);
    updateStatusBar('$(loading~spin)', 'Running…');

    let finished = false;
    const proc = cp.spawn(cliPath, ['run', file], { shell: false });

    proc.stdout.on('data', d => OUTPUT.append(d.toString()));
    proc.stderr.on('data', d => OUTPUT.append(d.toString()));

    proc.on('close', code => {
        finished = true;
        if (code === 0) {
            OUTPUT.appendLine('\n✓  exited 0');
            updateStatusBar('$(pass)', 'Last run: OK');
        } else {
            OUTPUT.appendLine(`\n✗  exited ${code}`);
            updateStatusBar('$(error)', `Last run: exit ${code}`);
        }
    });

    proc.on('error', err => {
        finished = true;
        if (err.code === 'ENOENT') {
            OUTPUT.appendLine(
                `\n✗  ternlang-cli not found on PATH.\n` +
                `   Build it:  cd ternlang-root && cargo build --release\n` +
                `   Then set:  ternlang.cliPath in settings.`
            );
            vscode.window.showErrorMessage(
                'ternlang-cli not found. Build it first (cargo build --release).',
                'Open Settings'
            ).then(sel => {
                if (sel === 'Open Settings')
                    vscode.commands.executeCommand('workbench.action.openSettings', 'ternlang.cliPath');
            });
        } else {
            OUTPUT.appendLine(`\n✗  ${err.message}`);
        }
        updateStatusBar('$(error)', 'CLI not found');
    });
}

// ── Tier-gated command stubs ────────────────────────────────────────────────

function makeUpgradeCommand(requiredTier, featureName) {
    return function () {
        const conf    = vscode.workspace.getConfiguration('ternlang');
        const apiKey  = conf.get('apiKey') || '';
        const current = getTier(apiKey);
        if (current >= requiredTier) {
            vscode.window.showInformationMessage(
                `${featureName} — coming in the next extension release. Stay tuned!`
            );
        } else {
            vscode.window.showInformationMessage(
                `${featureName} requires Tier ${requiredTier} (${tierLabel(requiredTier)}).`,
                'Get API key at ternlang.com'
            ).then(sel => {
                if (sel) vscode.env.openExternal(vscode.Uri.parse('https://ternlang.com/#pricing'));
            });
        }
    };
}

// ── LSP (Tier 1 local or Tier 2+ cloud) ────────────────────────────────────

function startLocalLsp(context) {
    const lspBin = path.join(context.extensionPath, 'bin', 'ternlang-lsp');
    if (!fs.existsSync(lspBin)) {
        const conf = vscode.workspace.getConfiguration('ternlang');
        if (!conf.get('suppressLspWarning')) {
            vscode.window.showInformationMessage(
                'ternlang-lsp not found — hover & diagnostics disabled. Build with: cargo build --release',
                'Dismiss', "Don't show again"
            ).then(sel => {
                if (sel === "Don't show again")
                    conf.update('suppressLspWarning', true, true);
            });
        }
        return;
    }

    const serverOptions = { command: lspBin, transport: TransportKind.stdio };
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'ternlang' }],
        synchronize: { fileEvents: vscode.workspace.createFileSystemWatcher('**/*.tern') }
    };
    client = new LanguageClient('ternlang-lsp', 'Ternlang Language Server', serverOptions, clientOptions);
    client.start();
}

// ── Activate ────────────────────────────────────────────────────────────────

function activate(context) {
    // Status bar
    statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    statusBar.command = 'ternlang.showStatus';
    context.subscriptions.push(statusBar);

    const conf  = vscode.workspace.getConfiguration('ternlang');
    const tier  = getTier(conf.get('apiKey') || '');
    updateStatusBar(tier > 1 ? `[${tierLabel(tier)}]` : '', `Ternlang — ${tierLabel(tier)} tier`);
    statusBar.show();

    // Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('ternlang.run', runTernFile),

        vscode.commands.registerCommand('ternlang.showStatus', () => {
            const k    = conf.get('apiKey') || '';
            const t    = getTier(k);
            const msg  = t === 1
                ? 'Tier 1 — Free. Set ternlang.apiKey to unlock Pro features.'
                : `Tier ${t} — ${tierLabel(t)}. API key active.`;
            vscode.window.showInformationMessage(msg, 'Open Settings').then(sel => {
                if (sel) vscode.commands.executeCommand('workbench.action.openSettings', 'ternlang');
            });
        }),

        // Tier 2 stubs
        vscode.commands.registerCommand('ternlang.inlineTritHints',
            makeUpgradeCommand(2, 'Inline trit value hints')),
        vscode.commands.registerCommand('ternlang.cloudDiagnostics',
            makeUpgradeCommand(2, 'Cloud diagnostics (ternlang.com LSP)')),

        // Tier 3 stubs
        vscode.commands.registerCommand('ternlang.debugBetVm',
            makeUpgradeCommand(3, 'BET VM step debugger')),
        vscode.commands.registerCommand('ternlang.viewTensor',
            makeUpgradeCommand(3, 'Tensor visualizer')),
        vscode.commands.registerCommand('ternlang.sparseskipCoverage',
            makeUpgradeCommand(3, '@sparseskip coverage overlay')),

        // Tier 4 stubs
        vscode.commands.registerCommand('ternlang.clusterPanel',
            makeUpgradeCommand(4, 'Remote cluster panel')),
        vscode.commands.registerCommand('ternlang.agentMonitor',
            makeUpgradeCommand(4, 'Agent monitor dashboard'))
    );

    // LSP
    startLocalLsp(context);
}

function deactivate() {
    if (client) return client.stop();
}

module.exports = { activate, deactivate };
