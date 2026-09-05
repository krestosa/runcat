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

    Write-Host "Built: $PSScriptRoot\target\release\catcpu.exe"
    Write-Host 'Left-click the tray cat for Settings; right-click for quick controls.'
}
finally {
    Pop-Location
}
