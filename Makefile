MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := benchmarks
.DELETE_ON_ERROR:
.SUFFIXES:

benchmarks:
	cargo run sqlite --duration 5s
	cargo run postgres --duration 5s
