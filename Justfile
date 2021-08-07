@_help:
    just --list --unsorted

# build in release mode
@build:
    cargo build --release

# generate shell completions
@generate-completions: build
    mkdir -p completions
    generate-completions target/release/repo --namespace '' > completions/repo.bash
