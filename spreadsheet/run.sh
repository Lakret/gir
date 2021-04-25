wasm-pack build --target web --out-name wasm --out-dir ./static

# to automatically serve, a binary CLI dep is required:
#
# $ cargo install miniserve

miniserve ./static --index index.html
