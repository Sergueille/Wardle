(cd client && python3 -m http.server > /dev/null) &
(sleep 1 && firefox http://localhost:8000/ 2> /dev/null && firefox http://localhost:8000/ 2> /dev/null) &
wait