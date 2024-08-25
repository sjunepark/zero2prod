set dotenv-filename := ".env"

# For development

git-push: sqlx-prepare test doctl-update
    git push

default:
    just -l

watch:
    cargo watch -q -s -c -s 'just test'

check:
    cargo check --all-features --all-targets

test:
    RUST_LOG="sqlx=error,warn" cargo test --all-features

test-scoped target:
    RUST_LOG="sqlx=error,info" cargo test {{target}}

format:
    cargo fmt --all

lint:
    cargo clippy -q --all-targets --all-features

# Manual database operations

init-db:
    ./scripts/init_db.sh

sqlx-prepare:
    #!/usr/bin/env bash
    set -euo pipefail

    # Run sqlx prepare
    cargo sqlx prepare --workspace

    # Check if there are any changes after running sqlx prepare
    if [[ -n $(git status --porcelain) ]]; then
        echo "Error: sqlx prepare made changes to the code. Please review and commit these changes before continuing."
        git status --short
        exit 1
    else
        echo "sqlx prepare completed successfully with no changes to commit."
    fi

migrate-add description:
    sqlx migrate add -r {{description}}

migrate-local:
    sqlx migrate run

migrate-local-revert:
    sqlx migrate revert

migrate-do-revert:
    DATABASE_URL=$DO_DATABASE_URL sqlx migrate revert

# Deployment related operations

docker-build: sqlx-prepare
    docker build -t zero2prod --file Dockerfile .

doctl-update: sqlx-prepare
    doctl apps update $DO_APP_ID --spec spec.yaml

migrate-do:
    DATABASE_URL=$DO_DATABASE_URL sqlx migrate run