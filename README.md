# To run

```bash
cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/rust_wasm_bug.wasm --out-dir . --target web
python3 -m http.server
```