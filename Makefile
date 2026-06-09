MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := caching
.DELETE_ON_ERROR:
.SUFFIXES:

# Comment out CARGO_FLAGS to run dev target
CARGO_FLAGS = --release
SEED = 0
ITERATIONS = 2500
NOISE_THRESHOLD = 5
WARMUP = 10
VERSION = v0.1.0

COMMON_ARGS = --seed $(SEED) --collector silent --iterations $(ITERATIONS) --warmup $(WARMUP)
CACHING_ARGS = --noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression
BASELINE_ARGS = --baseline-dir caching_baselines

.PHONY: caching caching_baselines

caching:
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-none-$(VERSION).json \
		caching sqlite none
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-truncate_all-$(VERSION).json \
		caching sqlite truncate_all
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-truncate-$(VERSION).json \
		caching sqlite truncate
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-trigger-$(VERSION).json \
		caching sqlite trigger
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-memory-$(VERSION).json \
		caching sqlite "memory:1000"
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-none-$(VERSION).json \
		caching postgresql none
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-truncate_all-$(VERSION).json \
		caching postgresql truncate_all
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-truncate-$(VERSION).json \
		caching postgresql truncate
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-trigger-$(VERSION).json \
		caching postgresql trigger
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-memory-$(VERSION).json \
		caching postgresql "memory:1000"

caching_baselines:
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline sqlite-none-$(VERSION) caching sqlite none
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline sqlite-truncate_all-$(VERSION) caching sqlite truncate_all
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline sqlite-truncate-$(VERSION) caching sqlite truncate
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline sqlite-trigger-$(VERSION) caching sqlite trigger
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline sqlite-memory-$(VERSION) caching sqlite "memory:1000"
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline postgresql-none-$(VERSION) caching postgresql none
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline postgresql-truncate_all-$(VERSION) caching postgresql truncate_all
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline postgresql-truncate-$(VERSION) caching postgresql truncate
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline postgresql-trigger-$(VERSION) caching postgresql trigger
	cargo run $(CARGO_FLAGS) -- $(COMMON_ARGS) $(BASELINE_ARGS) \
		--save-baseline postgresql-memory-$(VERSION) caching postgresql "memory:1000"
