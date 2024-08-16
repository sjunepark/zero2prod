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

sqlx-prepare:
    cargo sqlx prepare --workspace

docker-build: sqlx-prepare
    docker build -t zero2prod --file Dockerfile .