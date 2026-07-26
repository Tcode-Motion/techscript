@echo off
echo [Graphify] Running Graphify Knowledge Graph Update...
python "C:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\tools\scripts\update_graphify.py"
if %ERRORLEVEL% neq 0 (
    echo [Graphify] ERROR: Graphify update failed.
    pause
    exit /b %ERRORLEVEL%
)
echo [Graphify] Update completed successfully.
pause
