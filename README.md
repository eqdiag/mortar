# mortar


Running on desktop
```
cargo run
```

Running on web
```
wasm-pack build --target web
Change line in index.html file to point to .js file
python -m http.server
```


Ultimately I want a way to spit out a index.html + a pkg dir with everything ready to go for a web deploy