start=$EPOCHREALTIME

./build_linux.sh
./build_windows.sh

stop=$EPOCHREALTIME
elapsed=$(bc -l <<< "$stop - $start")
echo "Build took $elapsed seconds"