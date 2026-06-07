MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := caching
.DELETE_ON_ERROR:
.SUFFIXES:

ITERATIONS = 2500
NOISE_THRESHOLD = 5
WARMUP = 100
REGRESSION_METRICS = latency-mean,latency-median
VERSION = v0.1.0

caching:
	cargo run --release -- sqlite none \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/sqlite-none-$(VERSION).json
	cargo run --release -- sqlite truncate_all \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/sqlite-truncate_all-$(VERSION).json
	cargo run --release -- sqlite truncate \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/sqlite-truncate-$(VERSION).json
	cargo run --release -- sqlite trigger \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/sqlite-trigger-$(VERSION).json
	cargo run --release -- sqlite "memory:1000" \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/sqlite-memory-$(VERSION).json
	cargo run --release -- postgresql none \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-none-$(VERSION).json
	cargo run --release -- postgresql truncate_all \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-truncate_all-$(VERSION).json
	cargo run --release -- postgresql truncate \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-truncate-$(VERSION).json
	cargo run --release -- postgresql trigger \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-trigger-$(VERSION).json
	cargo run --release -- postgresql "memory:1000" \
		--collector silent \
		--iterations $(ITERATIONS) \
		--noise-threshold $(NOISE_THRESHOLD) \
		--warmup $(WARMUP) \
		--fail-on-regression \
		--regression-metrics $(REGRESSION_METRICS) \
		--baseline-file baselines/postgresql-memory-$(VERSION).json

save_baselines:
	cargo run --release -- sqlite none \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-none-$(VERSION)
	cargo run --release -- sqlite truncate_all \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-truncate_all-$(VERSION)
	cargo run --release -- sqlite truncate \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-truncate-$(VERSION)
	cargo run --release -- sqlite trigger \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-trigger-$(VERSION)
	cargo run --release -- sqlite "memory:1000" \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline sqlite-memory-$(VERSION)
	cargo run --release -- postgresql none \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-none-$(VERSION)
	cargo run --release -- postgresql truncate_all \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-truncate_all-$(VERSION)
	cargo run --release -- postgresql truncate \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-truncate-$(VERSION)
	cargo run --release -- postgresql trigger \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-trigger-$(VERSION)
	cargo run --release -- postgresql "memory:1000" \
		--collector silent \
		--iterations $(ITERATIONS) \
		--warmup $(WARMUP) \
		--baseline-dir baselines \
		--save-baseline postgresql-memory-$(VERSION)
