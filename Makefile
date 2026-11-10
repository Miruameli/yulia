# Yulia — standard developer tasks.
#
# Spec: docs/specs/2026-11-09-yulia-benchmark-methodology.md §4.3 requires the
# benchmark suite to live in `benchmarks/` and be reproducible via
# `make benchmark`.

CARGO ?= cargo
# SOP-RESLIM: heavy targets run under an explicit `timeout -k 5 <seconds>`.
# Requires GNU coreutils (present on Linux CI; on macOS `brew install coreutils`
# and pass `TIMEOUT=gtimeout`). Override per invocation, e.g. `make test TIMEOUT=`.
TIMEOUT ?= timeout -k 5

.PHONY: help fmt fmt-check lint test test-nextest bench benchmark coverage codegraph codegraph-sync doc build deny typos ci ci-full clean

help: ## List available targets
	@grep -E '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  %-16s %s\n", $$1, $$2}'

fmt: ## Format all Rust code
	$(CARGO) fmt --all

fmt-check: ## Verify formatting without writing
	$(CARGO) fmt --all -- --check

lint: ## Run clippy, denying warnings
	$(TIMEOUT) 600 $(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

test: ## Run the test suite
	$(TIMEOUT) 900 $(CARGO) test --workspace --all-features

test-nextest: ## Run the test suite with cargo-nextest
	$(TIMEOUT) 900 $(CARGO) nextest run --workspace --all-features

bench: benchmark ## Alias of `benchmark`

benchmark: ## Run the benchmark suite (spec §4.3)
	@$(TIMEOUT) 1800 bash benchmarks/run.sh

coverage: ## Run tests with LLVM coverage into coverage/
	$(TIMEOUT) 900 $(CARGO) llvm-cov --workspace --all-features --html --output-dir coverage

codegraph: ## Rebuild the codegraph index from scratch
	@$(TIMEOUT) 600 codegraph index

codegraph-sync: ## Incrementally update the codegraph index
	@$(TIMEOUT) 600 codegraph sync

doc: ## Build the rustdoc documentation
	$(TIMEOUT) 600 $(CARGO) doc --workspace --no-deps

build: ## Build the compiler in release mode
	$(TIMEOUT) 600 $(CARGO) build --workspace --all-features --release

deny: ## Run the dependency audit (needs `cargo-deny`)
	$(TIMEOUT) 300 cargo deny --config deny.toml check

typos: ## Run the spell checker (needs `typos-cli`)
	$(TIMEOUT) 300 typos

ci: fmt-check lint test build ## Fast local checks (no extra tools)
	@echo "local CI (fast): OK"

ci-full: ci deny typos codegraph ## Everything CI runs, locally (needs cargo-deny, typos-cli)
	@echo "local CI (full): OK"

clean: ## Remove build artifacts
	$(CARGO) clean
