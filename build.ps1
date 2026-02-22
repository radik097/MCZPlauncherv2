#!/usr/bin/env pwsh
<#
.SYNOPSIS
Build script for MCZ Launcher - Creates a portable Windows 11 Minecraft Launcher

.DESCRIPTION
Compiles the launcher and creates a distributable package

.EXAMPLE
.\build.ps1 -Release
.\build.ps1 -Clean
#>

param(
    [switch]$Release = $false,
    [switch]$Clean = $false,
    [switch]$Package = $false
)

$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$BuildDir = Join-Path $ProjectRoot "build"
$DistDir = Join-Path $ProjectRoot "dist"
$Target = if ($Release) { "release" } else { "debug" }
$BuildProfile = if ($Release) { "--release" } else { "" }

Write-Host "=== MCZ Launcher Build Script ===" -ForegroundColor Cyan
Write-Host "Project: $ProjectRoot" -ForegroundColor Gray

# Clean
if ($Clean -or -not (Test-Path $BuildDir)) {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Yellow
    if (Test-Path $BuildDir) { Remove-Item $BuildDir -Recurse -Force }
    if (Test-Path "..\target") { 
        Write-Host "Removing cargo target directory (this may take a moment)..." -ForegroundColor Gray
        cargo clean
    }
}

# Create build directory
New-Item -ItemType Directory -Path $BuildDir -Force | Out-Null

# Build
Write-Host "Building MCZ Launcher ($Target mode)..." -ForegroundColor Yellow
$buildCmd = "cargo build $BuildProfile"
Write-Host "Command: $buildCmd" -ForegroundColor Gray

Invoke-Expression $buildCmd
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Copy executable
$ExeName = "MCZPlauncher.exe"
$SourceExe = Join-Path $ProjectRoot "target" $Target $ExeName
$DestExe = Join-Path $BuildDir $ExeName

if (Test-Path $SourceExe) {
    Write-Host "Copying executable to build directory..." -ForegroundColor Yellow
    Copy-Item $SourceExe $DestExe -Force
    Write-Host "✓ Executable: $DestExe" -ForegroundColor Green
} else {
    Write-Host "ERROR: Executable not found at $SourceExe" -ForegroundColor Red
    exit 1
}

# Copy resources
Write-Host "Copying resources..." -ForegroundColor Yellow
Copy-Item (Join-Path $ProjectRoot "README.md") (Join-Path $BuildDir "README.md") -Force
Copy-Item (Join-Path $ProjectRoot "modpacks.json") (Join-Path $BuildDir "modpacks.json") -Force
Copy-Item (Join-Path $ProjectRoot "Cargo.toml") (Join-Path $BuildDir "Cargo.toml") -Force

# Create package
if ($Package) {
    Write-Host "Creating distribution package..." -ForegroundColor Yellow
    
    if (-not (Test-Path $DistDir)) {
        New-Item -ItemType Directory -Path $DistDir -Force | Out-Null
    }
    
    $PackageName = "MCZPlauncher-$Target.zip"
    $PackagePath = Join-Path $DistDir $PackageName
    
    if (Test-Path $PackagePath) {
        Remove-Item $PackagePath -Force
    }
    
    # Create zip using PowerShell
    Compress-Archive -Path $BuildDir -DestinationPath $PackagePath -Force
    Write-Host "✓ Package: $PackagePath" -ForegroundColor Green
}

Write-Host "`n=== Build Complete ===" -ForegroundColor Green
Write-Host "Executable: $DestExe" -ForegroundColor Cyan
Write-Host "Size: $((Get-Item $DestExe).Length / 1MB) MB" -ForegroundColor Gray

if ($Release) {
    Write-Host "`nOptimizations enabled:" -ForegroundColor Green
    Write-Host "- Link-time optimization (LTO)" -ForegroundColor Gray
    Write-Host "- Single codegen unit" -ForegroundColor Gray
    Write-Host "- Maximum optimization level" -ForegroundColor Gray
}

Write-Host "`nRun the launcher:" -ForegroundColor Yellow
Write-Host "  $DestExe`n" -ForegroundColor Cyan
