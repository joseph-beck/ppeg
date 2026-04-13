.PHONY: all
all:
	@cargo test
	@cargo clippy

.PHONY: ui
ui:
	@cd ppeg-ui && pnpm dev
