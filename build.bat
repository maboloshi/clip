@echo off
echo ========================================
echo Building Clip - Text to Clipboard Tool
echo ========================================
echo.

REM 清理之前的构建
echo [1/3] Cleaning previous builds...
cargo clean
echo.

REM 编译两个版本
echo [2/3] Building both versions...
echo.
echo Building console version (clip.exe)...
cargo build --bin clip --release
if %ERRORLEVEL% NEQ 0 (
    echo Error building console version!
    exit /b 1
)
echo   [OK] clip.exe
echo.

echo Building GUI version (clipw.exe)...
cargo build --bin clipw --release
if %ERRORLEVEL% NEQ 0 (
    echo Error building GUI version!
    exit /b 1
)
echo   [OK] clipw.exe
echo.

REM 显示文件大小
echo [3/3] Build complete!
echo.
echo ========================================
echo Output files:
echo ========================================
dir target\release\clip.exe | findstr "clip.exe"
dir target\release\clipw.exe | findstr "clipw.exe"
echo.

REM 可选：UPX 压缩
echo.
set /p compress="Compress with UPX? (y/N): "
if /i "%compress%"=="y" (
    echo.
    echo Compressing with UPX...
    upx --best --lzma target\release\clip.exe
    upx --best --lzma target\release\clipw.exe
    echo.
    echo Compressed sizes:
    dir target\release\clip.exe | findstr "clip.exe"
    dir target\release\clipw.exe | findstr "clipw.exe"
)

echo.
echo ========================================
echo Done! Files are in target\release\
echo   clip.exe  - Console version
echo   clipw.exe - GUI version (no window)
echo ========================================
pause