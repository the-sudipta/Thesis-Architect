@echo off
cd /d "%~dp0"
powershell -NoProfile -ExecutionPolicy Bypass -File "Generate_Research_Structure.ps1"
pause
