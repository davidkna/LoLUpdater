#!/bin/bash
echo "LoLUpdater for OS X - Restore"
echo "Password is required to run this script"
sudo cd .

CURRENTDIR=${PWD##*/}
if [ $CURRENTDIR != "League of Legends.app" ]; then
  cd "/Applications/League of Legends.app"
fi

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

function restore_it() {
  if [ "$(detect "$1")" = "YES" ]
  then
    sudo rm -fR "$3/$2"
    sudo cp -R -f "$1/$2" "$3"
  else
    echo "[ERROR] Did not find $1 in Backup!"
  fi
}


function restore() {
  cd "../.."
  echo "Restoring…"
  restore_it "$1" "Adobe Air.framework" "$AIR"
  restore_it "$1" "Cg.framework" "$SLN"
  restore_it "$1" "Cg.framework" "$LAUNCHER"
  restore_it "$1" "Cg.framework" "$GAMECL"
  restore_it "$1" "Bugsplat.framework" "$SLN"
  restore_it "$1" "Bugsplat.framework" "$LAUNCHER"
  restore_it "$1" "Bugsplat.framework" "$GAMECL"
  restore_it "$1" "Bugsplat.framework" "Contents/LoL/Play League of Legends.app/Contents/Frameworks/"
  restore_it "$1" "Bugsplat.framework" "Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks/"
  restore_it "$1" "libc++.1.dylib" "$SLN/../MacOS/"
  restore_it "$1" "libc++.1.dylib" "$GAMECL/../MacOS/"
  restore_it "$1" "libc++abi.dylib" "$SLN/../MacOS/"
  restore_it "$1" "libc++abi.dylib" "$GAMECL/../MacOS/"
}

echo "Which backup do you want to use?"

cd "LoLUpdater/Backups"
select MYBACKUP in *; do test -n "$MYBACKUP" && break; echo ">>> Invalid Selection"; done
restore "$(pwd)/$MYBACKUP"
