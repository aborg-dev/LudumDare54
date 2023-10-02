# LudumDare54
Game for Ludum Dare 54

To build for web:

```
cargo build --release --target wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/ludum_dare_54.wasm
```

Then push to `web` branch
