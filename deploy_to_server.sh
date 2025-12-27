
# Backend
cd server
cargo build --release
scp target/release/wardle-server root@\[2a09:6847:fa10:1410::278\]:/root/server/server
cd ..

# Frontend
scp -r client root@\[2a09:6847:fa10:1410::278\]:/root/client/www
