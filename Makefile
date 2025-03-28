CARGO := cargo
CLIPPY := $(CARGO) clippy --all-targets --all-features -- -D warnings
FMT := $(CARGO) fmt -- --check

.PHONY: build run test lint format check-format clean check fix-lint

build:
	$(CARGO) build

run:
	$(CARGO) run

test:
	$(CARGO) test

lint:
	$(CLIPPY)

format:
	$(CARGO) fmt

check-format:
	$(FMT)

clean:
	$(CARGO) clean

check: lint check-format build

fix-lint:
	$(CARGO) clippy --fix --allow-dirty --allow-staged

