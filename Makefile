.PHONY: fmt-check clippy test build run-smoke run smoke

fmt-check:
	@rustup component add rustfmt >/dev/null 2>&1 || true
	cargo fmt -- --check

clippy:
	@rustup component add clippy >/dev/null 2>&1 || true
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

build:
	cargo build --release --workspace --all-features

run:
	PORT=$${PORT:-8080} cargo run

run-smoke:
	@PORT=$${PORT:-8080} ; \
	echo "Starting server on $$PORT in background..." ; \
	PORT=$$PORT cargo run &> server.log & echo $$! > .server_pid ; \
	sleep 2 ; \
	./test/scripts/smoke.sh http://localhost:$$PORT ; \
	kill $$(cat .server_pid) || true ; rm -f .server_pid

smoke:
	./test/scripts/smoke.sh http://localhost:$${PORT:-8080}
