# Derived from https://github.com/japaric/trust

set -ex

# ----- Options -----

# TARGET enables cross-building
if [ -z $TARGET ]; then
    CARGO=cargo
elif [ "$TARGET" = "i686-unknown-linux-musl" ]; then
    CARGO=cargo
    TARGET="--target $TARGET"
else
    CARGO=cross
    TARGET="--target $TARGET"
fi

# ALLOC defaults on; is disabled for rustc < 1.36
if [ -z $ALLOC ]; then
    ALLOC=1
fi

# NIGHTLY defaults off


# ----- Script -----

main() {
  cd ./hello-world
  $CARGO build $TARGET --verbose
  $CARGO test $TARGET
}