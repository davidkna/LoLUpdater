#!/bin/bash

OLDCFLAGS="${CFLAGS}"
OLDCXXFLAGS="${CXXFLAGS}"

unset CFLAGS
unset CXXFLAGS

cargo update
cargo build --release

rm ./dist/LoLUpdater.app/Contents/MacOS/lolupdater-gui
cp ./target/release/lolupdater-gui ./dist/LoLUpdater.app/Contents/MacOS/
create-dmg ./dist/LoLUpdater.app
mv LoLUpdater-*.dmg ./dist

TMPDIR="$(mktemp -d)"
cp ./LICENSE "$TMPDIR"
cp ./README.MD "$TMPDIR"
cp ./target/release/lolupdater-cli "$TMPDIR"
tar -cvf "./dist/lolupdater-cli.tar" --exclude=".DS_Store" -C "$TMPDIR" .
rm "./dist/lolupdater-cli.tar.gz"
zopfli -v "./dist/lolupdater-cli.tar"
rm "./dist/lolupdater-cli.tar"
rm -rf "$TMPDIR"

cargo graph | dot -Tpng > graph.png

export CFLAGS="${OLDCFLAGS}"
export CXXFLAGS="${OLDCXXFLAGS}"
