@echo off
@cd /d "%~dp0"
@setlocal enableextensions enabledelayedexpansion
set LoL=%CD%

popd
pushd "RADS\solutions\lol_game_client_sln\releases\"
for /f "delims=" %%A in ('dir "<*>" /AD /O-D /B') do (
    set sln=%%A
)
popd
pushd "RADS\projects\lol_launcher\releases\"
for /f "delims=" %%A in ('dir "<*>" /AD /O-D /B') do (
    set launch=%%A
)
popd
pushd "RADS\projects\lol_air_client\releases\"
for /f "delims=" %%A in ('dir "<*>" /AD /O-D /B') do (
    set air=%%A
)
chdir /d %LoL%\
Powershell -executionpolicy Bypass -File -Verb RunAs "sources.ps1"
for /f "tokens=4-5 delims=. " %%i in ('ver') do set VERSION=%%i.%%j
if "%version%" == "6.*" COPY "dbghelp.dll" "RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
if "%version%" == "5.*" COPY "dbghelpxp.dll" "RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\dbghelp.dll" /y
COPY "cg.dll" "RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "cgD3D9.dll" "RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "cggl.dll" "RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "tbb.dll" "RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "BsSndRpt.exe" "RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "BugSplat.dll" "RADS\solutions\lol_game_client_sln\releases\%sln%\deploy\*" /y
COPY "Adobe Air.dll" "RADS\projects\lol_air_client\releases\%air%\deploy\Adobe AIR\Versions\1.0\*" /y
COPY "NPSWF32.dll" "RADS\projects\lol_air_client\releases\%air%\deploy\Adobe AIR\Versions\1.0\resources\*" /y
COPY "cg.dll" "RADS\projects\lol_launcher\releases\%launch%\deploy\*" /y
COPY "cgD3D9.dll" "RADS\projects\lol_launcher\releases\%launch%\deploy\*" /y
COPY "cggl.dll" "RADS\projects\lol_launcher\releases\%launch%\deploy\*" /y