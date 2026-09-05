$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw 'Cargo was not found. Install the Rust MSVC toolchain first.'
}

& (Join-Path $PSScriptRoot 'import-assets.ps1')

Push-Location $PSScriptRoot
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build --release failed with exit code $LASTEXITCODE"
    }

    $tray = Join-Path $PSScriptRoot 'target\release\catcpu.exe'
    $settings = Join-Path $PSScriptRoot 'target\release\catcpu-settings.exe'
    if (-not (Test-Path $tray)) {
        throw "Missing build output: $tray"
    }
    if (-not (Test-Path $settings)) {
        throw "Missing WinUI Settings output: $settings"
    }

    Write-Host "Built tray app: $tray"
    Write-Host "Built WinUI Settings: $settings"
    Write-Host 'Keep both executables and the staged Windows App SDK runtime files together in target\release.'
    Write-Host 'Left-click the tray cat for Settings; right-click for quick controls.'
}
finally {
    Pop-Location
}
