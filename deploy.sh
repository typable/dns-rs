#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly HOST="typable@git.typable.dev"
readonly TARGET="bin/dns-rs"
readonly ARCH="armv7-unknown-linux-gnueabihf"
readonly SOURCE="./target/$ARCH/release/dns-rs"

cargo build --release --target $ARCH

scp $SOURCE "$HOST:$TARGET"
