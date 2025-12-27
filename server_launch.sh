
# This script is in the server and is called on boot
./server/server &
./client/http-file-server -p 80 /=client/www

