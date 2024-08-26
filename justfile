default:
    just -l


# Run test cases
test:
    cargo test

# Run a pedantic clippy linting check
check:
    cargo clippy -- -W clippy::pedantic
