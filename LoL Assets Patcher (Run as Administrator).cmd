@echo off
@cd /d "%~dp0"
Powershell -executionpolicy Bypass -File "sources.ps1"
pause