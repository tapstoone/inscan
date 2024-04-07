#!/usr/bin/env bash

set -euxo pipefail

VERSION=${REF#"refs/tags/"}
DIST=`pwd`/dist

echo "Packaging inscan $VERSION for $TARGET..."

test -f Cargo.lock || cargo generate-lockfile

echo "Building inscan..."
RUSTFLAGS="$TARGET_RUSTFLAGS" \
  cargo build --bin inscan --target $TARGET --release
EXECUTABLE=target/$TARGET/release/inscan

if [[ $OS == windows-latest ]]; then
  EXECUTABLE=$EXECUTABLE.exe
fi

echo "Copying release files..."
mkdir -p dist/inscan-$VERSION
cp \
  $EXECUTABLE \
  Cargo.lock \
  Cargo.toml \
  readme.md \
  $DIST/inscan-$VERSION

cd $DIST
echo "Creating release archive..."
case $OS in
  ubuntu-latest | macos-latest)
    ARCHIVE=$DIST/inscan-$VERSION-$TARGET.tar.gz
    tar czf $ARCHIVE *
    echo "::set-output name=archive::$ARCHIVE"
    ;;
  windows-latest)
    ARCHIVE=$DIST/inscan-$VERSION-$TARGET.zip
    7z a $ARCHIVE *
    echo "::set-output name=archive::`pwd -W`/inscan-$VERSION-$TARGET.zip"
    ;;
esac