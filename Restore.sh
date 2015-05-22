#!/bin/bash
# LoLUpdater for OS X v1.4.4
# Ported by David Knaack
# LoLUpdater for Windows: https://github.com/Loggan08/LoLUpdater
# License: GPLv3 http://www.gnu.org/licenses/gpl-3.0.html
echo "LoLUpdater for OS X - Restore for 1.4.4"
echo "------------------------------------------------------------------------"
echo "Password is required to run this script"
sudo -v

# Edit this line if you installed LoL somewhere else
# For example brew-cask symlinks League of Legends.app to ~/Applications/
cd "/Applications/League of Legends.app"

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

function restore_it() {
  if [ "$(detect "Backups/$1")" = "YES" ]
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
      sudo cp -R -f -a  "Backups/$1" "$i"
    done
  else
      echo "[ERROR] Couldn't find $1."
      echo "$1 will be skipped."
  fi

}

restore_it "Adobe Air.framework" "$AIR"
restore_it "Cg.framework" "$SLN" "$LAUNCHER" "$GAMECL"
restore_it "Bugsplat.framework" "$SLN" "$LAUNCHER" "$GAMECL" "Contents/LoL/Play League of Legends.app/Contents/Frameworks" "Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks"
restore_it "libc++.1.dylib" "$SLN/../MacOS" "$GAMECL/../MacOS"
restore_it "libc++abi.dylib" "$SLN/../MacOS" "$GAMECL/../MacOS"
