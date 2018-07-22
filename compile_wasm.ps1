cargo build --target wasm32-unknown-unknown --lib
wasm-bindgen target/wasm32-unknown-unknown/debug/from_rust.wasm --out-dir ./frontend/src
