
# Stop the backends units
ls /etc/systemd/system | grep wardle | while read p; do
  systemctl stop $p
done

# Clear the frontend directory
rm -r /opt/server/client/www

while read branch || [[ -n $branch ]]; do
  if [ $branch == main ]; then
    branch_path=
  else
    branch_path=$branch/
  fi

  # Create the folders
  mkdir -p /opt/server/backend/$branch
  mkdir -p /opt/server/client/www/$branch_path

done </opt/server_versions.txt


