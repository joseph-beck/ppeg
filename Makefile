.phony: api
api:
	@cargo run --bin ppeg-api

.phony: ui
ui:
	@cd ppeg-ui && pnpm dev