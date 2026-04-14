## help: get info about the targets within this makefile
.PHONY: help
help:
	@echo "aircraft-data-tools-api usage:"
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' | sed -e 's/^/ /'

## all: run tests and clippy
.PHONY: all
all:
	@cargo test
	@cargo clippy

## ui: run the user interface
.PHONY: ui
ui:
	@cd ppeg-ui && pnpm dev
