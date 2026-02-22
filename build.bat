@echo off
REM MCZ Launcher - Quick Build Script
REM Builds the Minecraft launcher and runs it

setlocal enabledelayedexpansion

echo ==========================================
echo MCZ Launcher - Build and Run
echo ==========================================
echo.

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Rust is not installed!
    echo Please install Rust from: https://rustup.rs/
    pause
    exit /b 1
)

REM Determine build mode
set BUILD_MODE=release
set PROFILE_FLAG=--release

if "%1"=="debug" (
    set BUILD_MODE=debug
    set PROFILE_FLAG=
)

echo Building MCZ Launcher (%BUILD_MODE% mode)...
echo.

REM Build the project
cargo build %PROFILE_FLAG%
if errorlevel 1 (
    echo Build failed!
    pause
    exit /b 1
)

REM Find and run the executable
set EXE_PATH=target\%BUILD_MODE%\MCZPlauncher.exe

if not exist "%EXE_PATH%" (
    echo ERROR: Executable not found at %EXE_PATH%
    pause
    exit /b 1
)

echo.
echo Build successful!
echo Launching MCZ Launcher...
echo.

start "" "%EXE_PATH%"
