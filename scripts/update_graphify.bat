@echo off
echo [Graphify] Running Graphify Knowledge Graph Update...
python "%~dp0tools\update_graphify.py"
if %ERRORLEVEL% neq 0 (
    echo [Graphify] ERROR: Graphify update failed.
    exit /b %ERRORLEVEL%
)
echo [Graphify] Update completed successfully.
