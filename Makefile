run:
	RUST_ENV=debug cargo run

watch:
	RUST_ENV=debug cargo watch -x run

test:
	RUST_ENV=testing cargo test
