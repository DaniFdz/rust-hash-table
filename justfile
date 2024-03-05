run:
    cargo run -q -r

watch:
    cargo watch -q -c -w src/ -x "run -q -r"

test:
    cargo test

test_watch:
    cargo watch -q -c -w src/ -x "test"

test_verbose:
    cargo test --verbose

