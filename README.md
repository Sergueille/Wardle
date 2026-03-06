
# Wardle

## Test locally

### Start the backend server

`./local_server`

### Start the frontend server

`./local_client`

## Send files to server

- Make sure the server is on
- Make sur your git working directory is clean
- Create `server_versions.txt`:
    - Each line contains the name of the git branch to be uploaded
    - The file must contain the `main` branch
- Create `server_versions.txt`: must contain the name of the version that will be displayed for the main branch
- `cd scripts`
- `./update_server_files.sh`
- **Press `^C`** after caddy has started (the script tells you when)! For some reason the caddy command blocks forever

