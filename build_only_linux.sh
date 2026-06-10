echo
echo "LINUX BUILDS"
echo

BASE_DIR=$(cd -- "$(dirname -- "${B_SOURCE[0]}")" &> /dev/null && pwd)
NDI_SO="$BASE_DIR/api/ndi_sdk/lib/x86_64-linux-gnu/libndi.so"

echo
echo "DEBUG BUILDS"
echo

GLIBC_VERSION="2.28"
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --package hadalyth-twitch
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --package hadalyth-ndi
cp -L "$NDI_SO" "$BASE_DIR/target/x86_64-unknown-linux-gnu/debug/"

echo
echo "RELEASE BUILDS"
echo

GLIBC_VERSION="2.28"
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --release --package hadalyth-twitch
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --release --package hadalyth-ndi
cp -L "$NDI_SO" "$BASE_DIR/target/x86_64-unknown-linux-gnu/release/"

