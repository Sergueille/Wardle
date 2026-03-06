
port=4269

caddy_proxy_command=""

while read branch || [[ -n $branch ]]; do
  if [ $branch == main ]; then
    branch_path=
  else
    branch_path=$branch/
  fi

  # Create the systemd unit
  echo "
[Unit]
Description=Wardle backend for version $branch (automatically generated)

[Service]
ExecStart=/opt/server/backend/$branch/server --port $port
Restart=on-failure

[Install]
WantedBy=multi-user.target  
"> /etc/systemd/system/wardle-backend-$branch.service

  # Doing that because we're told to do so
  systemctl daemon-reload

  # Start the unit
  systemctl start wardle-backend-$branch.service

  # Add a proxy command to caddy
  caddy_proxy_command="$caddy_proxy_command
  handle /api/$branch/* {
    uri strip_prefix /api/$branch
    reverse_proxy localhost:$port
  }"

  port=$((port+1))

done </opt/server_versions.txt

# Tell caddy about the new config
echo "
:80 {
  root * /root/client/www
  file_server
  $caddy_proxy_command
}
" > /opt/server/client/Caddyfile

cd /opt/server/client
systemctl stop caddy
caddy stop
caddy adapt
caddy start --config Caddyfile

