# Debug memory allocation

## Build test file
- 100 MB file: `if=/dev/random of=TESTFILE status=progress bs=1m count=100`

## Steps

1. `cargo install --features vendored-openssl cargo-instruments`  
  Installs _cargo-instruments_ without needing OpenSSL (macOS shenanigans -.-)
2. `cargo instruments -t Allocations (--release)`
3. Find PID
4. `kill -SIGINT [PID]` to stop gracefully without interrupting cargo-instruments.
5. XCode instrument _Allocations_ opens up automatically