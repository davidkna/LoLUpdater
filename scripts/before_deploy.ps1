# This script takes care of packaging the build artifacts that will go in the
# release zipfile

$SRC_DIR = $PWD.Path
$STAGE = [System.Guid]::NewGuid().ToString()

Set-Location $ENV:Temp
New-Item -Type Directory -Name $STAGE
Set-Location $STAGE

$ZIP_CLI = "$SRC_DIR\lolupdater-cli-$($Env:APPVEYOR_REPO_TAG_NAME)-$($Env:TARGET).zip"

# TODO Update this to package the right artifacts
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\lolupdater-cli.exe" '.\'
Copy-Item "$SRC_DIR\README.md" '.\'

7z a "$ZIP_CLI" *

Push-AppveyorArtifact "$ZIP_CLI"

Remove-Item *.* -Force
Set-Location ..
Remove-Item $STAGE

$STAGE = [System.Guid]::NewGuid().ToString()

New-Item -Type Directory -Name $STAGE
Set-Location $STAGE

$ZIP_GUI = "$SRC_DIR\lolupdater-gui-$($Env:APPVEYOR_REPO_TAG_NAME)-$($Env:TARGET).zip"

$LIBUI_DLL = Get-ChildItem -Path "$SRC_DIR\target\$($Env:TARGET)\release" -Filter libui.dll -Recurse -ErrorAction SilentlyContinue -Force | %{$_.FullName}

Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\lolupdater-gui.exe" '.\'
Copy-Item "$LIBUI_DLL" '.\'

7z a "$ZIP_GUI" *

Push-AppveyorArtifact "$ZIP_GUI"

Remove-Item *.* -Force
Set-Location ..
Remove-Item $STAGE
