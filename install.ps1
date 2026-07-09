# Installer for Terminal Solitaire (Windows).
# Usage: irm https://raw.githubusercontent.com/marcus-wrrn/Terminal-Solitaire/main/install.ps1 | iex
#
# Environment variables:
#   SOLITAIRE_INSTALL_DIR  install location (default: $HOME\.local\bin)
#   SOLITAIRE_VERSION      tag to install, e.g. v0.1.5 (default: latest release)

$ErrorActionPreference = "Stop"

$Repo = "marcus-wrrn/Terminal-Solitaire"
$Bin = "solitaire.exe"
$InstallDir = if ($env:SOLITAIRE_INSTALL_DIR) { $env:SOLITAIRE_INSTALL_DIR } else { Join-Path $HOME ".local\bin" }

function Fail($msg) {
    Write-Error $msg
    exit 1
}

function Get-Target {
    switch ($env:PROCESSOR_ARCHITECTURE) {
        "AMD64" { $arch = "x86_64" }
        "ARM64" { $arch = "aarch64" }
        default { Fail "unsupported architecture: $($env:PROCESSOR_ARCHITECTURE)" }
    }
    return "$arch-pc-windows-msvc"
}

function Resolve-Version {
    if ($env:SOLITAIRE_VERSION) {
        return $env:SOLITAIRE_VERSION
    }
    $response = Invoke-WebRequest -Uri "https://github.com/$Repo/releases/latest" -MaximumRedirection 0 -SkipHttpErrorCheck
    $location = $response.Headers.Location
    if (-not $location) {
        Fail "could not determine the latest release; set SOLITAIRE_VERSION and retry"
    }
    return ($location -split "/tag/")[-1]
}

function Main {
    $target = Get-Target
    $version = Resolve-Version

    $archive = "solitaire-$version-$target.zip"
    $url = "https://github.com/$Repo/releases/download/$version/$archive"

    $tmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
    New-Item -ItemType Directory -Path $tmpDir | Out-Null
    try {
        Write-Host "Downloading $Bin $version ($target)..."
        $archivePath = Join-Path $tmpDir $archive
        Invoke-WebRequest -Uri $url -OutFile $archivePath

        Expand-Archive -Path $archivePath -DestinationPath $tmpDir -Force

        $binPath = Join-Path $tmpDir $Bin
        if (-not (Test-Path $binPath)) {
            Fail "archive did not contain the '$Bin' binary"
        }

        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        Copy-Item -Path $binPath -Destination (Join-Path $InstallDir $Bin) -Force

        Write-Host "Installed $Bin to $(Join-Path $InstallDir $Bin)"

        $pathEntries = $env:Path -split ";"
        if ($pathEntries -notcontains $InstallDir) {
            Write-Host ""
            Write-Host "warning: $InstallDir is not on your PATH."
            Write-Host "Add it with:"
            Write-Host "  [Environment]::SetEnvironmentVariable('Path', `"`$env:Path;$InstallDir`", 'User')"
        }
    }
    finally {
        Remove-Item -Path $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

Main
