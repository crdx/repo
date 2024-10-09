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
    find . -name '*.just' -print0 | xargs -0 -I{} just --fmt -f {}
