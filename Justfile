import 'scripts/mod.just'

set shell := ["bash", "-uc"]

@_default:
    just --list --no-aliases

bin-size:
    cargo build --release --target x86_64-unknown-linux-musl
    stat -c%s target/x86_64-unknown-linux-musl/release/fife | tee -a bin_size.txt
    cargo nextest run --target x86_64-unknown-linux-musl
    cargo run --release --target x86_64-unknown-linux-musl -- -r . -m test -p 9193
