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

Write-Host "[2/5] Downloading model weights (142 MB)..."
Download "$Release/bible_ternary_v2.0.0.safetensors" "$Dir\models\bible_ternary_v2.0.0.safetensors" 100000000

Write-Host "[3/5] Downloading model config..."
Download "$Release/bible_ternary_v2.0.0.config.json" "$Dir\models\bible_ternary_v2.0.0.config.json" 50
Download "$Release/bible_ternary_v2.0.0.meta"        "$Dir\models\bible_ternary_v2.0.0.meta" 1

Write-Host "[4/5] Downloading vocabulary + eval sample..."
Download "$Release/vocab.json"      "$Dir\data\vocab.json" 10000
Download "$Release/eval_sample.txt" "$Dir\eval_sample.txt" 10000

Write-Host "[5/5] Running benchmark suite..."
Write-Host ""

Set-Location $Dir
.\moe-test.exe --bench --csv albert_bench_results.csv

Write-Host ""
Write-Host "Results saved to .\$Dir\albert_bench_results.csv"
Write-Host "To run again:    cd $Dir; .\moe-test.exe --bench"
Write-Host "Interactive TUI: cd $Dir; .\moe-test.exe"
