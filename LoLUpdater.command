#!/bin/bash
# LoL Updater
# Ported to OS X
# Original https://github.com/Loggan08/LoLUpdater

function ebold { # bold echo for messages from script
  echo -e "\033[1m$1\033[0m"
}


ebold "Password is required to run this script"
sudo cd $1 || .
CURRENTDIR=${PWD##*/}

if [ $CURRENTDIR != "League of Legends.app" ]; then
  cd "/Applications/League of Legends.app"
fi


SLN=Contents/LoL/RADS/solutions/lol_game_client_sln/releases/
AIR=Contents/LoL/RADS/projects/lol_air_client/releases/
LAUNCHER=Contents/LoL/RADS/projects/lol_launcher/releases/

SLN="Contents/LoL/RADS/solutions/lol_game_client_sln/releases/$(ls -lrt Contents/LoL/RADS/solutions/lol_game_client_sln/releases/ | tail -1 | awk '{ print $9 }')/deploy/League of Legends.app/Contents/Frameworks"
AIR="$AIR$(ls -lrt $AIR | tail -1 | awk '{ print $9 }')/deploy/Frameworks"
LAUNCHER="$LAUNCHER$(ls -lrt  -t $LAUNCHER   | tail -1 | awk '{ print $9 }')/deploy/LoL Launcher.app/Contents/Frameworks"
PLAY="Contents/LoL/Play League of Legends.app/Contents/Frameworks"



ebold "Creating Backups..."
mkdir backups
no | cp -aRi "$AIR/Adobe Air.framework" backups/
no | cp -aRi "$LAUNCHER/Cg.framework" backups/
no | cp -aRi "$PLAY/BugSplat.framework" backups/


ebold "Downloading depencies..."
curl \
  -o air.dmg      http://airdownload.adobe.com/air/mac/download/13.0/AdobeAIR.dmg \
  -o cg.dmg       http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg \
  -o bugsplat.dmg http://www.bugsplatsoftware.com/files/MyCocoaCrasher.dmg


ebold "Mounting Adobe Air disk image..."
hdiutil attach -nobrowse air.dmg

ebold "Copying files..."
sudo cp -aRf \
  "/Volumes/Adobe Air/Adobe Air Installer.app/Contents/Frameworks/Adobe Air.framework" \
  "$AIR/"

ebold "Unmounting Adobe Air disk Image..."
hdiutil detach "/Volumes/Adobe Air/"


ebold "Mounting Nvidia Cg disk image..."
hdiutil attach -nobrowse cg.dmg

ebold "Copying files..."
mkdir tmp
cp "/Volumes/cg-3.1.0013/Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz" tmp/
cd tmp
tar -zxvf "NVIDIA_Cg.tgz"
cd ..
sudo cp -aRf "tmp/Library/Frameworks/Cg.framework" "$LAUNCHER/"
sudo cp -aRf "tmp/Library/Frameworks/Cg.framework" "$SLN/"

ebold "Unmounting Nvidia Cg disk Image..."
hdiutil detach "/Volumes/cg-3.1.0013"


ebold "Mounting Bugsplat disk image..."
hdiutil attach -nobrowse bugsplat.dmg

ebold "Copying files..."
sudo cp -aRf "/Volumes/MyCocoaCrasher/MyCocoaCrasher/BugSplat.framework" "$PLAY/"
sudo cp -aRf "/Volumes/MyCocoaCrasher/MyCocoaCrasher/BugSplat.framework" "$SLN/"
sudo cp -aRf "/Volumes/MyCocoaCrasher/MyCocoaCrasher/BugSplat.framework" "$LAUNCHER/"

ebold "Unmounting Bugsplat disk image..."
hdiutil detach "/Volumes/MyCocoaCrasher/"


ebold "Cleaning up..."
rm  -rvf air.dmg cg.dmg bugsplat.dmg tmp

ebold "Finished! Now your LoL client is updated. You will need to rerun as soon as the client gets updated again."
