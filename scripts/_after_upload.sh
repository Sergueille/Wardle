
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
ExecStart=/opt/server/backend/$branch/server
Restart=on-failure

[Install]
WantedBy=multi-user.target  
"> /etc/systemd/system/wardle-backend-$branch.service

  # Doing that because we're told to do so
  systemctl daemon-reload

  # Start the unit
  systemctl start wardle-backend-$branch.service

done </opt/server_versions.txt


