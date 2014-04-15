#!/bin/bash
# LoL Updater for OS X
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




FRAMEWORKS=/Library/Frameworks
SLN=Contents/LoL/RADS/solutions/lol_game_client_sln/releases/
AIR=Contents/LoL/RADS/projects/lol_air_client/releases/
LAUNCHER=Contents/LoL/RADS/projects/lol_launcher/releases/

SLN=$SLN$(ls -lrt $SLN | tail -1 | awk '{ print $9 }')/deploy/LeagueOfLegends.app/Contents/Frameworks
AIR=$AIR$(ls -lrt $AIR | tail -1 | awk '{ print $9 }')/deploy/Frameworks
LAUNCHER=$LAUNCHER$(ls -lrt  -t $LAUNCHER | tail -1 | awk '{ print $9 }')/deploy/LoLLauncher.app/Contents/Frameworks
PLAY=Contents/LoL/Play\ League\ of\ Legends.app/Contents/Frameworks

function detect_framework() {
  [[ -e $FRAMEWORKS/$1.framework ]] && echo YES || echo NO
}


echo "Creating Backups..."
mkdir backups
cp -R -n -a $AIR/Adobe\ Air.framework backups/
cp -R -n -a $LAUNCHER/Cg.framework backups/
cp -R -n -a $PLAY/BugSplat.framework backups/

if [ $(detect_framework "Adobe Air") = YES ]
then
  echo "Updating Adobe AIR"
  echo "Removing old files..."
  sudo rm -fR $AIR/Adobe\ Air.framework
  echo "Symlinking new files..."
  ln -s $FRAMEWORKS/Adobe\ Air.framework $AIR
else
  echo -e "\033[1mDid not detect Adobe Air. Not Updated.\033[0m"
fi

if [ $(detect_framework Cg) = YES ]
then
  echo "Updating NVIDIA Cg"
  echo "Removing old files..."
  sudo rm -fR $SLN/Cg.framework
  sudo rm -fR $LAUNCHER/Cg.framework
  echo "Symlinking new files..."
  ln -s $FRAMEWORKS/Cg.framework $SLN
  ln -s $FRAMEWORKS/Cg.framework $LAUNCHER
else
  echo -e "\033[1mDid not detect NVIDIA Cg. Not Updated.\033[0m"
fi

if [ $(detect_framework Bugsplat) = YES ]
then
  echo "Updating Bugsplat"
  echo "Removing old files..."
  sudo rm -fR $SLN/Bugsplat.framework
  sudo rm -fR $LAUNCHER/Bugsplat.framework
  echo "Symlinking new files..."
  ln -s $FRAMEWORKS/Bugsplat.framework $PLAY
  ln -s $FRAMEWORKS/Bugsplat.framework $SLN
  ln -s $FRAMEWORKS/Bugsplat.framework $LAUNCHER
else
  echo -e "\033[1mDid not detect Bugsplat. Not Updated.\033[0m"
fi

echo "Finished! Now your LoL client is updated. You will need to rerun the script as soon as the client gets updated again."
