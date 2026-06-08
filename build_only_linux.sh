echo
echo "LINUX BUILDS"
echo

echo
echo "DEBUG BUILDS"
echo

GLIBC_VERSION="2.28"
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --package hadalyth-twitch

echo
echo "RELEASE BUILDS"
echo

GLIBC_VERSION="2.28"
cargo zigbuild --target x86_64-unknown-linux-gnu.$GLIBC_VERSION --release --package hadalyth-twitch

