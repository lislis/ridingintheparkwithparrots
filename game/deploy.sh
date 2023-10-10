#!/bin/bash

# https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

# adjust these as needed
# assumes pub key auth is set up
readonly TARGET_HOST=lislis@raspberrypi.local
readonly TARGET_PATH=/home/lislis/ridingintheparkwithparrots
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/ridingintheparkwithparrots
readonly PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig
#PKG_CONFIG

cross build --target=${TARGET_ARCH}
# cargo build --release --target=${TARGET_ARCH}
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}