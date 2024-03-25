rustup target add wasm32-unknown-unknown
rustup toolchain add nightly-x86_64-unknown-linux-gnu
rustup component add rustfmt
rustup component add clippy
cargo install cargo-edit cargo-tarpaulin cargo-expand cargo-audit cargo-udeps