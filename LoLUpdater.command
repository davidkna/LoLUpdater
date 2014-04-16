#!/bin/bash
# LoL Updater for OS X v1.0.1
# Ported by David Knaack
# Original for Windows: https://github.com/Loggan08/LoLUpdater
# License: GPL-3 http://www.gnu.org/licenses/gpl-3.0.html

function ebold { # bold ebold for messages from script
   echo -e "\033[1m$1\033[0m"
}

ebold "LoLUpdater for OS X"
ebold "Password is required to run this script"
sudo cd .
CURRENTDIR=${PWD##*/}

if [ $CURRENTDIR != "League of Legends.app" ]; then
  cd "/Applications/League of Legends.app"
fi

GFRAMEWORKS=/Library/Frameworks
LFRAMEWORKS=LoLUpdater/Frameworks
SLN=Contents/LoL/RADS/solutions/lol_game_client_sln/releases/
AIR=Contents/LoL/RADS/projects/lol_air_client/releases/
LAUNCHER=Contents/LoL/RADS/projects/lol_launcher/releases/
SLN=$SLN$(ls -lrt $SLN | tail -1 | awk '{ print $9 }')/deploy/LeagueOfLegends.app/Contents/Frameworks
AIR=$AIR$(ls -lrt $AIR | tail -1 | awk '{ print $9 }')/deploy/Frameworks
LAUNCHER=$LAUNCHER$(ls -lrt  -t $LAUNCHER | tail -1 | awk '{ print $9 }')/deploy/LoLLauncher.app/Contents/Frameworks

function detect_framework() {
  [[ -e $GFRAMEWORKS/$1.framework ]] && echo YES || ebold NO
}

ebold "Creating Backups..."
mkdir -p LoLUpdater/Backups/$(date +%F)
mkdir -p LoLupdater/Frameworks
cp -R -n -a $AIR/Adobe\ Air.framework LoLUpdater/Backups/$(date +%F)
cp -R -n -a $LAUNCHER/Cg.framework LoLUpdater/Backups/$(date +%F)
cp -R -n -a $SLN/BugSplat.framework LoLUpdater/Backups/$(date +%F)

if [ $(detect_framework "Adobe Air") = NO ]
  then
  ebold "Did not detect Adobe Air."
  ebold "Downloading depency Adobe Air..."
  curl -o air.dmg http://airdownload.adobe.com/air/mac/download/13.0/AdobeAIR.dmg
  ebold "Mounting Adobe Air disk image..."
  hdiutil attach -nobrowse air.dmg
  ebold "Copying files..."
  sudo cp -R"/Volumes/Adobe Air/Adobe Air Installer.app/Contents/Frameworks/Adobe Air.framework" $LFRAMEWORKS/
  ebold "Unmounting Adobe Air disk Image and Cleaning up..."
  hdiutil detach "/Volumes/Adobe Air/"
  rm -fR air.dmg
else
  ebold "Using local Adobe Air..."
  sudo rm -fR $LFRAMEWORKS/Adobe\ Air.framework
  cp -R -f $GFRAMEWORKS/Adobe\ Air.framework $LFRAMEWORKS/Adobe\ Air.framework
fi

ebold "Updating LoL Adobe AIR"
ebold "Removing old files..."
sudo rm -fR $AIR/Adobe\ Air.framework
ebold "Symlinking new files..."
ln -s $LFRAMEWORKS/Adobe\ Air.framework $AIR


if [ $(detect_framework Cg) = NO ]
then
  ebold "Downloading depency Nvidia Cg..."
  curl -o cg.dmg http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg
  ebold "Mounting Nvidia Cg disk image..."
  hdiutil attach -nobrowse cg.dmg
  ebold "Copying files..."
  mkdir -p LoLUpdater/tmp
  cp "/Volumes/cg-3.1.0013/Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz" tmp/
  (cd LoLUpdater/tmp && tar -zxf "NVIDIA_Cg.tgz")
  sudo cp -R "LoLUpdater/tmp/Library/Frameworks/Cg.framework" "$LFRAMEWORKS"
  ebold "Unmounting Nvidia Cg disk Image and Cleaning Up..."
  hdiutil detach "/Volumes/cg-3.1.0013"
  sudo rm -fR tmp cg.dmg
else
  ebold "Using local Nvidia Cg..."
  sudo rm -fR $LFRAMEWORKS/Cg.Framework
  cp -R -f $GFRAMEWORKS/Cg.Framework $LFRAMEWORKS/Cg.Framework
fi

ebold "Updating LoL Nvidia Cg"
ebold "Removing old files..."
sudo rm -fR $SLN/Cg.framework
sudo rm -fR $LAUNCHER/Cg.framework
ebold "Symlinking new files..."
ln -s $LFRAMEWORKS/Cg.framework $SLN
ln -s $LFRAMEWORKS/Cg.framework $LAUNCHER


if [ $(detect_framework Bugsplat) = NO ]
then
  ebold "Downloading depency Bugsplat..."
  curl -o bugsplat.dmg http://www.bugsplatsoftware.com/files/MyCocoaCrasher.dmg
  ebold "Mounting Bugsplat disk image..."
  hdiutil attach -nobrowse bugsplat.dmg
  ebold "Copying files..."
  sudo cp -R "/Volumes/MyCocoaCrasher/MyCocoaCrasher/BugSplat.framework" "$LFRAMEWORKS/"
  ebold "Unmounting Bugsplat disk image and Cleanign Up..."
  hdiutil detach "/Volumes/MyCocoaCrasher/"
  rm -fR bugsplat.dmg
else
  ebold "Using local Bugsplat..."
  sudo rm -fR $LFRAMEWORKS/Bugsplat.Framework
  cp -R -f $GFRAMEWORKS/Bugsplat.Framework $LFRAMEWORKS/Bugsplat.Framework
fi

ebold "Updating LoL Bugsplat"
ebold "Removing old files..."
sudo rm -fR $SLN/Bugsplat.framework
sudo rm -fR $LAUNCHER/Bugsplat.framework
sudo rm -fR Contents/LoL/Play\ League\ of\ Legends.app/Contents/Frameworks/Bugsplat.framework
sudo rm -fR Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks/Bugsplat.framework
ebold "Symlinking new files..."
ln -s $LFRAMEWORKS/Bugsplat.framework $SLN
ln -s $LFRAMEWORKS/Bugsplat.framework $LAUNCHER
ln -s $LFRAMEWORKS/Bugsplat.framework Contents/LoL/Play\ League\ of\ Legends.app/Contents/Frameworks
ln -s $LFRAMEWORKS/Bugsplat.framework Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks


ebold "Finished! Now your LoL client is updated. You will need to rerun the script as soon as the client gets updated again."
