# Justfile

fmt:
    cargo +nightly fmt --all

clippy:
    cargo +nightly clippy

clean:
    cargo clean

gitclean:
    git clean -dfx

test:
    #!/usr/bin/env sh
    if cargo nextest --help &> /dev/null; then
        # If successful, run 'cargo nextest run'
        cargo nextest run
    else
        # If not successful, run 'cargo test'
        cargo test
    fi