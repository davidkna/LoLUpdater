# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    local target
    if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
        target=x86_64-apple-darwin
    elif [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
        target=x86_64-unknown-linux-gnu
    fi

    cargo build --target $target
    cargo build --target $target --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cargo test --target $target
    cargo test --target $target --release

}

if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
    brew update
    brew install cmake || true
fi

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
