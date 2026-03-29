# mortar


TODOS
- [ ] create builders for common wgpu objects 
- [ ] create builders for common graphics/compute structures (gcore)
- [ ] basic drawing library for creative coding
- [ ] simple ui library, can be plugged into games
- [ ] OOP game engine framework
- [ ] ECS game engine framework

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
