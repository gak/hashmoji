test:
    RUSTFLAGS="-D warnings" cargo test
    RUSTFLAGS="-D warnings" cargo test --lib --no-default-features
    RUSTFLAGS="-D warnings" cargo test --lib --no-default-features --features alloc
    cargo clippy --all-targets --all-features -- -D warnings

example:
    cargo run --example full
