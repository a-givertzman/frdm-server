#! /bin/bash

#
# Use this script to proper install Arena SDK under Linux Debian
#
# Installation steps:
# 
# - Download Arena SDK v0.1.95 or later from [Downloads](https://thinklucid.com/downloads-hub/) (for Ubuntu)
#
# - Execute install script with the path to the Arena SDK donloaded archive
#
#   ./install.sh "path/ArenaSDK_v0.1.95_Linux_x64.tar.gz"
#    

if [ -z "$1" ]; then
    echo 'Path to Arena SDK is not specified, use command: ./install.sh "path/ArenaSDK_v0.1.95_Linux_x64.tar.gz"'
else
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
    else
        sdkPath=$1

        echo "Removing existing Arena SDK from $installPath..."
        sudo rm -rf $installPath
        sudo mkdir -p $installPath

        echo "Extracting and Installing Arena SDK ..."
        echo -e "\t from '$sdkPath'"
        echo -e "\t   to '$installPath'"
        sudo tar -xf $sdkPath -C $installPath --strip-components=1
        sync

        cd $installPath
        sudo chmod +x $installPath/Arena_SDK_Linux_x64.conf

        echo "Use GENICAM_GENTL64_PATH environment variable to access lib64"

        sudo $installPath/Arena_SDK_Linux_x64.conf -r
        sudo $installPath/Arena_SDK_Linux_x64.conf

        echo "Arena SDK installed into $installPath"
    fi
fi


