#!/bin/bash
function ebold {
   echo -e "\033[1m$1\033[0m"
}

ebold "LoLUpdater for OS X - Restore"
ebold "Password is required to run this script"
sudo cd .

CURRENTDIR=${PWD##*/}
if [ $CURRENTDIR != "League of Legends.app" ]; then
  cd "/Applications/League of Legends.app"
fi

LFRAMEWORKS="LoLUpdater/Frameworks"
SLN="Contents/LoL/RADS/solutions/lol_game_client_sln/releases/"
AIR="Contents/LoL/RADS/projects/lol_air_client/releases/"
LAUNCHER="Contents/LoL/RADS/projects/lol_launcher/releases/"
SLN="$SLN$(ls -lrt "$SLN" | tail -1 | awk '{ print $9 }')/deploy/LeagueOfLegends.app/Contents/Frameworks"
AIR="$AIR$(ls -lrt "$AIR" | tail -1 | awk '{ print $9 }')/deploy/Frameworks"
LAUNCHER="$LAUNCHER$(ls -lrt  -t "$LAUNCHER" | tail -1 | awk '{ print $9 }')/deploy/LoLLauncher.app/Contents/Frameworks"
cd LoLUpdater/Backups
shopt -s nullglob
BACKUPS=(*/)
shopt -u nullglob
cd ../..


function restore_it() {
  ebold "Deleting old files..."
  sudo rm -fR "$AIR/Adobe Air.framework"
  sudo rm -fR "$SLN/Cg.framework"
  sudo rm -fR "$LAUNCHER/Cg.framework"
  sudo rm -fR "$SLN/Bugsplat.framework"
  sudo rm -fR "$LAUNCHER/Bugsplat.framework"
  sudo rm -fR "Contents/LoL/Play League of Legends.app/Contents/Frameworks/Bugsplat.framework"
  sudo rm -fR "Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks/Bugsplat.framework"

  ebold "Moving new files..."
  cp -R -f "LoLUpdater/Backups/$1Adobe Air.framework" "$AIR/"
  cp -R -f "LoLUpdater/Backups/$1Cg.framework" "$SLN/"
  cp -R -f "LoLUpdater/Backups/$1Cg.framework" "$LAUNCHER/"
  cp -R -f "LoLUpdater/Backups/$1Bugsplat.framework" "$SLN/"
  cp -R -f "LoLUpdater/Backups/$1Bugsplat.framework" "$LAUNCHER/"
  cp -R -f "LoLUpdater/Backups/$1Bugsplat.framework" "Contents/LoL/Play League of Legends.app/Contents/Frameworks/"
  cp -R -f "LoLUpdater/Backups/$1Bugsplat.framework" "Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks/"
}

ebold "Which backup do you want to use?"
select MYBACKUP in $BACKUPS; do
    restore_it $MYBACKUP
    ebold "Finished! Now your Backup should be restored."
    ebold "Report errors or any issues at https://github.com/davidkna/LoLUpdater/issues and not anywhere else."
    exit;
done
