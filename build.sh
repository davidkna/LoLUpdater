#!/bin/bash

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
tar -cvf "./dist/lolupdater.tar" --exclude=".DS_Store" "$TMPDIR"
rm "./dist/lolupdater.tar.gz"
zopfli -v "./dist/lolupdater.tar"
rm "./dist/lolupdater.tar"
rm -rf "$TMPDIR"

cargo graph | dot -Tpng > graph.png
