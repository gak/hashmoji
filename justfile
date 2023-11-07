test:
    RUSTFLAGS="-D warnings" cargo test
    RUSTFLAGS="-D warnings" cargo test --lib --no-default-features
    RUSTFLAGS="-D warnings" cargo test --lib --no-default-features --features alloc

    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,face-sleepy | grep -v 😴
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,additive,face-sleepy | grep 😴 | grep -v 🚗
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,travel-and-places | grep 🌈 | grep -v 🚗
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,additive,travel-and-places | grep 🚗 | grep -v 👌

    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,v0006 | grep 🚀 | grep -v 🏳️
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,v1200 | grep 🥱 | grep -v 🧑‍🦰

    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,skin-tones | grep 🖖 | grep -v 🖖🏻
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,additive,person,skin-tones | grep 👶🏽

    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,genders | grep 🧔 | grep -v 🧔‍♀️
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,additive,person | grep 🧔 | grep -v 🧔‍♀️
    RUSTFLAGS="-D warnings" cargo run --example iter --no-default-features --features std,additive,person,genders | grep 👧 | grep 🧔‍♀️

    cargo clippy -- -D warnings
    cargo clippy --no-default-features -- -D warnings
    cargo clippy --no-default-features --features alloc -- -D warnings

examples:
    cargo run --example iter
    cargo run --example full
