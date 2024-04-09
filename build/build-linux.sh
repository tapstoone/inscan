
# build linux_x86_x64 with docker
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl \
 cargo build --release