default:
    just -l


# Run test cases
test:
    cargo test -q

# Run a pedantic clippy linting check
check:
    cargo clippy --all-targets -- -W clippy::pedantic
