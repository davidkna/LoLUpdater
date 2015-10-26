#!/bin/bash
# LoLUpdater for OS X v1.6.3
# Ported by David Knaack
# LoLUpdater for Windows: https://github.com/Loggan08/LoLUpdater
# License: GPLv3 http://www.gnu.org/licenses/gpl-3.0.html
{
echo "LoLUpdater for OS X 1.6.3"
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

TEMP="$(mktemp -dt LoLUpdaterXXXXXXX)"
if [ "$?" != "0" ]; then
    echo "[Error] Failed to create temporary direcory..." 1>&2
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
    [[ -e $1 ]]
}

function backup() {
    echo "Creating Backups..."
    ! detect Backups \
        && mkdir "Backups"

    backup_it "Adobe Air.framework" "$AIR/Adobe Air.framework"
    backup_it "BugSplat.framework" "$SLN/BugSplat.framework"
    backup_it "Cg.framework" "$LAUNCHER/Cg.framework"
}

function backup_it() {
    local FILENAME="$1"
    local SOURCE="$2"
    if ! detect "Backups/$FILENAME"
        then
            cp -R -n -a "$SOURCE" "Backups/"
            if [ "$?" != "0" ]; then
                echo "[Error] Failed to backup $FILENAME..." 1>&2
                exit 1
            fi
        else
            echo "[Notice] Backup for $FILENAME already exists. Skipping!"
    fi
}


function download_air() {
    local AIRVERSION="19.0"
    local MOUNTPOINT="$TEMP/AirMount"

    echo "Downloading the Adobe Air dependency..."
    curl -L -# -o "$TEMP/air.dmg" "https://airdownload.adobe.com/air/mac/download/$AIRVERSION/AdobeAIR.dmg"

    echo "Mounting Adobe Air disk image..."
    mkdir -p "$MOUNTPOINT"
    hdiutil attach -nobrowse -quiet -mountpoint "$MOUNTPOINT" "$TEMP/air.dmg"

    echo "Copying files..."
    sudo cp -R "$MOUNTPOINT/Adobe Air Installer.app/Contents/Frameworks/Adobe Air.framework" "$TEMP"

    echo "Unmounting the Adobe Air disk Image..."
    hdiutil detach -quiet "$MOUNTPOINT"
}

function download_cg() {
    local CHECKSUM="85c7a0de82252b703191fee5fe7b29f60d357924dc7b8ca59c2badeac7af407d"
    local FILECHECKSUM
    local MOUNTPOINT="$TEMP/CgMount"
    local CGTEMP="$TEMP/CgArchive"

    mkdir -p "$MOUNTPOINT"

    echo "Downloading the Nvidia Cg dependency..."
    curl -L -k -# -o "$TEMP/cg.dmg" "https://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg"

    echo "Verifying the Cg checksum..."

    if ! checksum_test "$CHECKSUM" "$TEMP/cg.dmg"; then
        echo "[ERROR] Failed to match checksum for cg.dmg"
        return $?
    fi

    echo "Mounting Nvidia Cg disk image..."
    hdiutil attach -nobrowse -quiet -mountpoint "$MOUNTPOINT" "$TEMP/cg.dmg"

    echo "Copying und extracting files..."
    mkdir -p "$CGTEMP"
    tar -zxf "$MOUNTPOINT/Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz" -C "$CGTEMP"
    sudo mv "$CGTEMP/Library/Frameworks/Cg.framework" "$TEMP/Cg.framework"

    echo "Unmounting the Nvidia Cg disk Image..."
    hdiutil detach -quiet "$MOUNTPOINT"
}

function download_bugsplat() {
    local MOUNTPOINT="$TEMP/bugsplatMount"
    local CHECKSUM="09f9d5d54a90cb93b01844f31f8d7fcb3c216d25b4fbdff5d7058b49b4671c7c"

    echo "Downloading the Bugsplat dependency..."
    curl -L -# -o "$TEMP/bugsplat.dmg" "https://www.bugsplatsoftware.com/files/MyCocoaCrasher.dmg"

    echo "Verifying the Bugsplat checksum..."
    if ! checksum_test "$CHECKSUM" "$TEMP/bugsplat.dmg"; then
        echo "[ERROR] Failed to match checksum for bugsplat.dmg"
        return $?
    fi

    echo "Mounting Bugsplat disk image..."
    mkdir -p "$MOUNTPOINT"
    hdiutil attach -nobrowse -quiet -mountpoint "$MOUNTPOINT" "$TEMP/bugsplat.dmg"

    echo "Copying files..."
    sudo cp -R "$MOUNTPOINT/MyCocoaCrasher/BugSplat.framework" "$TEMP/"

    echo "Unmounting Bugsplat disk image..."
    hdiutil detach -quiet "$MOUNTPOINT"
}

function update_it() {
    if detect "$TEMP/$1"; then
        echo "Updating $1"

        echo "Removing old files..."
        for i in "${@:2}"; do
            sudo rm -fR "$i/$1"
        done

        echo "Copying new files..."
        for i in "${@:2}"; do
            sudo cp -R -f "$TEMP/$1" "$i"
            sudo chmod -R 777 "$i/$1" # make files writable by launcher
            sudo chown -R "$(whoami):admin" "$i/$1" # own files
        done
    else
        echo "[ERROR] Couldn't find $1."
        echo "$1 will be skipped."
    fi
}

function checksum_test() {
    local CHECKSUM="$1"
    local FILE="$2"
    local FILECHECKSUM
    FILECHECKSUM="$(shasum -a 256 $FILE)"

    [[ "$?" == "0" ]] && [[ "$CHECKSUM  $FILE" == "$FILECHECKSUM" ]]
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

    local GFRAMEWORKS="/Library/Frameworks"

    if ! detect "$GFRAMEWORKS/Adobe Air.framework"; then
        echo "[Notice] Unable to find locally installed Adobe Air. Downloading instead!"
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

    echo "Cleaning up..."
    sudo rm -fR "$TEMP"
    echo "Finished! Your League of Legends is now updated. You will need to rerun the script as soon as the client gets updated again."
    echo "Report errors, feature requests or any issues at https://github.com/LoLUpdater/LoLUpdater-OSX/issues."
}

backup && main
}
