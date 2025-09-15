#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@10.0.0.133
readonly TARGET_PATH=/home/pi/webby-s
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/webby-s

readonly PACKAGE=out

cargo build --release

rm -rf ${PACKAGE}
mkdir ${PACKAGE}
mv ${SOURCE_PATH} ./${PACKAGE}
cp -r pages ./${PACKAGE}
cp startup.sh ./${PACKAGE}
rsync -r ${PACKAGE}/ ${TARGET_HOST}:${TARGET_PATH}