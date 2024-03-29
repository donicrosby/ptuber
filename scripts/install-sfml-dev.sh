#!/usr/bin/env bash

SFML_VERSION="2.5.1"
GCC_VERSION="7.3.0"
ARCH="64"
ZIP="SFML-${SFML_VERSION}-windows-gcc-${GCC_VERSION}-mingw-${ARCH}-bit.zip"

TEMP_DIR="/tmp/sfml"
LIBS_DIR=""

apt-get update && apt-get install -y zip apt-utils wget;
cp -f /mingw64/x86_64-w64-mingw32/lib/{*.a,*.o} `rustc --print sysroot`/lib/rustlib/x86_64-pc-windows-gnu/lib/self-contained

mkdir ${TEMP_DIR}
cd ${TEMP_DIR}
wget "https://www.sfml-dev.org/files/${ZIP}"
unzip ${ZIP}
mv SFML-${SFML_VERSION} /SFML

rm -rf ${TEMP_DIR}

