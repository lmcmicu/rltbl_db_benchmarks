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

.PHONY: caching caching_baselines tokio_raw tokio_raw_save rltbl_tokio rltbl_tokio_save

tokio_raw: | baselines
	cargo run -- $(COMMON_ARGS) $(DRIVER_ARGS) \
		--output-file output/driver-tokio-postgres-raw-$(VERSION).txt \
		--baseline-file baselines/driver-tokio-postgres-raw-$(VERSION).json \
		tokio-postgres-driver

tokio_raw_save: | baselines
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) $(DRIVER_ARGS) \
		--save-baseline driver-tokio-postgres-raw-$(VERSION) \
		tokio-postgres-driver

rltbl_tokio: | baselines
	cargo run -- $(COMMON_ARGS) $(DRIVER_ARGS) \
		--output-file output/driver-rltbl-tokio-postgres-$(VERSION).txt \
		--baseline-file baselines/driver-rltbl-tokio-postgres-$(VERSION).json \
		rltbl-driver tokio-postgres

rltbl_tokio_save: | baselines
	cargo run -- $(COMMON_ARGS) $(BASELINE_ARGS) $(DRIVER_ARGS) \
		--save-baseline driver-rltbl-tokio-postgres-$(VERSION) \
		rltbl-driver tokio-postgres

output:
	mkdir -p $@

caching: | baselines output
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-none-$(VERSION).json \
		--output-file output/caching-sqlite-none-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite none
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-none-$(VERSION).json \
		--output-file output/caching-postgresql-none-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql none
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-truncate_all-$(VERSION).json \
		--output-file output/caching-sqlite-truncate_all-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-truncate_all-$(VERSION).json \
		--output-file output/caching-postgresql-truncate_all-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-truncate-$(VERSION).json \
		--output-file output/caching-sqlite-truncate-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-truncate-$(VERSION).json \
		--output-file output/caching-postgresql-truncate-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-trigger-$(VERSION).json \
		--output-file output/caching-sqlite-trigger-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-trigger-$(VERSION).json \
		--output-file output/caching-postgresql-trigger-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-sqlite-memory-$(VERSION).json \
		--output-file output/caching-sqlite-memory-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite "memory:1000"
	cargo run -- $(COMMON_ARGS) $(CACHING_ARGS) \
		--baseline-file baselines/caching-postgresql-memory-$(VERSION).json \
		--output-file output/caching-postgresql-memory-$(VERSION).txt \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql "memory:1000"

caching_baselines: | baselines
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-none-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite none
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-none-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql none
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-truncate_all-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-truncate_all-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql truncate_all
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-truncate-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-truncate-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql truncate
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-trigger-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-trigger-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql trigger
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-sqlite-memory-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json sqlite "memory:1000"
	cargo run -- $(COMMON_ARGS) $(CACHING_BASELINE_ARGS) \
		--save-baseline caching-postgresql-memory-$(VERSION) \
		caching --totals-file baselines/caching-totals-$(VERSION).json postgresql "memory:1000"
