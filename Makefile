CARGO := cargo
CLIPPY := $(CARGO) clippy --all-targets --all-features -- -D warnings
FMT := $(CARGO) fmt -- --check

.PHONY: build run test lint format check-format clean check fix-lint

build:
	$(CARGO) build

run:
	APP_ENV=dev $(CARGO) run $(ARGS)

test:
	APP_ENV=test $(CARGO) test -- --test-threads=1

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

