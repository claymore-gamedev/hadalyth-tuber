
echo
echo "WINDOWS BUILDS"
echo

BASE_DIR=$(cd -- "$(dirname -- "${B_SOURCE[0]}")" &> /dev/null && pwd)
NDI_DLL="$BASE_DIR/api/ndi_sdk/lib/x86_64-pc-windows-msvc/ndi.dll"

echo
echo "DEBUG BUILDS"
echo

cargo xwin build --target x86_64-pc-windows-msvc --package hadalyth-arkit
cargo xwin build --target x86_64-pc-windows-msvc --package hadalyth-twitch
cargo xwin build --target x86_64-pc-windows-msvc --package hadalyth-networking
cargo xwin build --target x86_64-pc-windows-msvc --package hadalyth-ndi
cp "$NDI_DLL" "$BASE_DIR/target/x86_64-pc-windows-msvc/debug/"

echo
echo "RELEASE BUILDS"
echo

cargo xwin build --target x86_64-pc-windows-msvc --release --package hadalyth-arkit
cargo xwin build --target x86_64-pc-windows-msvc --release --package hadalyth-twitch
cargo xwin build --target x86_64-pc-windows-msvc --release --package hadalyth-networking
cargo xwin build --target x86_64-pc-windows-msvc --release --package hadalyth-ndi
cp "$NDI_DLL" "$BASE_DIR/target/x86_64-pc-windows-msvc/release/"
