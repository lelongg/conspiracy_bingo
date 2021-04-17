build:
    wasm-pack build --target web --out-name wasm --out-dir ./static

run:
    just build
    miniserve ./static --index index.html