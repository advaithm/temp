#!/bin/bash

set -x
set -e

# building in temporary directory to keep system clean
TEMP_BASE=/dev/shm

BUILD_DIR=$(mktemp -d -p "$TEMP_BASE" appimage-build-XXXXXX)

# make sure to clean up build dir, even if errors occur
cleanup () {
    if [ -d "$BUILD_DIR" ]; then
        rm -rf "$BUILD_DIR"
    fi
}
trap cleanup EXIT

# store repo root as variable
REPO_ROOT=$(readlink -f $(dirname $(dirname $0)))
OLD_CWD=$(readlink -f .)

# switch to build dir
pushd "$BUILD_DIR"
echo $BUILD_DIR

git clone https://github.com/advaithm/temp.git

cd $BUILD_DIR/temp/custom_tools/custom_app

# build project and install files into AppDir
make stress && make webserver && make app
mkdir AppDir/
mkdir AppDir/usr/
mkdir AppDir/usr/bin
make install DESTDIR=AppDir
cp egees.desktop AppDir
cp icon.png AppDir
cp -r css AppDir/usr/bin/
cp -r js AppDir/usr/bin/

# now, build AppImage using linuxdeploy 
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage

# make them executable
chmod +x linuxdeploy-x86_64.AppImage


# initialize AppDir, bundle shared libraries for QtQuickApp, use Qt plugin to bundle additional resources, and build AppImage, all in one single command
./linuxdeploy-x86_64.AppImage --appdir AppDir --output appimage

# move built AppImage back into original CWD
mv *.AppImage "$OLD_CWD"
