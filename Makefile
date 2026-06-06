MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := caching
.DELETE_ON_ERROR:
.SUFFIXES:

caching:
	cargo run --release -- sqlite none \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/sqlite-none-v0.1.0.json
	cargo run --release -- sqlite truncate_all \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/sqlite-truncate_all-v0.1.0.json
	cargo run --release -- sqlite truncate \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/sqlite-truncate-v0.1.0.json
	cargo run --release -- sqlite trigger \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/sqlite-trigger-v0.1.0.json
	cargo run --release -- sqlite "memory:1000" \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/sqlite-memory-v0.1.0.json
	cargo run --release -- postgresql none \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/postgresql-none-v0.1.0.json
	cargo run --release -- postgresql truncate_all \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/postgresql-truncate_all-v0.1.0.json
	cargo run --release -- postgresql truncate \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/postgresql-truncate-v0.1.0.json
	cargo run --release -- postgresql trigger \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/postgresql-trigger-v0.1.0.json
	cargo run --release -- postgresql "memory:1000" \
		--collector silent \
		--iterations 2500 \
		--noise-threshold 5 \
		--warmup 100 \
		--fail-on-regression \
		--regression-metrics iters-rate,latency-mean,latency-median \
		--baseline-file baselines/postgresql-memory-v0.1.0.json

save_baselines:
	cargo run --release -- sqlite none \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline sqlite-none-v0.1.0
	cargo run --release -- sqlite truncate_all \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline sqlite-truncate_all-v0.1.0
	cargo run --release -- sqlite truncate \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline sqlite-truncate-v0.1.0
	cargo run --release -- sqlite trigger \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline sqlite-trigger-v0.1.0
	cargo run --release -- sqlite "memory:1000" \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline sqlite-memory-v0.1.0
	cargo run --release -- postgresql none \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline postgresql-none-v0.1.0
	cargo run --release -- postgresql truncate_all \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline postgresql-truncate_all-v0.1.0
	cargo run --release -- postgresql truncate \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline postgresql-truncate-v0.1.0
	cargo run --release -- postgresql trigger \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline postgresql-trigger-v0.1.0
	cargo run --release -- postgresql "memory:1000" \
		--collector silent \
		--iterations 2500 \
		--warmup 100 \
		--baseline-dir baselines \
		--save-baseline postgresql-memory-v0.1.0
