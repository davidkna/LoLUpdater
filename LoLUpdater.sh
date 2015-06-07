#!/bin/bash
# LoLUpdater for OS X v1.4.4
# Ported by David Knaack
# LoLUpdater for Windows: https://github.com/Loggan08/LoLUpdater
# License: GPLv3 http://www.gnu.org/licenses/gpl-3.0.html
echo "LoLUpdater for OS X 1.4.4"
echo "------------------------------------------------------------------------"
echo "Password is required to run this script"
sudo -v

# Edit this line if you installed LoL somewhere else
# For example brew-cask symlinks League of Legends.app to ~/Applications/
cd "/Applications/League of Legends.app"

GFRAMEWORKS="/Library/Frameworks"
TEMP="$(mktemp -dt LoLUpdater)"
SLN="Contents/LoL/RADS/solutions/lol_game_client_sln/releases/"
AIR="Contents/LoL/RADS/projects/lol_air_client/releases/"
LAUNCHER="Contents/LoL/RADS/projects/lol_launcher/releases/"
GAMECL="Contents/LoL/RADS/projects/lol_game_client/releases/"
SLN="$SLN$(ls -lrt "$SLN" | tail -1 | awk '{ print $9 }')/deploy/LeagueOfLegends.app/Contents/Frameworks"
AIR="$AIR$(ls -lrt "$AIR" | tail -1 | awk '{ print $9 }')/deploy/Frameworks"
LAUNCHER="$LAUNCHER$(ls -lrt  -t "$LAUNCHER" | tail -1 | awk '{ print $9 }')/deploy/LoLLauncher.app/Contents/Frameworks"
GAMECL="$GAMECL$(ls -lrt "$GAMECL" | tail -1 | awk '{ print $9 }')/deploy/LeagueOfLegends.app/Contents/Frameworks"

function detect() {
  if [ -e "$1" ]
  then
    printf "YES"
  else
    printf "NO"
  fi
}

function backup() {
  echo "Creating Backups (ignore failed overwrite errors)..."
  mkdir "Backups"
  cp -R -n -a "$AIR/Adobe Air.framework" "Backups/"
  cp -R -n -a "$LAUNCHER/Cg.framework" "Backups/"
  cp -R -n -a "$SLN/BugSplat.framework" "Backups/"
  cp -R -n -a "$SLN/../MacOS/libc++.1.dylib" "Backups/"
  cp -R -n -a "$SLN/../MacOS/libc++abi.dylib" "Backups/"
}


function download_air() {
  echo "Downloading the Adobe Air dependency..."
  curl -#o "$TEMP/air.dmg" "http://airdownload.adobe.com/air/mac/download/17.0/AdobeAIR.dmg"
  echo "Mounting Adobe Air disk image..."
  hdiutil attach -nobrowse -quiet "$TEMP/air.dmg"
  echo "Copying files..."
  sudo cp -R "/Volumes/Adobe Air/Adobe Air Installer.app/Contents/Frameworks/Adobe Air.framework" "$TEMP"
  echo "Unmounting the Adobe Air disk Image..."
  hdiutil detach -quiet "/Volumes/Adobe Air/"
}

function download_cg() {
  echo "Downloading the Nvidia Cg dependency..."
  curl -#o "$TEMP/cg.dmg" "http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg"
  echo "Mounting Nvidia Cg disk image..."
  hdiutil attach -nobrowse -quiet "$TEMP/cg.dmg"
  echo "Copying files..."
  mkdir "$TEMP/NVIDIA_Cg"
  cp "/Volumes/cg-3.1.0013/Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz" "$TEMP/NVIDIA_Cg/"
  (cd "$TEMP/NVIDIA_Cg/" && tar -zxf "NVIDIA_Cg.tgz")
  sudo mv "$TEMP/NVIDIA_Cg/Library/Frameworks/Cg.framework" "$TEMP/Cg.framework"
  echo "Unmounting the Nvidia Cg disk Image..."
  hdiutil detach -quiet "/Volumes/cg-3.1.0013"
}

function download_bugsplat() {
  echo "Downloading the Bugsplat dependency..."
  curl -#o "$TEMP/bugsplat.dmg" "http://www.bugsplatsoftware.com/files/MyCocoaCrasher.dmg"
  echo "Mounting Bugsplat disk image..."
  hdiutil attach -nobrowse -quiet "$TEMP/bugsplat.dmg"
  echo "Copying files..."
  sudo cp -R "/Volumes/MyCocoaCrasher/MyCocoaCrasher/BugSplat.framework" "$TEMP/"
  echo "Unmounting Bugsplat disk image..."
  hdiutil detach -quiet "/Volumes/MyCocoaCrasher/"
}


function update_it() {
  if [ "$(detect "$TEMP/$1")" = "YES" ]
    then
    echo "Updating $1"
    echo "Removing old files..."
    for i in "${@:2}"
      do
        sudo rm -fR "$i/$1"
    done
    echo "Copying new files..."
    for i in "${@:2}"
      do
      sudo cp -R -f "$TEMP/$1" "$i"
      sudo chmod -R 777 "$i/$1" # make files writable by launcher
      sudo chown -R "$(whoami)" "$i/$1" # own files
    done
  else
      echo "[ERROR] Couldn't find $1."
      echo "$1 will be skipped."
  fi
}

backup

if [ "$(detect "$GFRAMEWORKS/Adobe Air.framework")" = "NO" ]
  then
  echo "Unable to detect Adobe Air."
  download_air
else
  echo "Using local Adobe Air..."
  sudo rm -fR "$TEMP/Adobe Air.framework"
  cp -R -f "$GFRAMEWORKS/Adobe Air.framework" "$TEMP/Adobe Air.framework"
  if [ "$?" != "0" ]; then
    echo "[Error] Copy failed! Will download instead..." 1>&2
    download_air
  fi
fi
update_it "Adobe Air.framework" "$AIR"



download_cg
update_it "Cg.framework" "$SLN" "$LAUNCHER" "$GAMECL"

download_bugsplat
update_it "Bugsplat.framework" "$SLN" "$LAUNCHER" "$GAMECL" "Contents/LoL/Play League of Legends.app/Contents/Frameworks" "Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks"

echo "Using local libc++ and libc++abi..."
sudo rm -f "$TEMP/libc++.1.dylib" "$TEMP/libc++abi.dylib"
cp -f "/usr/lib/libc++"{"abi.dylib",".1.dylib"} "$TEMP"
if [ "$?" != "0" ]; then
  echo "[Error] Copy failed!" 1>&2
fi
update_it "libc++.1.dylib" "$SLN/../MacOS" "$GAMECL/../MacOS"
update_it "libc++abi.dylib" "$SLN/../MacOS" "$GAMECL/../MacOS"

echo "Cleaning up..."
sudo rm -fR "$TEMP"
echo "Finished! Your League of Legends is now updated. You will need to rerun the script as soon as the client gets updated again."
echo "Report errors, feature requests or any issues at https://github.com/LoLUpdater/LoLUpdater-OSX/issues and not anywhere else."
