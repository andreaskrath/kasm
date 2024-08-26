default:
    just -l

test:
    cargo test

check:
    cargo clippy -- -W clippy::pedantic
