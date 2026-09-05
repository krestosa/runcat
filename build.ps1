$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw 'Cargo was not found. Install the Rust MSVC toolchain first.'
}

$required = @(
    (0..4 | ForEach-Object { Join-Path $PSScriptRoot "assets\cat_$_.png" })
    (Join-Path $PSScriptRoot 'assets\sleeping-cat.png')
)
if ($required.Where({ -not (Test-Path $_) }).Count -gt 0) {
    & (Join-Path $PSScriptRoot 'import-assets.ps1')
}

Push-Location $PSScriptRoot
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build --release failed with exit code $LASTEXITCODE"
    }
    Write-Host "Built: $PSScriptRoot\target\release\catcpu.exe"
    Write-Host 'Right-click the tray cat and choose Settings... for continuous controls.'
}
finally {
    Pop-Location
}
