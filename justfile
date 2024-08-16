set dotenv-filename := ".env"

default:
    just -l

watch:
    cargo watch -q -c -s 'just test' -s 'just format' -s 'just lint'

test:
    cargo test --all-features | bunyan

format:
    cargo fmt --all

lint:
    cargo clippy -q --all-targets --all-features

init-db:
    ./scripts/init_db.sh

migrate-db:
    sqlx migrate run

migrate-db-revert:
    sqlx migrate revert

sqlx-prepare:
    cargo sqlx prepare --workspace

docker-build: sqlx-prepare
    docker build -t zero2prod --file Dockerfile .

doctl-update:
    doctl apps update $DO_APP_ID --spec spec.yaml

do-migrate:
    DATABASE_URL=$DO_DATABASE_URL sqlx migrate run