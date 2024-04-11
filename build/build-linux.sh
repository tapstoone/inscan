
# build linux_x86_x64 with docker
# output path: target/x86_64-unknown-linux-musl/release/
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl \
 cargo build --release