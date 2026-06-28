start=$EPOCHREALTIME

./build_only_linux.sh
./build_only_windows.sh

stop=$EPOCHREALTIME
elapsed=$(bc -l <<< "$stop - $start")
echo $elapsed