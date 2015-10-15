#!/bin/bash
# LoLUpdater for OS X v1.6.2
# Ported by David Knaack
# LoLUpdater for Windows: https://github.com/Loggan08/LoLUpdater
# License: GPLv3 http://www.gnu.org/licenses/gpl-3.0.html
{
echo "LoLUpdater for OS X - Restore for 1.6.2"
echo "------------------------------------------------------------------------"
echo "[Help] Please supply a command line argument if you haven't installed LoL at \"/Applications/League of Legends.app\"."
echo "Password is required to run this script"
sudo -p "Password for %u: " -v

# Keep sudo alive
while true; do sudo -n true; sleep 60; kill -0 "$$" || exit; done 2>/dev/null &

# shellcheck disable=SC2164
cd ${1:-"/Applications/League of Legends.app"}
if [ "$?" != "0" ]; then
    echo "[Error] Could not find LoL. Please supply a command line argument specifying where you installed it!" 1>&2
    exit 1
fi

function get_full_path() {
    local versionNumber
    versionNumber=$(ls -lrt "$1" | tail -1 | awk '{ print $9 }')
    if [ "$?" != "0" ] || [ -z "$versionNumber" ] || ! detect "$1/$versionNumber/$2"; then
      echo "[Error] Could not find a path for LoL..." 1>&2
      exit 1
    fi

    echo "$1/$versionNumber/$2"
}

function detect() {
    [[ -e "$1" ]]
}

function restore_it() {
    if detect "Backups/$1"; then
        echo "Updating $1"
        echo "Removing old files..."
        for i in "${@:2}"; do
            sudo rm -fR "$i/$1"
        done
        echo "Copying new files..."
        for i in "${@:2}"; do
            sudo cp -R -f -a  "Backups/$1" "$i"
        done
    else
        echo "[ERROR] Couldn't find $1 in Backups."
        echo "$1 will be skipped."
    fi
}

function main() {
    local SLN
    local AIR
    local LAUNCHER
    local GAMECL

    SLN="$(get_full_path Contents/LoL/RADS/solutions/lol_game_client_sln/releases deploy/LeagueOfLegends.app/Contents/Frameworks)"
    AIR="$(get_full_path Contents/LoL/RADS/projects/lol_air_client/releases deploy/Frameworks)"
    LAUNCHER="$(get_full_path Contents/LoL/RADS/projects/lol_launcher/releases deploy/LoLLauncher.app/Contents/Frameworks)"
    GAMECL="$(get_full_path Contents/LoL/RADS/projects/lol_game_client/releases deploy/LeagueOfLegends.app/Contents/Frameworks)"

    restore_it "Adobe Air.framework" "$AIR"
    restore_it "Cg.framework" "$SLN" "$LAUNCHER" "$GAMECL"
    restore_it "Bugsplat.framework" "$SLN" "$LAUNCHER" "$GAMECL" "Contents/LoL/Play League of Legends.app/Contents/Frameworks" "Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks"
}
main
}
