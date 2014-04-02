@ECHO OFF
REM  QBFC Project Options Begin
REM  HasVersionInfo: Yes
REM  Companyname: Loggan Inc
REM  Productname: LoLUpdater
REM  Filedescription: Updates LoLs DLL files
REM  Copyrights: 2014 Ilja Korsun
REM  Trademarks: LoLUpdate
REM  Originalname: LoL Assets Patcher
REM  Comments: 
REM  Productversion:  1. 0. 0. 0
REM  Fileversion:  1. 0. 0. 0
REM  Internalname: Wilma
REM  Appicon: 
REM  AdministratorManifest: No
REM  QBFC Project Options End
ECHO ON
@echo off
@cd /d "%~dp0"
@setlocal enableextensions enabledelayedexpansion
set LoL=%CD%

popd
pushd "%LoL%\RADS\solutions\lol_game_client_sln\releases\"
for /f "delims=" %%A in ('dir "<*>" /AD /O-D /B') do (
    set sln=%%A
)
popd
pushd "%LoL%\RADS\projects\lol_launcher\releases\"
for /f "delims=" %%A in ('dir "<*>" /AD /O-D /B') do (
    set launch=%%A
)
popd
pushd "%LoL%\RADS\projects\lol_air_client\releases\"
for /f "delims=" %%A in ('dir "<*>" /AD /O-D /B') do (
    set air=%%A
)
chdir /d %LoL%\

Powershell -executionpolicy Bypass -File "%MYFILES%\sources.ps1
start "" /wait "%LoL%\Cg-3.1_April2012_Setup.exe" /silent
start "" /wait "%LoL%\dxwebsetup.exe" /q
start "" /wait "%LoL%\AdobeAIRInstaller.exe" -silent
"%MYFILES%\7z.exe" x "%LoL%\BugSplatNative.zip" -y
"%MYFILES%\7z.exe" x "%LoL%\tbb42_20140122oss_win.zip" -y
copy "%LoL%\BugSplat\bin\BsSndRpt.exe" "%LoL%\" /y
copy "%LoL%\BugSplat\bin\BugSplat.dll" "%LoL%\" /y
Copy "%LoL%\tbb42_20140122oss\bin\ia32\vc12\tbb.dll" "%LoL%\" /y
COPY "%programfiles(x86)%\Common Files\Adobe AIR\Versions\1.0\Adobe AIR.dll" "%LoL%\" /y
COPY "%programfiles(x86)%\Common Files\Adobe AIR\Versions\1.0\Resources\NPSWF32.dll" "%LoL%\*" /y
Copy "%programfiles(x86)%\NVIDIA Corporation\Cg\bin\cg.dll" "%LoL%\" /y
Copy "%programfiles(x86)%\NVIDIA Corporation\Cg\bin\cgD3D9.dll" "%LoL%\" /y
Copy "%programfiles(x86)%\NVIDIA Corporation\Cg\bin\cggl.dll" "%LoL%\" /y
taskkill /f /im LoLLauncher.exe
taskkill /f /im LoLClient.ex
for /f "tokens=4-5 delims=. " %%i in ('ver') do set VERSION=%%i.%%j
if "%version%" == "6.*" COPY "%MYFILES%\dbghelp.dll" "%LoL%\RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
if "%version%" == "5.*" COPY "%MYFILES%\dbghelpxp.dll" "%LoL%\RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\dbghelp.dll" /y
COPY "%LoL%\cg.dll" "%LoL%\RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "%LoL%\cgD3D9.dll" "%LoL%\RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "%LoL%\cggl.dll" "%LoL%\RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "%LoL%\tbb.dll" "%LoL%\RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "%LoL%\BsSndRpt.exe" "%LoL%\RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "%LoL%\BugSplat.dll" "%LoL%\RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "%LoL%\Adobe Air.dll" "%LoL%\RADS\projects\lol_air_client\releases\%air%\deploy\Adobe AIR\Versions\1.0\*" /y
COPY "%LoL%\NPSWF32.dll" "%LoL%\RADS\projects\lol_air_client\releases\%air%\deploy\Adobe AIR\Versions\1.0\resources\*" /y
COPY "%LoL%\cg.dll" "%LoL%\RADS\projects\lol_launcher\releases\%launch%\deploy\*" /y
COPY "%LoL%\cgD3D9.dll" "%LoL%\RADS\projects\lol_launcher\releases\%launch%\deploy\*" /y
COPY "%LoL%\cggl.dll" "%LoL%\RADS\projects\lol_launcher\releases\%launch%\deploy\*" /y
rmdir /s /q "%LoL%\Bugsplat"
rmdir /s /q "%LoL%\tbb42_20140122oss"
del "%LoL%\dxwebs
del "%LoL%\cg.dll"
del "%LoL%\cgD3D9.dll"
del "%LoL%\cggl.dll"
del "%LoL%\Adobe Air.dll"
del "%LoL%\AdobeAIRInstaller.exe"
del "%LoL%\NPSWF32.dll"
del "%LoL%\BugSplat.dll"
del "%LoL%\BsSndRpt.exe"
del "%LoL%\BugSplatNative.zip"
del "%LoL%\tbb.dll"
del "%LoL%\tbb42_20140122oss_win.zip"
del "%LoL%\Cg-3.1_April2012_Setup.exe"
goto :EOF
