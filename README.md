# mortar


Running on desktop
```
cd examples/example1
cargo run
```

Running on web
```
wasm-pack build examples/example1 --target web
Change line in index.html file to point to .js file
python -m http.server
```