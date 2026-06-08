
echo
echo "WINDOWS BUILDS"
echo

echo
echo "DEBUG BUILDS"
echo

cargo xwin build --target x86_64-pc-windows-msvc --package hadalyth-twitch

echo
echo "RELEASE BUILDS"
echo

cargo xwin build --target x86_64-pc-windows-msvc --release --package hadalyth-twitch

