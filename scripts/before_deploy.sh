# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd)
    local stage
    local target

    if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
        target=x86_64-apple-darwin
        stage=$(mktemp -d)
    elif [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
        target=x86_64-unknown-linux-gnu
        stage=$(mktemp -d -t tmpXXXXXXXXX)
    fi

    mkdir -p $src/deploy
    test -f Cargo.lock || cargo generate-lockfile

    cargo rustc --target $target --release --bin lolupdater-cli --package lolupdater-cli -- -C lto
    cargo rustc --target $target --release --bin lolupdater-gui --package lolupdater-gui -- -C lto

    cp target/$target/release/lolupdater-cli $stage/
    cp README.md $stage/

    cd $stage
    tar czf $src/deploy/lolupdater-cli-$target.tar.gz *
    cd $src

    rm -rf $stage

    if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
        brew install node || true
        npm install --global create-dmg
        rm scripts/LoLUpdater.app/Contents/MacOS/lolupdater-gui || true
        cp target/$target/release/lolupdater-gui scripts/LoLUpdater.app/Contents/MacOS/
        rm deploy/LoLUpdater.dmg || true
        create-dmg ./scripts/LoLUpdater.app
        mv ./LoLUpdater-*.dmg deploy/LoLUpdater.dmg
    elif [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
        stage=$(mktemp -d)

        cp target/$target/release/lolupdater-gui $stage/
        cp README.md $stage/

        cd $stage
        tar czf $src/deploy/lolupdater-gui-$target.tar.gz *
        cd $src

        rm -rf $stage
    fi
}

main
