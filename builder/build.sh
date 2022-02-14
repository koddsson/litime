#!/usr/bin/env bash

info() {
    echo "::info $*" >&2
}

error() {
    echo "::error file=build.sh:: $*" >&2
}

set -eu -o pipefail

crash() {
    error "Command exited with non-zero exit code"
    exit 1
}

trap 'crash' ERR
OUTPUT_DIR="$1"
RUSTTARGET="$2"

info "Installing additional linkers"
case ${RUSTTARGET} in
    "x86_64-pc-windows-gnu") ;;

    "x86_64-unknown-linux-musl") ;;

    "x86_64-apple-darwin")
        export CC=/opt/osxcross/target/bin/o64-clang
        export CXX=/opt/osxcross/target/bin/o64-clang++
        export PATH="/opt/osxcross/target/bin:$PATH"
        export LIBZ_SYS_STATIC=1
        mkdir -p /.cargo
        cat > /.cargo/config.toml << EOF
[target.x86_64-apple-darwin]
linker = "/opt/osxcross/target/bin/x86_64-apple-darwin14-clang"
ar = "/opt/osxcross/target/bin/x86_64-apple-darwin14-ar"
EOF
        ;;

    *)
        error "${RUSTTARGET} is not supported"
        exit 1
        ;;
esac

info "Setting up toolchain"
TOOLCHAIN_VERSION="${TOOLCHAIN_VERSION:-""}"
if [ "$TOOLCHAIN_VERSION" != "" ]; then
    rustup default "$TOOLCHAIN_VERSION" >&2
fi
rustup target add "$RUSTTARGET" >&2

BINARIES="$(cargo read-manifest | jq -r ".targets[] | select(.kind[] | contains(\"bin\")) | .name")"

OUTPUT_LIST=""
for BINARY in $BINARIES; do
    info "Building $BINARY..."

    OPENSSL_LIB_DIR=/usr/lib OPENSSL_INCLUDE_DIR=/usr/include/openssl CARGO_TARGET_DIR="./target" cargo build --release --target "$RUSTTARGET" --bin "$BINARY" >&2
    OUTPUT=$(find "target/${RUSTTARGET}/release/" -maxdepth 1 -type f -executable \( -name "${BINARY}" -o -name "${BINARY}.exe" \) -print0 | xargs -0)

    info "$OUTPUT"

    if [ "$OUTPUT" = "" ]; then
        error "Unable to find output"
        exit 1
    fi

    info "Minifying ${OUTPUT}..."

    info "Stripping..."
    strip "${OUTPUT}" >&2 || info "Strip failed."
    info "File stripped successfully."

    info "Saving $OUTPUT..."

  # shellcheck disable=SC2086
  mv $OUTPUT "$OUTPUT_DIR" || error "Unable to copy binary"

  for f in $OUTPUT; do
      OUTPUT_LIST="$OUTPUT_LIST $(basename "$f")"
  done
done
echo "$OUTPUT_LIST"
