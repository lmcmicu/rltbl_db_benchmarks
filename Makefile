MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := caching
.DELETE_ON_ERROR:
.SUFFIXES:

SEED = 0
WARMUP = 100
NOISE_THRESHOLD = 5
REGRESSION_METRICS = iters-rate,latency-mean
VERSION = v0.1.0

COMMON_ARGS = \
	--seed $(SEED) \
	--collector silent \
	--warmup $(WARMUP) \
	--totals-file caching_baselines/totals-$(VERSION).json
CACHING_ARGS = --fail-on-regression --regression-metrics $(REGRESSION_METRICS) \
	--noise-threshold $(NOISE_THRESHOLD)
BASELINE_ARGS = --baseline-dir caching_baselines

.PHONY: caching caching_baselines

caching:
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-none-$(VERSION).json --duration 1m \
		caching sqlite none
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-none-$(VERSION).json --duration 1m \
		caching postgresql none
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-truncate_all-$(VERSION).json --duration 1m \
		caching sqlite truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-truncate_all-$(VERSION).json --duration 1m \
		caching postgresql truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-truncate-$(VERSION).json --duration 1m \
		caching sqlite truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-truncate-$(VERSION).json --duration 1m \
		caching postgresql truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-trigger-$(VERSION).json --duration 1m \
		caching sqlite trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-trigger-$(VERSION).json --duration 1m \
		caching postgresql trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/sqlite-memory-$(VERSION).json --duration 1m \
		caching sqlite "memory:1000"
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file caching_baselines/postgresql-memory-$(VERSION).json --duration 1m \
		caching postgresql "memory:1000"

caching_baselines:
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m 
		--save-baseline sqlite-none-$(VERSION) \
		caching sqlite none
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m \
		--save-baseline postgresql-none-$(VERSION) \
		caching postgresql none
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m \
		--save-baseline sqlite-truncate_all-$(VERSION) \
		caching sqlite truncate_all
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m \
		--save-baseline postgresql-truncate_all-$(VERSION) \
		caching postgresql truncate_all
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m \
		--save-baseline sqlite-truncate-$(VERSION) \
		caching sqlite truncate
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m \
		--save-baseline postgresql-truncate-$(VERSION) \
		caching postgresql truncate
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m \
		--save-baseline sqlite-trigger-$(VERSION) \
		caching sqlite trigger
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m \
		--save-baseline postgresql-trigger-$(VERSION) \
		caching postgresql trigger
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m \
		--save-baseline sqlite-memory-$(VERSION) \
		caching sqlite "memory:1000"
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) --duration 1m \
		--save-baseline postgresql-memory-$(VERSION) \
		caching postgresql "memory:1000"
