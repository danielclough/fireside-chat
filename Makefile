help:
	@echo "Make Commands:"
	@echo "make help\n\tThis menu"
	@echo "make init\n\tInitialize Project (For Debian/Ubuntu)"
	@echo "make dev\n\tStart with Hot Module Reload."
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

.PHONY: docker
docker:
	@cd docker && sh docker-build.sh
	@cd docker && docker compose pull
	@cd docker && docker compose up -d
	@cd docker && docker system prune -a

# requires kill in order to shut everything down
dev:
	@cd backend && cargo watch -q -c -w src/ -x run &
	@cd database && cargo watch -q -c -w src/ -x run &
	@cd frontend && trunk serve &

.PHONY: frontend backend
# requires kill in order to shut everything down
frontend:
	@cd frontend && trunk serve &

# requires kill in order to shut everything down
backend:
	@cd backend && ls && cargo watch -q -c -w src/ -x run &
	@cd database && ls && cargo watch -q -c -w src/ -x run &

test:
	@CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
	@grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/html
	@find ./ -type f -name "*.profraw" -delete

fmt:
	@cargo fmt
	@cd frontend && leptosfmt ./**/**/**/**/**/**/**/*.rs

build-all:
	@sed -i 's|"targets": \["deb", "app", "dmg"\],|"targets": "all",|' tauri/tauri.conf.json
	@export CUDA_COMPUTE_CAP=75 && cargo tauri build
	@sed -i 's|"targets": "all",|"targets": \["deb", "app", "dmg"\],|' tauri/tauri.conf.json

kill:
	@kill -9 $$(ps aux | grep -v "grep" | grep "frontend" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "backend" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "database" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "cargo-watch" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "cargo run" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "trunk serve" | xargs | cut -d ' ' -f 2) 2&1> /dev/null