#!/bin/bash

echo $PWD

OLDCFLAGS="${CFLAGS}"
OLDCXXFLAGS="${CXXFLAGS}"

unset CFLAGS
unset CXXFLAGS

cargo update
cargo build --all --release

rm ./scripts/LoLUpdater.app/Contents/MacOS/lolupdater-gui
cp ./target/release/lolupdater-gui ./scripts/LoLUpdater.app/Contents/MacOS/
rm ./dist/LoLUpdater.dmg
create-dmg ./scripts/LoLUpdater.app
mv LoLUpdater-*.dmg ./dist/LoLUpdater.dmg

TMPDIR="$(mktemp -d)"
cp ./LICENSE "$TMPDIR"
cp ./README.MD "$TMPDIR"
cp ./target/release/lolupdater-cli "$TMPDIR"
tar -cvf "./dist/lolupdater-cli.tar" --exclude=".DS_Store" -C "$TMPDIR" .
rm "./dist/lolupdater-cli.tar.gz"
zopfli -v "./dist/lolupdater-cli.tar"
rm "./dist/lolupdater-cli.tar"
rm -rf "$TMPDIR"

if [ "$CI" != "true" ]; then
cargo graph | dot -Tpng > graph.png
fi

export CFLAGS="${OLDCFLAGS}"
export CXXFLAGS="${OLDCXXFLAGS}"
