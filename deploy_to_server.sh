
echo "test version $(git rev-parse --short HEAD)" > client/version.txt

# Backend
cd server
cargo build --release
scp target/release/wardle-server root@\[2a09:6847:fa10:1410::278\]:/opt/server/backend/server
cd ..

# Frontend
scp -r client/* root@\[2a09:6847:fa10:1410::278\]:/opt/server/client/www
scp -r client/index.html root@\[2a09:6847:fa10:1410::278\]:/opt/server/client/www/test.html
