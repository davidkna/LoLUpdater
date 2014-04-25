#!/bin/bash
# LoL Updater for OS X v1.1.2
# Ported by David Knaack
# Original for Windows: https://github.com/Loggan08/LoLUpdater
# License: GPL-3 http://www.gnu.org/licenses/gpl-3.0.html
function ebold {
   echo -e "\033[1m$1\033[0m"
}

ebold "LoLUpdater for OS X"
ebold "Password is required to run this script"
sudo cd .

CURRENTDIR=${PWD##*/}
if [ $CURRENTDIR != "League of Legends.app" ]; then
  cd "/Applications/League of Legends.app"
fi

GFRAMEWORKS="/Library/Frameworks"
LFRAMEWORKS="LoLUpdater/Frameworks"
SLN="Contents/LoL/RADS/solutions/lol_game_client_sln/releases/"
AIR="Contents/LoL/RADS/projects/lol_air_client/releases/"
LAUNCHER="Contents/LoL/RADS/projects/lol_launcher/releases/"
SLN="$SLN$(ls -lrt "$SLN" | tail -1 | awk '{ print $9 }')/deploy/LeagueOfLegends.app/Contents/Frameworks"
AIR="$AIR$(ls -lrt "$AIR" | tail -1 | awk '{ print $9 }')/deploy/Frameworks"
LAUNCHER="$LAUNCHER$(ls -lrt  -t "$LAUNCHER" | tail -1 | awk '{ print $9 }')/deploy/LoLLauncher.app/Contents/Frameworks"

function detect() {
  [[ -e "$1" ]] && printf "YES" || printf "NO"
}

function backup() {
  ebold "Creating Backups..."
  mkdir -p "LoLUpdater/Backups/$(date +%X-%x)"
  mkdir -p "LoLupdater/Frameworks"
  cp -R -n -a "$AIR/Adobe Air.framework" "LoLUpdater/Backups/$(date +%X-%x)"
  cp -R -n -a "$LAUNCHER/Cg.framework" "LoLUpdater/Backups/$(date +%X-%x)"
  cp -R -n -a "$SLN/BugSplat.framework" "LoLUpdater/Backups/$(date +%X-%x)"
}

function download_air() {
  ebold "Downloading depency Adobe Air..."
  curl -#o air.dmg "http://airdownload.adobe.com/air/mac/download/13.0/AdobeAIR.dmg"
  ebold "Mounting Adobe Air disk image..."
  hdiutil attach -nobrowse -quiet "air.dmg"
  ebold "Copying files..."
  sudo rm -fR "$LFRAMEWORKS/Adobe Air.framework"
  sudo cp -R"/Volumes/Adobe Air/Adobe Air Installer.app/Contents/Frameworks/Adobe Air.framework" $LFRAMEWORKS/
  ebold "Unmounting Adobe Air disk Image and Cleaning up..."
  hdiutil detach -quiet "/Volumes/Adobe Air/"
  rm -fR "air.dmg"
}

function download_cg() {
  ebold "Downloading depency Nvidia Cg..."
  curl -#o "cg.dmg" "http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg"
  ebold "Mounting Nvidia Cg disk image..."
  hdiutil attach -nobrowse -quiet "cg.dmg"
  ebold "Copying files..."
  mkdir -p "LoLUpdater/tmp"
  cp "/Volumes/cg-3.1.0013/Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz" "LoLUpdater/tmp/"
  (cd "LoLUpdater/tmp" && tar -zxf "NVIDIA_Cg.tgz")
  sudo rm -fR "$LFRAMEWORKS/Cg.framework"
  sudo cp -R "LoLUpdater/tmp/Library/Frameworks/Cg.framework" "$LFRAMEWORKS"
  ebold "Unmounting Nvidia Cg disk Image and Cleaning Up..."
  hdiutil detach -quiet "/Volumes/cg-3.1.0013"
  sudo rm -fR "LoLUpdater/tmp" "cg.dmg"
}

function download_bugsplat() {
  ebold "Downloading depency Bugsplat..."
  curl -#o "bugsplat.dmg" "http://www.bugsplatsoftware.com/files/MyCocoaCrasher.dmg"
  ebold "Mounting Bugsplat disk image..."
  hdiutil attach -nobrowse -quiet "bugsplat.dmg"
  ebold "Copying files..."
  sudo rm -fR "$LFRAMEWORKS/Bugsplat.framework"
  sudo cp -R "/Volumes/MyCocoaCrasher/MyCocoaCrasher/BugSplat.framework" "$LFRAMEWORKS/"
  ebold "Unmounting Bugsplat disk image and Cleaning Up..."
  hdiutil detach -quiet "/Volumes/MyCocoaCrasher/"
  rm -fR "bugsplat.dmg"
}

function update_it() {
  if [ "$(detect "$LFRAMEWORKS/$1")" = "YES" ]
    then
    ebold "Updating $1"
    ebold "Removing old files..."
    for i in "${@:2}"
      do
        sudo rm -fR "$i/$1"
    done
    ebold "Symlinking new files..."
    for i in "${@:2}"
      do
      ln -Fs "${PWD}/$LFRAMEWORKS/$1" "$i"
    done
  else
      ebold "[ERROR] Couldn't find $1."
      ebold "$1 will be skipped."
  fi

}

backup

if [ "$(detect "$GFRAMEWORKS/Adobe Air.framework")" = "NO" ]
  then
  ebold "Did not detect Adobe Air."
  download_air
else
  ebold "Using local Adobe Air..."
  sudo rm -fR "$LFRAMEWORKS/Adobe Air.framework"
  cp -R -f "$GFRAMEWORKS/Adobe Air.framework" "$LFRAMEWORKS/Adobe Air.framework"
  if [ "$?" != "0" ]; then
      echo "[Error] Copy failed! Will download instead..." 1>&2
      download_air
  fi
fi
update_it "Adobe Air.framework" "$AIR"



if [ "$(detect "$GFRAMEWORKS/Cg.framework")" = "NO" ]
then
  ebold "Did not detect Nvidia Cg."
  download_cg
else
  ebold "Using local Nvidia Cg..."
  sudo rm -fR "$LFRAMEWORKS/Cg.Framework"
  cp -R -f "$GFRAMEWORKS/Cg.Framework" "$LFRAMEWORKS/Cg.Framework"
  if [ "$?" != "0" ]; then
      echo "[Error] copy failed! Will download instead..." 1>&2
      download_cg
  fi
fi
update_it "Cg.framework" "$SLN" "$LAUNCHER"


if [ "$(detect "$GFRAMEWORKS/Bugsplat.framework")" = "NO" ]
then
  ebold "Did not detect Bugsplat."
  download_bugsplat
else
  ebold "Using local Bugsplat..."
  sudo rm -fR "$LFRAMEWORKS/Bugsplat.Framework"
  cp -R -f "$GFRAMEWORKS/Bugsplat.Framework" "$LFRAMEWORKS/Bugsplat.Framework"
  if [ "$?" != "0" ]; then
      echo "[Error] Copy failed! Will download instead..." 1>&2
      download_bugsplat
  fi
fi
update_it "Bugsplat.framework" "$SLN" "$LAUNCHER" "Contents/LoL/Play League of Legends.app/Contents/Frameworks" "Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks"




ebold "Finished! Now your LoL client is updated. You will need to rerun the script as soon as the client gets updated again."
ebold "Report errors or any issues at https://github.com/davidkna/LoLUpdater/issues and not anywhere else."
