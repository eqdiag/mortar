# mortar

## Game 1
- at the level of app/game, apps have screens
- screens have same interface as below...game has one active screen at a time
- uiscreen type can be specialized for drawing ui elements
- screens can delegate to other screens for overlays, transparency,etc
- keep board square for game 1
- create simple rendercontext type for drawing shapes (wraps wgpu)
- math library
- use palette for colors
- make game feel interactive (color changes,etc)
- l2 after game 1
- [ ] onTick(double nanoseconds) for update
- [ ] onDraw() method
- [ ] onResize(size)
- [ ] onKeyPressed()
- [ ] onMouseDragged()

TODOS
- [ ] create builders for common wgpu objects 
- [ ] create builders for common graphics/compute structures (gcore)
- [ ] basic drawing library for creative coding
- [ ] simple ui library, can be plugged into games
- [ ] OOP game engine framework
- [ ] ECS game engine framework

Running on desktop
```
cargo runs
```

## Notes to self
- math (dir)
    - vec.rs
    - matrix.rs
- graphics (dir)
    - types.rs: helpers for building up wgpu types
    - render_context.rs: type that makes building graphics applications much easier
    - ui.rs: basic ui components built on render_context
- core (dir)
    - app flow stuff, entry point
- engine (dir)
    - oop engine stuff
    - ecs engine stuff