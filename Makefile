MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := caching
.DELETE_ON_ERROR:
.SUFFIXES:

caching:
	cargo run --release -- sqlite \
		--collector silent \
		--iterations 2500 \
		--baseline-file baselines/sqlite-v0.1.0.json
	cargo run --release -- postgresql \
		--collector silent \
		--iterations 2500 \
		--baseline-file baselines/postgresql-v0.1.0.json

save_baselines:
	cargo run --release -- sqlite \
		--collector silent \
		--iterations 2500 \
		--baseline-dir baselines \
		--save-baseline sqlite-v0.1.0
	cargo run --release -- postgresql \
		--collector silent \
		--iterations 2500 \
		--baseline-dir baselines \
		--save-baseline postgresql-v0.1.0
