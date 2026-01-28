@echo off
REM Clean Rust build artifacts

echo Cleaning Rust build artifacts...
cargo clean
if %ERRORLEVEL% NEQ 0 (
    echo Failed to clean build artifacts.
    exit /b %ERRORLEVEL%
)
echo Done.
