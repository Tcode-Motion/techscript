@echo off
title TechScript Git Manager
color 0A

:MENU
cls
echo ==========================================
echo        TechScript 2.0 Git Manager
echo ==========================================
echo.
echo [1] Commit and Push
echo [2] Git Status
echo [3] View Branches
echo [4] Switch Branch
echo [5] Pull Latest
echo [6] Fetch
echo [7] Create New Branch
echo [8] Merge Branch into Main
echo [9] View Commit History
echo [10] Open GitHub Repo
echo [0] Exit
echo.

set /p choice=Select Option:

if "%choice%"=="1" goto PUSH
if "%choice%"=="2" goto STATUS
if "%choice%"=="3" goto BRANCHES
if "%choice%"=="4" goto SWITCH
if "%choice%"=="5" goto PULL
if "%choice%"=="6" goto FETCH
if "%choice%"=="7" goto CREATE
if "%choice%"=="8" goto MERGE
if "%choice%"=="9" goto LOG
if "%choice%"=="10" goto OPEN
if "%choice%"=="0" exit

goto MENU

:PUSH
cls
echo.
set /p msg=Commit Message (Leave empty for automatic):
if "%msg%"=="" (
set msg=Auto Commit %date% %time%
)

git add .
git commit -m "%msg%"
git push origin main

echo.
pause
goto MENU

:STATUS
cls
git status
echo.
pause
goto MENU

:BRANCHES
cls
git branch -a
echo.
pause
goto MENU

:SWITCH
cls
git branch -a
echo.
set /p br=Enter branch name:
git checkout %br%
pause
goto MENU

:PULL
cls
git pull origin main
pause
goto MENU

:FETCH
cls
git fetch --all
pause
goto MENU

:CREATE
cls
set /p nb=New branch name:
git checkout -b %nb%
git push -u origin %nb%
pause
goto MENU

:MERGE
cls
git checkout main
git pull
echo.
git branch
echo.
set /p mb=Merge branch:
git merge %mb%
git push origin main
pause
goto MENU

:LOG
cls
git log --oneline --graph --decorate --all
pause
goto MENU

:OPEN
cls
start https://github.com/Tcode-Motion/TechScript-2.0
goto MENU