@echo off
@cd /d "%~dp0"
@setlocal enableextensions enabledelayedexpansion
set LoL=%CD%

chdir /d %LoL%\
Powershell -executionpolicy Bypass -File "sources.ps1"
pause