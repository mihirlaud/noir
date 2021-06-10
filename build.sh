cargo build --target wasm32-unknown-unknown --release

rm -rf docs
mkdir docs

cp target/wasm32-unknown-unknown/release/noir.wasm docs/
cp target/wasm32-unknown-unknown/release/noir.d docs/
cp utils/index.html docs/
cp utils/gl.js docs/
cp utils/square.ttf docs/
