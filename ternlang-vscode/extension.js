// ternlang VS Code extension — v0.4.0
// Tier 1 (Free):    syntax, snippets, run command, status bar, LSP (auto-downloaded)
// Tier 2 (Pro):     inline trit ghost decorations after run  — tern_t2_... API key
// Tier 3 (Ind):     BET VM debugger, tensor view, @sparseskip overlay — tern_t3_...
// Tier 4 (Ent):     cluster panel, agent monitor              — tern_t4_...

const vscode  = require('vscode');
const path    = require('path');
const fs      = require('fs');
const https   = require('https');
const cp      = require('child_process');
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

// ── Inline trit decorations (Tier 2) ────────────────────────────────────────

const DECO = {
    affirm: vscode.window.createTextEditorDecorationType({
        after: { contentText: '  → Affirm', color: '#4ec9b0', fontStyle: 'italic', margin: '0 0 0 8px' }
    }),
    tend: vscode.window.createTextEditorDecorationType({
        after: { contentText: '  → Tend', color: '#dcdcaa', fontStyle: 'italic', margin: '0 0 0 8px' }
    }),
    reject: vscode.window.createTextEditorDecorationType({
        after: { contentText: '  → Reject', color: '#f44747', fontStyle: 'italic', margin: '0 0 0 8px' }
    }),
};

function clearDecorations(editor) {
    editor.setDecorations(DECO.affirm, []);
    editor.setDecorations(DECO.tend,   []);
    editor.setDecorations(DECO.reject, []);
}

/**
 * Parse TERN_SYMBOLS line from stderr: "TERN_SYMBOLS:a=0,b=1,result=2"
 * Returns Map<regNum, varName>
 */
function parseSymbols(stderrText) {
    const match = stderrText.match(/TERN_SYMBOLS:([^\n]*)/);
    if (!match || !match[1].trim()) return new Map();
    const regToVar = new Map();
    for (const pair of match[1].split(',')) {
        const [name, reg] = pair.split('=');
        if (name && reg !== undefined) regToVar.set(parseInt(reg, 10), name.trim());
    }
    return regToVar;
}

/**
 * Parse register dump from stdout: "Reg 0: trit(affirm)" etc.
 * Returns Map<regNum, 'affirm'|'tend'|'reject'>
 */
function parseRegisterDump(stdoutText) {
    const regStates = new Map();
    for (const line of stdoutText.split('\n')) {
        const m = line.match(/^Reg\s+(\d+):\s+trit\((\w+)\)/);
        if (m) regStates.set(parseInt(m[1], 10), m[2].toLowerCase());
    }
    return regStates;
}

/**
 * Apply ghost decorations to the active editor.
 * varStates: Map<varName, 'affirm'|'tend'|'reject'>
 */
function applyDecorations(editor, varStates) {
    if (!editor || varStates.size === 0) return;
    const doc  = editor.document;
    const text = doc.getText();
    const lines = text.split('\n');

    const affirms = [], tends = [], rejects = [];

    for (const [varName, state] of varStates) {
        // Match: let [mut] varName [: type]
        const pattern = new RegExp(`\\blet\\s+(?:mut\\s+)?${varName}\\b`);
        for (let i = 0; i < lines.length; i++) {
            if (pattern.test(lines[i])) {
                const range = new vscode.Range(i, lines[i].length, i, lines[i].length);
                const deco  = { range };
                if (state === 'affirm')  affirms.push(deco);
                else if (state === 'tend')    tends.push(deco);
                else if (state === 'reject')  rejects.push(deco);
                break; // first declaration wins
            }
        }
    }

    editor.setDecorations(DECO.affirm, affirms);
    editor.setDecorations(DECO.tend,   tends);
    editor.setDecorations(DECO.reject, rejects);
}

// ── Status bar ──────────────────────────────────────────────────────────────

function updateStatusBar(text, tooltip) {
    if (!statusBar) return;
    statusBar.text    = `$(symbol-boolean) Ternlang ${text}`;
    statusBar.tooltip = tooltip || 'Ternlang — Balanced Ternary Language';
}

// ── Run command (Tier 1 + Tier 2 hints) ─────────────────────────────────────

async function runTernFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'ternlang') {
        vscode.window.showWarningMessage('Open a .tern file first.');
        return;
    }
    await editor.document.save();

    const conf    = vscode.workspace.getConfiguration('ternlang');
    const apiKey  = conf.get('apiKey') || '';
    const tier    = getTier(apiKey);
    const cliPath = conf.get('cliPath') || 'ternlang-cli';
    const file    = editor.document.fileName;

    OUTPUT.clear();
    OUTPUT.show(true);
    OUTPUT.appendLine(`▶  ternlang-cli run "${path.basename(file)}"\n`);
    updateStatusBar('$(loading~spin)', 'Running…');

    // Tier 2+: request symbol map so we can annotate bindings
    const args = ['run', file];
    if (tier >= 2) args.push('--emit-symbols');

    let stdoutBuf = '';
    let stderrBuf = '';

    const proc = cp.spawn(cliPath, args, { shell: false });
    proc.stdout.on('data', d => { stdoutBuf += d.toString(); OUTPUT.append(d.toString()); });
    proc.stderr.on('data', d => { stderrBuf += d.toString(); });  // symbols live here

    proc.on('close', code => {
        if (code === 0) {
            OUTPUT.appendLine('\n✓  exited 0');
            updateStatusBar('$(pass)', 'Last run: OK');
        } else {
            OUTPUT.appendLine(`\n✗  exited ${code}`);
            updateStatusBar('$(error)', `Last run: exit ${code}`);
        }

        // Tier 2: apply inline trit ghost decorations
        if (tier >= 2) {
            const regToVar  = parseSymbols(stderrBuf);
            const regStates = parseRegisterDump(stdoutBuf);
            const varStates = new Map();
            for (const [reg, varName] of regToVar) {
                const state = regStates.get(reg);
                if (state) varStates.set(varName, state);
            }
            const activeEd = vscode.window.activeTextEditor;
            if (activeEd && activeEd.document.fileName === file) {
                applyDecorations(activeEd, varStates);
            }
        }
    });

    proc.on('error', err => {
        if (err.code === 'ENOENT') {
            OUTPUT.appendLine(
                `\n✗  ternlang-cli not found.\n` +
                `   Build it:  cd ternlang-root && cargo build --release\n` +
                `   Or set:    ternlang.cliPath in settings.`
            );
            vscode.window.showErrorMessage(
                'ternlang-cli not found. Build it first.',
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

// ── LSP: platform-aware binary, auto-download from GitHub Releases ──────────

function platformBinaryName() {
    const p = process.platform;
    const a = process.arch;
    if (p === 'linux'  && a === 'x64')   return 'ternlang-lsp-linux-x64';
    if (p === 'linux'  && a === 'arm64') return 'ternlang-lsp-linux-arm64';
    if (p === 'darwin' && a === 'x64')   return 'ternlang-lsp-darwin-x64';
    if (p === 'darwin' && a === 'arm64') return 'ternlang-lsp-darwin-x64'; // Rosetta fallback
    if (p === 'win32'  && a === 'x64')   return 'ternlang-lsp-win32-x64.exe';
    return null;
}

function downloadFile(url, dest) {
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(dest);
        const get = (u) => {
            https.get(u, res => {
                if (res.statusCode === 301 || res.statusCode === 302) {
                    get(res.headers.location); return;
                }
                if (res.statusCode !== 200) {
                    reject(new Error(`HTTP ${res.statusCode} downloading LSP binary`)); return;
                }
                res.pipe(file);
                file.on('finish', () => { file.close(); resolve(); });
            }).on('error', reject);
        };
        get(url);
    });
}

async function ensureLspBinary(context) {
    const binName    = platformBinaryName();
    if (!binName) return null; // unsupported platform

    // 1. Check bundled binary (matches this platform)
    const bundled = path.join(context.extensionPath, 'bin', binName);
    if (fs.existsSync(bundled)) {
        try { fs.chmodSync(bundled, 0o755); } catch (_) {}
        return bundled;
    }

    // 2. Check previously downloaded binary in global storage
    const storageDir  = context.globalStoragePath;
    const cachedPath  = path.join(storageDir, binName);
    if (fs.existsSync(cachedPath)) return cachedPath;

    // 3. Download from latest GitHub Release
    const conf = vscode.workspace.getConfiguration('ternlang');
    if (conf.get('suppressLspWarning')) return null;

    const downloadUrl =
        `https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/releases/latest/download/${binName}`;

    return vscode.window.withProgress(
        { location: vscode.ProgressLocation.Notification, title: 'Ternlang: Downloading language server…', cancellable: false },
        async () => {
            try {
                fs.mkdirSync(storageDir, { recursive: true });
                await downloadFile(downloadUrl, cachedPath);
                try { fs.chmodSync(cachedPath, 0o755); } catch (_) {}
                return cachedPath;
            } catch (err) {
                vscode.window.showWarningMessage(
                    `ternlang-lsp download failed: ${err.message}. Hover & diagnostics disabled.`,
                    "Don't show again"
                ).then(sel => {
                    if (sel) conf.update('suppressLspWarning', true, true);
                });
                return null;
            }
        }
    );
}

async function startLocalLsp(context) {
    const lspBin = await ensureLspBinary(context);
    if (!lspBin) return;

    const serverOptions = { command: lspBin, transport: TransportKind.stdio };
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'ternlang' }],
        synchronize: { fileEvents: vscode.workspace.createFileSystemWatcher('**/*.tern') }
    };
    client = new LanguageClient('ternlang-lsp', 'Ternlang Language Server', serverOptions, clientOptions);
    client.start();
}

// ── Tier-gated stubs ─────────────────────────────────────────────────────────

function makeUpgradeCommand(requiredTier, featureName) {
    return function () {
        const conf    = vscode.workspace.getConfiguration('ternlang');
        const current = getTier(conf.get('apiKey') || '');
        if (current >= requiredTier) {
            vscode.window.showInformationMessage(`${featureName} — coming in the next release.`);
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

// ── Activate ─────────────────────────────────────────────────────────────────

function activate(context) {
    // Status bar
    statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    statusBar.command = 'ternlang.showStatus';
    context.subscriptions.push(statusBar);

    const conf  = vscode.workspace.getConfiguration('ternlang');
    const tier  = getTier(conf.get('apiKey') || '');
    updateStatusBar(tier > 1 ? `[${tierLabel(tier)}]` : '', `Ternlang — ${tierLabel(tier)} tier`);
    statusBar.show();

    // Clear decorations when switching files
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(editor => {
            if (editor) clearDecorations(editor);
        })
    );

    // Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('ternlang.run', runTernFile),

        vscode.commands.registerCommand('ternlang.showStatus', () => {
            const k   = conf.get('apiKey') || '';
            const t   = getTier(k);
            const msg = t === 1
                ? 'Tier 1 — Free. Set ternlang.apiKey to unlock Pro features (inline trit hints, cloud diagnostics).'
                : `Tier ${t} — ${tierLabel(t)}. API key active.`;
            vscode.window.showInformationMessage(msg, 'Open Settings').then(sel => {
                if (sel) vscode.commands.executeCommand('workbench.action.openSettings', 'ternlang');
            });
        }),

        // Tier 2
        vscode.commands.registerCommand('ternlang.inlineTritHints',
            makeUpgradeCommand(2, 'Inline trit value hints')),
        vscode.commands.registerCommand('ternlang.cloudDiagnostics',
            makeUpgradeCommand(2, 'Cloud diagnostics')),

        // Tier 3
        vscode.commands.registerCommand('ternlang.debugBetVm',
            makeUpgradeCommand(3, 'BET VM step debugger')),
        vscode.commands.registerCommand('ternlang.viewTensor',
            makeUpgradeCommand(3, 'Tensor visualizer')),
        vscode.commands.registerCommand('ternlang.sparseskipCoverage',
            makeUpgradeCommand(3, '@sparseskip coverage overlay')),

        // Tier 4
        vscode.commands.registerCommand('ternlang.clusterPanel',
            makeUpgradeCommand(4, 'Remote cluster panel')),
        vscode.commands.registerCommand('ternlang.agentMonitor',
            makeUpgradeCommand(4, 'Agent monitor dashboard'))
    );

    // LSP — async, platform-aware
    startLocalLsp(context);
}

function deactivate() {
    if (client) return client.stop();
}

module.exports = { activate, deactivate };
