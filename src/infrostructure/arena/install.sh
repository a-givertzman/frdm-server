#! /bin/bash

#
# Use it ti proper install Arena SDK under Linux Debian
# 
# Just copy in into Arena SDK folder
# and execute it

#
# System path to install
installPath="/usr/lib/arena-sdk"

if [ "$1" = "-r" ]; then
    echo "Removing if installed in $installPath"
    cd $installPath
    sudo $installPath/Arena_SDK_Linux_x64.conf -r
    sudo rm -rf $installPath
    echo
    echo "Arena SDK uninstalled into $installPath"
    exit 0
fi



echo "Removing existing Arena SDK from $installPath..."
sudo rm -rf $installPath

echo "Installing Arena SDK into $installPath..."
sudo cp -r ./ArenaSDK_Linux_x64 $installPath

cd $installPath
sudo chmod +x $installPath/Arena_SDK_Linux_x64.conf

echo "Use GENICAM_GENTL64_PATH environment variable to access lib64"

sudo $installPath/Arena_SDK_Linux_x64.conf -r
sudo $installPath/Arena_SDK_Linux_x64.conf

echo "Arena SDK installed into $installPath"
