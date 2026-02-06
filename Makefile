.PHONY: api
api:
	@cargo run --bin ppeg-api

.PHONY: ui
ui:
	@cd ppeg-ui && pnpm dev

.PHONY: bench
bench:
	@cd bench && cargo bench

.PHONY: e2e
e2e:
	@cd e2e && cargo test
