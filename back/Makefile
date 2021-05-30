TARGET ?= x86_64-pc-windows-gnu

build:
	@cargo build

clean:
	@cargo clean

TESTS = ""
test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	cargo run -- --ip 127.0.0.1 --port 8000

serve: 
	cargo run -- --ip 192.168.137.1 --port 8000

prod:
	cargo build --target $(TARGET) --release
	@echo "Executable is at target/$(TARGET)/release"
	cp -r www target/$(TARGET)/release
	@echo "Copied www dir near the executable. THIS DIRECTORY MUST STAY WITH IT"

.PHONY: build test docs style-check lint
