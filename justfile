default:
    just -l

watch:
    cargo watch -q -c -s 'just test' -s 'just format' -s 'just lint'

test:
    TEST_LOG=true cargo test --all-features | bunyan

format:
    cargo fmt --all

lint:
    cargo clippy -q --all-targets --all-features
