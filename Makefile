run:
	RUST_ENV=development  cargo run

watch:
	RUST_ENV=development   cargo watch -x run

watch-test:
	RUST_ENV=testing  cargo watch -x test

test:
	RUST_ENV=testing cargo test
