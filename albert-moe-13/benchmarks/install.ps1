# Albert MoE-13 — TIS Benchmark Installer (Windows)
# Usage: irm https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/albert-moe-13/benchmarks/install.ps1 | iex
#
# Requires PowerShell 5.1+ (built into Windows 10/11) or PowerShell 7+
# No admin rights needed.

$ErrorActionPreference = 'Stop'

$Release = "https://github.com/eriirfos-eng/ternary-intelligence-stack/releases/download/bench-v2.0.0"
$Dir     = "albert-bench"

Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════════╗"
Write-Host "║         Albert MoE-13 — TIS Benchmark Suite v2.0.0        ║"
Write-Host "║         Ternary Intelligence Stack / RFI-IRFOS             ║"
Write-Host "╚════════════════════════════════════════════════════════════╝"
Write-Host ""
Write-Host "Platform : Windows x86_64"
Write-Host "Dest     : .\$Dir\"
Write-Host ""

New-Item -ItemType Directory -Force -Path "$Dir\models" | Out-Null
New-Item -ItemType Directory -Force -Path "$Dir\data"   | Out-Null

# SHA256 checksums for bench-v2.0.0 artifacts
$SHA256 = @{
    "moe-test-windows-x86_64.exe"          = "683888e689b0710e0faae604b71275dd91ee18a944a43103b4b712cb9ba390ba"
    "bible_ternary_v2.0.0.safetensors"     = "3dde0a6894573a3a4f65473dd0bce438dbe0a0c3e85f70799acf5f72498246c5"
    "bible_ternary_v2.0.0.config.json"     = "ca821d4b7443d832c096fb75acec05e9e8829f3a0b188799686f889a2470d271"
    "bible_ternary_v2.0.0.meta"            = "37b73510175057c633ebe4beb0a34917fa2a0696432db43a4eeb2c3ff83a4c3b"
    "vocab.json"                           = "ff4149d3e7b1b25745713e9affe9cdbd2767802e10f2af6486b900f70d9e72d6"
    "eval_sample.txt"                      = "2e29a40f9139a62e06b02126ece490b72163d9e979536c2c242119c565710f20"
}

function Verify-SHA256($path) {
    $filename = [System.IO.Path]::GetFileName($path)
    if (-not $SHA256.ContainsKey($filename)) {
        Write-Host "  (no checksum entry for $filename — skipping)" -ForegroundColor Yellow
        return
    }
    $expected = $SHA256[$filename]
    $actual   = (Get-FileHash $path -Algorithm SHA256).Hash.ToLower()
    if ($actual -ne $expected) {
        Write-Host "ERROR: checksum mismatch for $filename" -ForegroundColor Red
        Write-Host "  expected: $expected"
        Write-Host "  actual:   $actual"
        Write-Host "The file may be corrupted or tampered with. Delete and retry." -ForegroundColor Red
        exit 1
    }
    Write-Host "  ok (sha256 verified)"
}

function Download($url, $dest, $minBytes = 0) {
    $filename = [System.IO.Path]::GetFileName($dest)
    $fullDest = Join-Path (Get-Location) $dest
    Write-Host "  <- $filename" -NoNewline

    try {
        $wc = New-Object System.Net.WebClient
        $wc.DownloadFile($url, $fullDest)
    } catch {
        Write-Host " FAILED"
        Write-Host "Error downloading ${filename}: $_" -ForegroundColor Red
        exit 1
    }

    $size = (Get-Item $fullDest).Length
    Write-Host "  ($([math]::Round($size/1KB, 1)) KB)"

    if ($minBytes -gt 0 -and $size -lt $minBytes) {
        Write-Host "Error: $filename too small ($size bytes) — download may have been blocked or truncated." -ForegroundColor Red
        Write-Host "Try opening this URL in your browser to download manually:" -ForegroundColor Yellow
        Write-Host "  $url" -ForegroundColor Yellow
        exit 1
    }
}

Write-Host "[1/5] Downloading binary (Windows x86_64)..."
Download "$Release/moe-test-windows-x86_64.exe" "$Dir\moe-test.exe" 1000000
Verify-SHA256 "$Dir\moe-test.exe"

Write-Host "[2/5] Downloading model weights (142 MB)..."
Download "$Release/bible_ternary_v2.0.0.safetensors" "$Dir\models\bible_ternary_v2.0.0.safetensors" 100000000
Verify-SHA256 "$Dir\models\bible_ternary_v2.0.0.safetensors"

Write-Host "[3/5] Downloading model config..."
Download "$Release/bible_ternary_v2.0.0.config.json" "$Dir\models\bible_ternary_v2.0.0.config.json" 50
Verify-SHA256 "$Dir\models\bible_ternary_v2.0.0.config.json"
Download "$Release/bible_ternary_v2.0.0.meta"        "$Dir\models\bible_ternary_v2.0.0.meta" 1
Verify-SHA256 "$Dir\models\bible_ternary_v2.0.0.meta"

Write-Host "[4/5] Downloading vocabulary + eval sample..."
Download "$Release/vocab.json"      "$Dir\data\vocab.json" 10000
Verify-SHA256 "$Dir\data\vocab.json"
Download "$Release/eval_sample.txt" "$Dir\eval_sample.txt" 10000
Verify-SHA256 "$Dir\eval_sample.txt"

Write-Host "[5/5] Running benchmark suite..."
Write-Host ""

Set-Location $Dir
.\moe-test.exe --bench --csv albert_bench_results.csv

Write-Host ""
Write-Host "Results saved to .\$Dir\albert_bench_results.csv"
Write-Host "To run again:    cd $Dir; .\moe-test.exe --bench"
Write-Host "Interactive TUI: cd $Dir; .\moe-test.exe"
