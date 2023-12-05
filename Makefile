help:
	@echo "Make Commands:"
	@echo "make help\n\tThis menu"
	@echo "make init\n\tInitialize Project (For Debian/Ubuntu)"
	@echo "make dev\n\tStart with Hot Module Reload."
	@echo "make prod\n\tStart with --release"
	@echo "make stop\n\tKill running processes."

init:
# apt-get install
	@sudo apt-get update && sudo apt-get install -y gcc build-essential libssl-dev pkg-config nvidia-cuda-toolkit;
# install rust if not available
	@if ! command -v cargo; then \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; \
		. "$$HOME/.cargo/env"; \
	fi
# install wasm target if not available
	@if ! command -v rustup target list | grep "wasm32-unknown-unknown (installed)"; then \
		rustup target add wasm32-unknown-unknown; \
	fi
# install trunk if not available
	@if ! command -v trunk; then \
		cargo install trunk; \
	fi
# install cargo-watch if not available
	@if ! command -v cargo-watch; then \
		cargo install cargo-watch; \
	fi

dev:
	cd backend && cargo watch -q -c -w src/ -x run &
	cd frontend && trunk serve &

prod:
	cd backend && cargo run --release &
	cd frontend && trunk serve --release &

kill:
	@kill -9 $$(ps aux | grep -v "grep" | grep "candle-chat" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "candle-chat-backend" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "cargo-watch" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "cargo run" | xargs | cut -d ' ' -f 2) 2&1> /dev/null