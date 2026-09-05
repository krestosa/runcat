$ErrorActionPreference = 'Stop'

$assetDir = Join-Path $PSScriptRoot 'assets'
New-Item -ItemType Directory -Force -Path $assetDir | Out-Null

$runCommit = '03b6e2b288c2df5df2433398f5547857bb4d0e2f'
$runBase = "https://raw.githubusercontent.com/runcat-dev/RunCat365/$runCommit/RunCat365/resources/runners/cat"
$neoCommit = 'b3b1543049ea0a051ecb78654a45f144724ea737'
$neoSleepUrl = "https://raw.githubusercontent.com/runcat-dev/RunCatNeo/$neoCommit/LocalPackage/Sources/UserInterface/Resources/Media.xcassets/sleeping-cat.imageset/sleeping-cat.png"

$assets = @(
    @{ Name = 'cat_0.png'; Url = "$runBase/cat_0.png"; Sha = 'c94ffebf337d0040892d7f271275607dd5740fa0' },
    @{ Name = 'cat_1.png'; Url = "$runBase/cat_1.png"; Sha = '3bb178f4ce21e44c8e66540972f1205ccd040900' },
    @{ Name = 'cat_2.png'; Url = "$runBase/cat_2.png"; Sha = 'fd48ca83f21011390f12e1130e5ca54340af11e7' },
    @{ Name = 'cat_3.png'; Url = "$runBase/cat_3.png"; Sha = '128fb5d284b5c4d3a79078f4b3c819b51c7466c9' },
    @{ Name = 'cat_4.png'; Url = "$runBase/cat_4.png"; Sha = 'cccc51b4ea2ad38955bff3fc710fddd86682db86' },
    @{ Name = 'sleeping-cat.png'; Url = $neoSleepUrl; Sha = '3b336290aa041f332bf42c69bbb5899991ced99b' }
)

function Get-GitBlobSha1([byte[]] $Bytes) {
    $header = [System.Text.Encoding]::ASCII.GetBytes("blob $($Bytes.Length)`0")
    $payload = New-Object byte[] ($header.Length + $Bytes.Length)
    [Array]::Copy($header, 0, $payload, 0, $header.Length)
    [Array]::Copy($Bytes, 0, $payload, $header.Length, $Bytes.Length)

    $sha1 = [System.Security.Cryptography.SHA1]::Create()
    try {
        $hash = $sha1.ComputeHash($payload)
    }
    finally {
        $sha1.Dispose()
    }

    return -join ($hash | ForEach-Object { $_.ToString('x2') })
}

function Test-VerifiedAsset([string] $Path, [string] $ExpectedSha) {
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        return $false
    }

    try {
        $bytes = [System.IO.File]::ReadAllBytes($Path)
        if ($bytes.Length -lt 100) {
            return $false
        }
        if ($bytes[0] -ne 0x89 -or $bytes[1] -ne 0x50 -or $bytes[2] -ne 0x4E -or $bytes[3] -ne 0x47) {
            return $false
        }
        return (Get-GitBlobSha1 $bytes) -eq $ExpectedSha
    }
    catch {
        return $false
    }
}

foreach ($asset in $assets) {
    $target = Join-Path $assetDir $asset.Name

    if (Test-VerifiedAsset $target $asset.Sha) {
        continue
    }

    $download = "$target.download"
    Remove-Item -Force -ErrorAction SilentlyContinue $download
    try {
        Invoke-WebRequest -UseBasicParsing -Uri $asset.Url -OutFile $download
        if (-not (Test-VerifiedAsset $download $asset.Sha)) {
            throw "Asset integrity check failed: $($asset.Name)"
        }
        Move-Item -Force $download $target
    }
    finally {
        Remove-Item -Force -ErrorAction SilentlyContinue $download
    }
}

Write-Host 'Cat assets verified.'
