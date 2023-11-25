help:
	@echo "Make Commands:"
	@echo "make help\n\tThis menu"
	@echo "make init\n\tInitialize Project (For Debian/Ubuntu)"
	@echo "make dev\n\tStart with Hot Module Reload."
	@echo "make prod\n\tStart with --release"
	@echo "make stop\n\tKill running processes."

init:
	@sudo apt-get update && sudo apt-get install -y gcc build-essential libssl-dev pkg-config nvidia-cuda-toolkit;
	@if ! command -v cargo &> /dev/null; then \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; \
		. "$$HOME/.cargo/env"; \
		rustup toolchain install nightly; \
		rustup override set nightly; \
		rustup target add wasm32-unknown-unknown; \
	fi
	@if ! command -v trunk &> /dev/null; then \
		cargo install trunk; \
	fi
	@if ! command -v cargo-watch &> /dev/null; then \
		cargo install cargo-watch; \
	fi

dev:
	cd backend && cargo watch -q -c -w src/ -x run &
	cd frontend && trunk serve

prod:
	cd backend && cargo run --release &
	cd frontend && trunk serve --release 

stop:
	@kill -9 $$(ps aux | grep target/debug/axum_mistral | xargs | cut -d ' ' -f 2) 2&> /dev/null
	@kill -9 $$(ps aux | grep leptos_frontend | xargs | cut -d ' ' -f 2) 2&> /dev/null