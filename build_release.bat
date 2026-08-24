@echo off
setlocal
cd /d "%~dp0"
call "%~dp0scripts\build_release_windows.bat" %*
