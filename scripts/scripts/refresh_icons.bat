@echo off
REM ================================================================
REM  Force Windows Icon Cache Refresh
REM  This script forcibly rebuilds the Windows icon cache by
REM  killing Explorer, deleting the cache files, and restarting it.
REM  Run as Administrator for best results.
REM ================================================================

echo.
echo  ========================================
echo   Rebuilding Windows Icon Cache...
echo  ========================================
echo.

REM 1. Suppress prompt messages
setlocal

REM 2. Gracefully exit Explorer (or forcefully if it hangs)
echo [1/3] Stopping Windows Explorer...
taskkill /f /im explorer.exe >nul 2>&1

REM Give it a moment to release file locks
timeout /t 2 /nobreak >nul

REM 3. Delete the legacy icon cache
echo [2/3] Deleting icon caches...
if exist "%localappdata%\IconCache.db" (
    del /a "%localappdata%\IconCache.db" >nul 2>&1
)

REM 4. Delete the modern Windows 10/11 icon caches
if exist "%localappdata%\Microsoft\Windows\Explorer\iconcache_*" (
    del /a /f /q "%localappdata%\Microsoft\Windows\Explorer\iconcache_*" >nul 2>&1
)

REM 5. Restart Explorer
echo [3/3] Restarting Windows Explorer...
start explorer.exe

echo.
echo [DONE] Icon cache rebuilt! Your .txs icons should now appear.
echo.
endlocal
pause
