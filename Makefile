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

#
# BuildS
# run first if building without cuda (is auto run in some cases)
cpu-only:
	@sed -i 's|, features = \["cuda"\]||g' backend/Cargo.toml

# ARMv7 (32-bit)
linux-v7: cpu-only
	@rustup target add armv7-unknown-linux-gnueabihf
	@cargo tauri build --target armv7-unknown-linux-gnueabihf

# ARMv7 (32-bit) Cross Compile !!Broken!!
linux-cross-v7: cpu-only
	@rustup target add armv7-unknown-linux-gnueabihf
	@sudo apt install gcc-arm-linux-gnueabihf
	@echo '\n[target.armv7-unknown-linux-gnueabihf] \nlinker = "arm-linux-gnueabihf-gcc"' | tee -a .cargo/config.toml
	@sudo dpkg --add-architecture armhf
	@echo '\n\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy main restricted\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates main restricted\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy universe\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates universe\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy multiverse\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates multiverse\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-backports main restricted universe multiverse\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security main restricted\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security universe\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security multiverse' | sudo tee -a /etc/apt/sources.list
	@sudo sed -i 's|deb http|deb [arch=amd64] http|g' /etc/apt/sources.list
	@sudo apt-get update && sudo apt-get upgrade -y
	@sudo apt install libwebkit2gtk-4.0-dev:armhf
	@sudo apt install libssl-dev:armhf
	@export PKG_CONFIG_SYSROOT_DIR=/usr/arm-linux-gnueabihf/
	@cargo tauri build --target armv7-unknown-linux-gnueabihf

linux-setup:
	@[ -n "$$(which cargo)" ] && rustup update || curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
	@echo '\tGet Tauri installed for your system:\n https://tauri.app/v1/guides/getting-started/prerequisites/'

# ARMv8 (ARM64, 64-bit)
linux-cross-v8: cpu-only
	@rustup target add aarch64-unknown-linux-gnu
	@cargo tauri build --target aarch64-unknown-linux-gnu

# ARMv8 (ARM64, 64-bit) Cross Compile !!Broken!!
linux-cross-v8: cpu-only
	@rustup target add aarch64-unknown-linux-gnu
	@sudo apt install gcc-aarch64-linux-gnu
	@echo '[target.aarch64-unknown-linux-gnu]\nlinker = "aarch64-linux-gnu-gcc"' | tee -a .cargo/config.toml
	@sudo dpkg --add-architecture arm64
	@echo '\n\ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy main restricted \ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates main restricted \ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy universe \ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates universe \ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy multiverse \ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates multiverse \ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-backports main restricted universe multiverse \ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security main restricted \ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security universe \ndeb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security multiverse \n' | sudo tee -a /etc/apt/sources.list
	@sudo sed -i 's|deb http|deb [arch=amd64] http|g' /etc/apt/sources.list
	@sudo apt-get update && sudo apt-get upgrade -y
	@sudo apt install libwebkit2gtk-4.0-dev:arm64
	@sudo apt install libssl-dev:arm64
	@export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/
	@cargo tauri build --target aarch64-unknown-linux-gnu

# windows i686
windows-i686:
	@rustup target add i686-pc-windows-msvc
	@tauri build --target i686-pc-windows-msvc

# windows aarch64
windows-aarch64:
	@rustup target add aarch64-pc-windows-msvc
	@tauri build --target aarch64-pc-windows-msvc

# Install Rust (Mac)
mac-setup: cpu-only
	@xcode-select --install
	@[ -n "$$(which cargo)" ] && rustup update || curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# mac aarch64
mac-aarch64: mac-setup
	@rustup target add x86_64-apple-darwin
	@tauri build --target aarch64-apple-darwin

# mac x86
mac-x86: mac-setup
	@rustup target add aarch64-apple-darwin
	@tauri build --target x86_64-apple-darwin

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
	@cd frontend && leptosfmt ./**/**/**/**/**/**/**/*.rs

kill:
	@kill -9 $$(ps aux | grep -v "grep" | grep "frontend" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "backend" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "database" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "cargo-watch" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "cargo run" | xargs | cut -d ' ' -f 2) 2&1> /dev/null
	@kill -9 $$(ps aux | grep -v "grep" | grep "trunk serve" | xargs | cut -d ' ' -f 2) 2&1> /dev/null