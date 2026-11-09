param([switch]$Internal, [switch]$SkipBuild)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
if (!$Internal) { throw 'Public release is not approved yet. Use -Internal for a local review build; see docs/release-readiness.md.' }
if (!$SkipBuild) {
    & cargo build --release --locked --manifest-path (Join-Path $projectRoot 'Cargo.toml')
    if ($LASTEXITCODE -ne 0) { throw 'Release build failed' }
}
$executable = Join-Path $projectRoot 'target/release/jikanda.exe'
if (!(Test-Path -LiteralPath $executable)) { throw 'Release executable missing' }
$packageRoot = Join-Path $projectRoot 'artifacts/packages'
New-Item -ItemType Directory -Force -Path $packageRoot | Out-Null
$name = 'Jikanda-0.1.0-windows-x64-internal-' + (Get-Date -Format 'yyyyMMdd-HHmmss')
$staging = Join-Path $packageRoot $name
if (Test-Path -LiteralPath $staging) { throw 'Staging directory already exists; run again with a new timestamp' }
New-Item -ItemType Directory -Path $staging | Out-Null
Copy-Item -LiteralPath $executable -Destination (Join-Path $staging 'jikanda.exe')
foreach ($file in @('README.md','CONTRIBUTING.md','CHANGELOG.md','LICENSING.md','assets/fonts/OFL-PlusJakartaSans.txt','scripts/install-windows.ps1')) {
    Copy-Item -LiteralPath (Join-Path $projectRoot $file) -Destination $staging
}
Copy-Item -LiteralPath (Join-Path $projectRoot 'docs') -Destination (Join-Path $staging 'docs') -Recurse
if (Test-Path -LiteralPath (Join-Path $projectRoot 'LICENSE')) { Copy-Item -LiteralPath (Join-Path $projectRoot 'LICENSE') -Destination $staging }
'PRE-RELEASE REVIEW BUILD. Jikanda code: GPL-3.0-only; see LICENSE and LICENSING.md. This quality label does not restrict GPL rights. Production branding, platform validation and complete Corresponding Source/dependency audit remain release tasks. No user data is included.' | Set-Content -Encoding UTF8 -LiteralPath (Join-Path $staging 'INTERNAL-BUILD.txt')
$sourceStaging = Join-Path $staging 'source/jikanda'
New-Item -ItemType Directory -Force -Path $sourceStaging | Out-Null
foreach ($folder in @('src','ui','assets','docs','scripts','.github')) {
    Copy-Item -LiteralPath (Join-Path $projectRoot $folder) -Destination (Join-Path $sourceStaging $folder) -Recurse
}
foreach ($file in @('Cargo.toml','Cargo.lock','build.rs','LICENSE','LICENSING.md','README.md','CONTRIBUTING.md','CHANGELOG.md','AGENTS.md','.gitignore','Jikanda Brand Identity Sheet.png')) {
    Copy-Item -LiteralPath (Join-Path $projectRoot $file) -Destination (Join-Path $sourceStaging $file)
}
& (Join-Path $PSScriptRoot 'third-party-notices.ps1') -OutputDirectory $staging
if (!$?) { throw 'Notice generation failed' }
# Some Cargo registry license files use Unix epoch timestamps, outside ZIP's
# 1980-2107 range. Normalize only copies in this fresh staging directory.
foreach ($file in (Get-ChildItem -LiteralPath $staging -File -Recurse)) {
    if ($file.LastWriteTime.Year -lt 1980 -or $file.LastWriteTime.Year -gt 2106) {
        $file.LastWriteTimeUtc = [datetime]::SpecifyKind([datetime]'2000-01-01', [DateTimeKind]::Utc)
    }
}
$zip = Join-Path $packageRoot ($name + '.zip')
Compress-Archive -Path (Join-Path $staging '*') -DestinationPath $zip
$hash = Get-FileHash -LiteralPath $zip -Algorithm SHA256
("{0}  {1}" -f $hash.Hash, (Split-Path -Leaf $zip)) | Set-Content -Encoding ASCII -LiteralPath ($zip + '.sha256')
Write-Output $zip
