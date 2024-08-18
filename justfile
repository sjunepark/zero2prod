set dotenv-filename := ".env"

default:
    just -l

watch:
    cargo watch -q -c -s 'just test'

check:
    cargo check --all-features --all-targets

test:
    cargo test --all-features | bunyan

format:
    cargo fmt --all

lint:
    cargo clippy -q --all-targets --all-features

init-db:
    ./scripts/init_db.sh

migrate-add description:
    sqlx migrate add -r {{description}}

migrate-local:
    sqlx migrate run

migrate-local-revert:
    sqlx migrate revert

sqlx-prepare:
    cargo sqlx prepare --workspace
    git diff --exit-code || (echo "Error: sqlx prepare made changes to the code. Please commit them before continuing." && exit 1)

docker-build: sqlx-prepare
    docker build -t zero2prod --file Dockerfile .

doctl-update: sqlx-prepare
    doctl apps update $DO_APP_ID --spec spec.yaml

migrate-do:
    DATABASE_URL=$DO_DATABASE_URL sqlx migrate run

migrate-do-revert:
    DATABASE_URL=$DO_DATABASE_URL sqlx migrate revert

git-push: sqlx-prepare test doctl-update
    git push