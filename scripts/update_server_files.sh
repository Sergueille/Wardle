
echo "Setup..."
scp ../server_versions.txt root@[2a09:6847:fa10:1410::278]:/opt/server_versions.txt >/dev/null
ssh root@2a09:6847:fa10:1410::278 < _before_upload.sh >/dev/null

current_branch="$(git rev-parse --abbrev-ref HEAD)"

while read branch || [[ -n $branch ]]; do
  if [ $branch == production ]; then
    branch_path=
  else
    branch_path=$branch/
  fi
  
  # Set up the files
  git checkout $branch >/dev/null

  if [ $branch == production ]; then
    echo "$(cat ../production_version.txt) ($(git rev-parse --short HEAD))" > ../client/version.txt
  else
    echo "$branch ($(git rev-parse --short HEAD))" > ../client/version.txt
  fi

  # Compile and send the executable
  cd ../server
  cargo build --release >/dev/null
  scp target/release/wardle-server root@\[2a09:6847:fa10:1410::278\]:/opt/server/backend/$branch/server >/dev/null
  cd ../scripts

  echo "Uploading files..."

  # Send frontend files
  scp -r ../client/* root@\[2a09:6847:fa10:1410::278\]:/opt/server/client/www/$branch_path >/dev/null

done <../server_versions.txt

echo "Restarting everything..."

ssh root@2a09:6847:fa10:1410::278 < _after_upload.sh >/dev/null


git checkout $current_branch # Switch back to the branch used before

