#!/bin/bash

mkdir -p ./dist

OLDCFLAGS="${CFLAGS}"
OLDCXXFLAGS="${CXXFLAGS}"

unset CFLAGS
unset CXXFLAGS

cargo update
cargo build --all --release

rm ./scripts/LoLUpdater.app/Contents/MacOS/lolupdater-gui || true
cp ./target/release/lolupdater-gui ./scripts/LoLUpdater.app/Contents/MacOS/
rm ./dist/LoLUpdater.dmg || true
create-dmg ./scripts/LoLUpdater.app
mv ./LoLUpdater-*.dmg ./dist/LoLUpdater.dmg

TMPDIR="$(mktemp -d)"
cp ./LICENSE "$TMPDIR"
cp ./README.MD "$TMPDIR"
cp ./target/release/lolupdater-cli "$TMPDIR"
rm "./dist/lolupdater-cli-x86_64-apple-darwin.tar.bz2" || true
tar cjvf "./dist/lolupdater-cli-x86_64-apple-darwin.tar.bz2" --exclude=".DS_Store" -C "$TMPDIR" .
rm -rf "$TMPDIR"

export CFLAGS="${OLDCFLAGS}"
export CXXFLAGS="${OLDCXXFLAGS}"
