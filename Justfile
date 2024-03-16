set quiet := true

[private]
help:
    just --list --unsorted

build:
    cargo build --release

generate-completions: build
    mkdir -p completions
    docopt-compgen target/release/repo --namespace '' > completions/repo.bash

fmt:
    just --fmt
