test:
    RUSTFLAGS="-D warnings" cargo test
    RUSTFLAGS="-D warnings" cargo test --lib --no-default-features
    RUSTFLAGS="-D warnings" cargo test --lib --no-default-features --features alloc
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features additive,face-sleepy
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features additive,travel-and-places
    cargo clippy --all-targets --all-features -- -D warnings

example:
    cargo run --example full
