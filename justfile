set dotenv-load := true

check: fmt-check cargo-check test clippy

before-commit: fix check

fix:
    cargo fix --allow-dirty --allow-staged
    cargo fmt --

fmt-check:
    cargo fmt -- --check

cargo-check:
    cargo check

test:
    cargo test --all --no-fail-fast

clippy:
    cargo clippy -- -D warnings -D clippy::pedantic -A clippy::cast_precision_loss -A clippy::cast_possible_truncation -A clippy::cast_possible_wrap -A clippy::cast_sign_loss --verbose --no-deps

update:
    cargo update
