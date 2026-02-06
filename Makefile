.PHONY: api
api:
	@cargo run --bin ppeg-api

.PHONY: ui
ui:
	@cd ppeg-ui && pnpm dev
