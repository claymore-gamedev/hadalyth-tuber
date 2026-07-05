start=$EPOCHREALTIME

echo
echo "LINUX BUILDS"
echo

BASE_DIR=$(cd -- "$(dirname -- "${B_SOURCE[0]}")" &> /dev/null && pwd)
NDI_SO="$BASE_DIR/api/ndi_sdk/lib/x86_64-linux-gnu/libndi.so"

GLIBC_VERSION="2.41"

echo
echo "DEBUG BUILDS"
echo

cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --package hadalyth-arkit
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --package hadalyth-ndi
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --package hadalyth-networking
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --package hadalyth-twitch
cp -L "$NDI_SO" "$BASE_DIR/target/x86_64-unknown-linux-gnu/debug/"

echo
echo "RELEASE BUILDS"
echo

cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --release --package hadalyth-arkit
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --release --package hadalyth-ndi
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --release --package hadalyth-networking
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --release --package hadalyth-twitch
cp -L "$NDI_SO" "$BASE_DIR/target/x86_64-unknown-linux-gnu/release/"

stop=$EPOCHREALTIME
elapsed=$(bc -l <<< "$stop - $start")
echo "Build took $elapsed seconds"