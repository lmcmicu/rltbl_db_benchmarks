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

CACHING_ARGS = --noise-threshold $(NOISE_THRESHOLD) 
CACHING_BASELINE_ARGS = $(BASELINE_ARGS)

DRIVER_ARGS = --duration 1m

baselines:
	mkdir -p $@

output:
	mkdir -p $@

.PHONY: caching caching_baselines tokio_raw tokio_raw_save rltbl_tokio rltbl_tokio_save save_baselines

save_baselines: tokio_raw_save rltbl_tokio_save caching_baselines

tokio_raw: | baselines output
	cargo run -- $(COMMON_ARGS) $(DRIVER_ARGS) \
		--output json --output-file output/driver-tokio-postgres-raw-$(VERSION).json \
		--baseline-file baselines/driver-tokio-postgres-raw-$(VERSION).json \
		--fail-on-regression \
		tokio-postgres-driver

tokio_raw_save: | baselines
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) $(DRIVER_ARGS) \
		--save-baseline driver-tokio-postgres-raw-$(VERSION) \
		tokio-postgres-driver

rltbl_tokio: | baselines output
	cargo run -- $(COMMON_ARGS) $(DRIVER_ARGS) \
		--output json --output-file output/driver-rltbl-tokio-postgres-$(VERSION).json \
		--baseline-file baselines/driver-rltbl-tokio-postgres-$(VERSION).json \
		--fail-on-regression \
		rltbl-driver tokio-postgres

rltbl_tokio_save: | baselines
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) $(DRIVER_ARGS) \
		--save-baseline driver-rltbl-tokio-postgres-$(VERSION) \
		rltbl-driver tokio-postgres

caching: | baselines output
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-none-$(VERSION).json \
		--output json --output-file output/caching-sqlite-none-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite none
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgres-none-$(VERSION).json \
		--output json --output-file output/caching-postgres-none-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres none
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-truncate_all-$(VERSION).json \
		--output json --output-file output/caching-sqlite-truncate_all-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgres-truncate_all-$(VERSION).json \
		--output json --output-file output/caching-postgres-truncate_all-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-truncate-$(VERSION).json \
		--output json --output-file output/caching-sqlite-truncate-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgres-truncate-$(VERSION).json \
		--output json --output-file output/caching-postgres-truncate-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-trigger-$(VERSION).json \
		--output json --output-file output/caching-sqlite-trigger-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgres-trigger-$(VERSION).json \
		--output json --output-file output/caching-postgres-trigger-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-memory-$(VERSION).json \
		--output json --output-file output/caching-sqlite-memory-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite "memory:1000"
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgres-memory-$(VERSION).json \
		--output json --output-file output/caching-postgres-memory-$(VERSION).json \
		--fail-on-regression \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres "memory:1000"

caching_baselines: | baselines
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-none-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite none
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgres-none-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres none
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-truncate_all-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgres-truncate_all-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-truncate-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgres-truncate-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-trigger-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgres-trigger-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-memory-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite "memory:1000"
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgres-memory-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgres "memory:1000"
