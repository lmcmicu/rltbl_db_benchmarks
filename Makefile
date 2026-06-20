MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := caching
.DELETE_ON_ERROR:
.SUFFIXES:

VERSION = v0.1.0
SEED = 0
WARMUP = 10
NOISE_THRESHOLD = 5
REGRESSION_METRICS = iters-rate,latency-mean

COMMON_ARGS = --seed $(SEED) --collector silent --warmup $(WARMUP)
BASELINE_ARGS = --baseline-dir baselines

CACHING_ARGS = --noise-threshold $(NOISE_THRESHOLD) --totals-file baselines/totals-$(VERSION).json
CACHING_BASELINE_ARGS = $(BASELINE_ARGS) --totals-file baselines/totals-$(VERSION).json

baselines:
	mkdir -p $@

.PHONY: caching caching_baselines tokio_raw tokio_raw_save rltbl_tokio rltbl_tokio_save

tokio_raw: | baselines
	cargo run -- $(COMMON_ARGS) --duration 1m \
		--baseline-file baselines/driver-tokio-raw-$(VERSION).json \
		--fail-on-regression \
		tokio

tokio_raw_save: | baselines
	cargo run -- $(COMMON_ARGS) --duration 1m --baseline-dir baselines \
		--save-baseline driver-tokio-raw-$(VERSION) \
		tokio

rltbl_tokio: | baselines
	cargo run -- $(COMMON_ARGS) --duration 1m \
		--baseline-file baselines/driver-rltbl-tokio-$(VERSION).json \
		--fail-on-regression \
		rltbl tokio-postgresql

rltbl_tokio_save: | baselines
	cargo run -- $(COMMON_ARGS) --duration 1m --baseline-dir baselines \
		--save-baseline driver-rltbl-tokio-$(VERSION) \
		rltbl tokio-postgresql

caching: | baselines
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-none-$(VERSION).json \
		caching sqlite none
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-none-$(VERSION).json \
		caching postgresql none
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-truncate_all-$(VERSION).json \
		caching sqlite truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-truncate_all-$(VERSION).json \
		caching postgresql truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-truncate-$(VERSION).json \
		caching sqlite truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-truncate-$(VERSION).json \
		caching postgresql truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-trigger-$(VERSION).json \
		caching sqlite trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-trigger-$(VERSION).json \
		caching postgresql trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-memory-$(VERSION).json \
		caching sqlite "memory:1000"
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-memory-$(VERSION).json \
		caching postgresql "memory:1000"

caching_baselines: | baselines
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-none-$(VERSION) caching sqlite none
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-none-$(VERSION) caching postgresql none
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-truncate_all-$(VERSION) caching sqlite truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-truncate_all-$(VERSION) caching postgresql truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-truncate-$(VERSION) caching sqlite truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-truncate-$(VERSION) caching postgresql truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-trigger-$(VERSION) caching sqlite trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-trigger-$(VERSION) caching postgresql trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-memory-$(VERSION) caching sqlite "memory:1000"
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-memory-$(VERSION) caching postgresql "memory:1000"
