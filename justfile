test:
    RUSTFLAGS="-D warnings" cargo test
    RUSTFLAGS="-D warnings" cargo test --lib --no-default-features
    RUSTFLAGS="-D warnings" cargo test --lib --no-default-features --features alloc
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,face-sleepy | grep -v 😴
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,additive,face-sleepy | grep 😴
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,additive,face-sleepy | grep -v 🚗
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,travel-and-places | grep -v 🚗
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,travel-and-places | grep 🌈
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,additive,travel-and-places | grep 🚗
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,additive,travel-and-places | grep -v 👌
    cargo clippy --all-targets --all-features -- -D warnings

example:
    cargo run --example full
