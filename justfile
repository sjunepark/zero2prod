default:
    just -l

watch:
    cargo watch -q -c -s 'just test' -s 'just format' -s 'just lint'

test:
    cargo test --all-features

format:
    cargo fmt --all

lint:
    cargo clippy -q --all-targets --all-features -- -D warnings
