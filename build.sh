cargo build --target wasm32-unknown-unknown --release

rm -rf docs
mkdir docs

wasm-bindgen target/wasm32-unknown-unknown/release/noir.wasm --out-dir docs --no-modules --no-typescript
cp utils/index.html docs
cp utils/styles.css docs