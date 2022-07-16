## About

`jp-transcript` is a library for generating transcriptions from japanese texts used in the NYARS Online Dictionary. It has no built-in taggers, so it only works with kana.

Written with Rust, compiles to WASM with `wasm-pack` and works as an NPM module. Make sure your bundler supports WASM.

## Development

`wasm-pack build --release --target=bundler`
