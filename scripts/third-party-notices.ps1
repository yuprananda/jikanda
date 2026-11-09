param([Parameter(Mandatory=$true)][string]$OutputDirectory)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$metadataText = & cargo metadata --format-version 1 --locked --filter-platform x86_64-pc-windows-msvc --manifest-path (Join-Path $projectRoot 'Cargo.toml')
if ($LASTEXITCODE -ne 0) { throw 'cargo metadata failed' }
$metadata = $metadataText | ConvertFrom-Json
$resolvedIds = @{}
foreach ($node in $metadata.resolve.nodes) { $resolvedIds[$node.id] = $true }
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
$lines = @('Jikanda - third-party dependency inventory', 'Includes runtime and build dependencies resolved for Windows.', 'License expressions and copied upstream texts require final distribution review.', '')
foreach ($package in ($metadata.packages | Where-Object { $resolvedIds.ContainsKey($_.id) -and $_.name -ne 'jikanda' } | Sort-Object name,version)) {
    $lines += "$($package.name) $($package.version) | $($package.license) | $($package.repository)"
    $sourceDirectory = Split-Path -Parent $package.manifest_path
    $destination = Join-Path $OutputDirectory ("licenses/{0}-{1}" -f $package.name,$package.version)
    $licenseFiles = @(Get-ChildItem -LiteralPath $sourceDirectory -File | Where-Object { $_.Name -match '^(LICENSE|COPYING|NOTICE|UNLICENSE)' })
    if ($package.license_file) {
        $explicit = Join-Path $sourceDirectory $package.license_file
        if (Test-Path -LiteralPath $explicit -PathType Leaf) { $licenseFiles += Get-Item -LiteralPath $explicit }
    }
    foreach ($licenseFile in ($licenseFiles | Sort-Object FullName -Unique)) {
        New-Item -ItemType Directory -Force -Path $destination | Out-Null
        Copy-Item -LiteralPath $licenseFile.FullName -Destination (Join-Path $destination $licenseFile.Name)
    }
    foreach ($folder in @('LICENSES', 'licenses')) {
        $sourceLicenses = Join-Path $sourceDirectory $folder
        if (Test-Path -LiteralPath $sourceLicenses -PathType Container) {
            New-Item -ItemType Directory -Force -Path $destination | Out-Null
            Copy-Item -LiteralPath $sourceLicenses -Destination (Join-Path $destination 'upstream-license-directory') -Recurse -Force
            break
        }
    }
}
$lines | Set-Content -Encoding UTF8 -LiteralPath (Join-Path $OutputDirectory 'THIRD-PARTY-NOTICES.txt')
