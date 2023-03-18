build:
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/slamburger.wasm ./
server:
	@python3 -m http.server 8000