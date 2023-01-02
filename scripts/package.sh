#!/bin/bash
PACKAGE_PATH=${1}

export DLLS=`peldd ${PACKAGE_PATH}/*.exe -p /usr/lib/gcc/x86_64-w64-mingw32/9.3-win32/ -t --ignore-errors`
for DLL in $DLLS
    do cp "$DLL" ${PACKAGE_PATH}/
done