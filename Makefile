MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := caching
.DELETE_ON_ERROR:
.SUFFIXES:

ITERATIONS = 2500
NOISE_THRESHOLD = 20
WARMUP = 100
REGRESSION_METRICS = iters-rate,latency-mean,latency-median
VERSION = v0.1.0

caching:
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--fail-on-regression \
		--baseline-file baselines/sqlite-none-$(VERSION).json \
		caching sqlite none
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/sqlite-truncate_all-$(VERSION).json \
		caching sqlite truncate_all
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/sqlite-truncate-$(VERSION).json \
		caching sqlite truncate
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/sqlite-trigger-$(VERSION).json \
		caching sqlite trigger
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/sqlite-memory-$(VERSION).json \
		caching sqlite "memory:1000"
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-none-$(VERSION).json \
		caching postgresql none
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-truncate_all-$(VERSION).json \
		caching postgresql truncate_all
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-truncate-$(VERSION).json \
		caching postgresql truncate
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-trigger-$(VERSION).json \
		caching postgresql trigger
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--fail-on-regression \
		--warmup $(WARMUP) \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-memory-$(VERSION).json \
		caching postgresql "memory:1000"

save_baselines:
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-none-$(VERSION) \
		caching sqlite none
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-truncate_all-$(VERSION) \
		caching sqlite truncate_all
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-truncate-$(VERSION) \
		caching sqlite truncate
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-trigger-$(VERSION) \
		caching sqlite trigger
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-memory-$(VERSION) \
		caching sqlite "memory:1000"
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-none-$(VERSION) \
		caching postgresql none
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-truncate_all-$(VERSION) \
		caching postgresql truncate_all
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-truncate-$(VERSION) \
		caching postgresql truncate
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-trigger-$(VERSION) \
		caching postgresql trigger
	cargo run --release -- \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-memory-$(VERSION) \
		caching postgresql "memory:1000"
