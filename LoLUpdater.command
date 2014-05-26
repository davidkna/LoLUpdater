#!/bin/bash
# LoL Updater for OS X v1.3.0
# Ported by David Knaack
# Original for Windows: https://github.com/Loggan08/LoLUpdater
# License: GPL-3 http://www.gnu.org/licenses/gpl-3.0.html
echo "LoLUpdater for OS X"
echo "Password is required to run this script"
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
GAMECL="Contents/LoL/RADS/projects/lol_game_client/releases/"
SLN="$SLN$(ls -lrt "$SLN" | tail -1 | awk '{ print $9 }')/deploy/LeagueOfLegends.app/Contents/Frameworks"
AIR="$AIR$(ls -lrt "$AIR" | tail -1 | awk '{ print $9 }')/deploy/Frameworks"
LAUNCHER="$LAUNCHER$(ls -lrt  -t "$LAUNCHER" | tail -1 | awk '{ print $9 }')/deploy/LoLLauncher.app/Contents/Frameworks"
GAMECL="$GAMECL$(ls -lrt "$GAMECL" | tail -1 | awk '{ print $9 }')/deploy/LeagueOfLegends.app/Contents/Frameworks"

function detect() {
  [[ -e "$1" ]] && printf "YES" || printf "NO"
}

function backup() {
  datetime="$(date +%x\ %X)"
  echo "Creating Backups…"
  mkdir -p "LoLUpdater/Backups/$datetime"
  mkdir -p "LoLupdater/Frameworks"
  cp -R -n -a "$AIR/Adobe Air.framework" "LoLUpdater/Backups/$datetime"
  cp -R -n -a "$LAUNCHER/Cg.framework" "LoLUpdater/Backups/$datetime"
  cp -R -n -a "$SLN/BugSplat.framework" "LoLUpdater/Backups/$datetime"
  cp -R -n -a "$SLN/../MacOS/libc++.1.dylib" "LoLUpdater/Backups/$datetime"
  cp -R -n -a "$SLN/../MacOS/libc++abi.dylib" "LoLUpdater/Backups/$datetime"
}


function download_air() {
  echo "Downloading depency Adobe Air…"
  curl -#o air.dmg "http://airdownload.adobe.com/air/mac/download/13.0/AdobeAIR.dmg"
  echo "Mounting Adobe Air disk image…"
  hdiutil attach -nobrowse -quiet "air.dmg"
  echo "Copying files…"
  sudo rm -fR "$LFRAMEWORKS/Adobe Air.framework"
  sudo cp -R"/Volumes/Adobe Air/Adobe Air Installer.app/Contents/Frameworks/Adobe Air.framework" $LFRAMEWORKS/
  echo "Unmounting Adobe Air disk Image and Cleaning up…"
  hdiutil detach -quiet "/Volumes/Adobe Air/"
  rm -fR "air.dmg"
}

function download_cg() {
  echo "Downloading depency Nvidia Cg…"
  curl -#o "cg.dmg" "http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg"
  echo "Mounting Nvidia Cg disk image…"
  hdiutil attach -nobrowse -quiet "cg.dmg"
  echo "Copying files…"
  mkdir -p "LoLUpdater/tmp"
  cp "/Volumes/cg-3.1.0013/Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz" "LoLUpdater/tmp/"
  (cd "LoLUpdater/tmp" && tar -zxf "NVIDIA_Cg.tgz")
  sudo rm -fR "$LFRAMEWORKS/Cg.framework"
  sudo cp -R "LoLUpdater/tmp/Library/Frameworks/Cg.framework" "$LFRAMEWORKS"
  echo "Unmounting Nvidia Cg disk Image and Cleaning Up…"
  hdiutil detach -quiet "/Volumes/cg-3.1.0013"
  sudo rm -fR "LoLUpdater/tmp" "cg.dmg"
}

function download_bugsplat() {
  echo "Downloading depency Bugsplat…"
  curl -#o "bugsplat.dmg" "http://www.bugsplatsoftware.com/files/MyCocoaCrasher.dmg"
  echo "Mounting Bugsplat disk image…"
  hdiutil attach -nobrowse -quiet "bugsplat.dmg"
  echo "Copying files…"
  sudo rm -fR "$LFRAMEWORKS/Bugsplat.framework"
  sudo cp -R "/Volumes/MyCocoaCrasher/MyCocoaCrasher/BugSplat.framework" "$LFRAMEWORKS/"
  echo "Unmounting Bugsplat disk image and Cleaning Up…"
  hdiutil detach -quiet "/Volumes/MyCocoaCrasher/"
  rm -fR "bugsplat.dmg"
}

function update_it() {
  if [ "$(detect "$LFRAMEWORKS/$1")" = "YES" ]
    then
    echo "Updating $1"
    echo "Removing old files…"
    for i in "${@:2}"
      do
        sudo rm -fR "$i/$1"
    done
    echo "Copying new files…"
    for i in "${@:2}"
      do
      sudo cp -R -f  "${PWD}/$LFRAMEWORKS/$1" "$i"
      sudo chmod -R 777 "$i/$1"
    done
  else
      echo "[ERROR] Couldn't find $1."
      echo "$1 will be skipped."
  fi

}

backup

if [ "$(detect "$GFRAMEWORKS/Adobe Air.framework")" = "NO" ]
  then
  echo "Did not detect Adobe Air."
  download_air
else
  echo "Using local Adobe Air…"
  sudo rm -fR "$LFRAMEWORKS/Adobe Air.framework"
  cp -R -f "$GFRAMEWORKS/Adobe Air.framework" "$LFRAMEWORKS/Adobe Air.framework"
  if [ "$?" != "0" ]; then
      echo "[Error] Copy failed! Will download instead…" 1>&2
      download_air
  fi
fi
update_it "Adobe Air.framework" "$AIR"



if [ "$(detect "$GFRAMEWORKS/Cg.framework")" = "NO" ]
then
  echo "Did not detect Nvidia Cg."
  download_cg
else
  echo "Using local Nvidia Cg…"
  sudo rm -fR "$LFRAMEWORKS/Cg.Framework"
  cp -R -f "$GFRAMEWORKS/Cg.Framework" "$LFRAMEWORKS/Cg.Framework"
  if [ "$?" != "0" ]; then
      echo "[Error] copy failed! Will download instead…" 1>&2
      download_cg
  fi
fi
update_it "Cg.framework" "$SLN" "$LAUNCHER" "$GAMECL"

download_bugsplat
update_it "Bugsplat.framework" "$SLN" "$LAUNCHER" "$GAMECL" "Contents/LoL/Play League of Legends.app/Contents/Frameworks" "Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks"

# echo "Using local libc++ and libc++abi…"
# sudo rm -f "$LFRAMEWORKS/libc++.1.dylib" "$LFRAMEWORKS/libc++abi.dylib"
# cp -f "/usr/lib/libc++"{"abi.dylib",".1.dylib"} "$LFRAMEWORKS"
# if [ "$?" != "0" ]; then
#     echo "[Error] Copy failed!" 1>&2
# fi
# update_it "libc++.1.dylib" "$SLN/../MacOS" "$GAMECL/../MacOS"
# update_it "libc++abi.dylib" "$SLN/../MacOS" "$GAMECL/../MacOS"



echo "Finished! Now your LoL client is updated. You will need to rerun the script as soon as the client gets updated again."
echo "Report errors, feature requests or any issues at https://github.com/davidkna/LoLUpdater/issues and not anywhere else."
