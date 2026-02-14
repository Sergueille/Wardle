
# Wardle

## Test locally

### Start the backend server

- Install rust
- go to `server` folder
- run `cargo run -- --localhost`

### Start the frontend server

- go to `client` folder
- start a local server (for instance you can use the python test server: `python -m http.server`, which will serve files on `https://localhost:8000/`)

## Send files to server

- reboot the server
- run `deploy_to_server.sh`
- follow [Start the server](#start-the-server) steps

## Start the server

- make sure server is on
- start an ssh session
- `systemctl start wardle-backend.service`
- `systemctl start caddy`

