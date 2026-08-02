# Capstone 09: 2D Game Compiled to WebAssembly

**Status: complete**

A terminal-based 2D game engine demonstrating core game-dev and WASM concepts through a pure-Rust simulation with ECS-like entity management, game loop patterns, and collision detection.

## Architecture

This capstone implements a grid-based 2D game engine entirely in Rust without external game-engine dependencies. The design mirrors patterns found in real ECS (Entity Component System) engines like Bevy:

- **Entities** — each object in the game world (player, walls, enemies, collectibles, exit) is an entity with a unique ID, position, and tile type; this models the "entity" half of an ECS.
- **Components** — `Position`, `TileType`, and `alive` serve as inline component data attached to each entity.
- **Systems** — `update_enemies()` implements AI (a system that queries entities with `TileType::Enemy` and mutates their positions), `move_entity()` handles input and collision resolution, `render()` performs the draw pass.
- **Game Loop** — increment `tick`, process input, update AI, check win/loss conditions, and render each frame; delta time is implicit in the tick counter.
- **Input Handling** — player movement driven by `Direction` enum (Up/Down/Left/Right), analogous to keyboard WASD bindings.

A "real" WASM-capable game would replace this text-grid renderer with Bevy's `bevy_render` pipeline (modules 085–086) and compile the project with `wasm-pack build --target web` (module 088) to run in a plain HTML `<canvas>` page.

## Modules Covered

| Module | Topic | How Applied |
|--------|-------|-------------|
| 081 | WASM fundamentals | Simulated ECS patterns that map 1:1 to Bevy's query system |
| 082 | Rust-Web interop | Render output is a string — the natural bridge to DOM manipulation |
| 083 | Frontend integration | Game state is serialization-friendly (serde derives ready) |
| 084 | Game engines | Entity management, tile maps, collision detection |
| 085 | Bevy ECS intro | `GameState` stores entities in a HashMap (simplified `bevy::world`) |
| 086 | Systems & queries | `update_enemies()` and `move_entity()` are manual system implementations |
| 087 | Graphics pipeline | `render()` is a software rasterization pass producing ASCII art |
| 088 | wasm-pack | The crate is structured for compilation to the `wasm32-unknown-unknown` target |
| 089 | Career comparison | Demonstrates the spectrum from pure Rust to full Bevy + WASM |
| 090 | Portfolio project | A buildable, testable game engine ready for the capstone portfolio |

## Running

```bash
# Build
cargo build -p capstone-09-2d-game-solution

# Test
cargo test -p capstone-09-2d-game-solution

# For the starter (exercises):
cargo test -p capstone-09-2d-game-starter
```

## Game Map Legend

| Char | Tile Type     |
|------|---------------|
| `#`  | Wall          |
| `@`  | Player        |
| `E`  | Enemy         |
| `*`  | Collectible   |
| `X`  | Exit          |
| `.`  | Empty         |

## Path to WASM

To adapt this engine to run in a browser via WebAssembly:

1. Add `bevy` with the `webgl2` feature (or `bevy_ecs` for headless).
2. Replace `render()` with Bevy's sprite/camera setup.
3. Add a `wasm-bindgen` entry point that initializes the Bevy app.
4. Build with `wasm-pack build --target web`.
5. Serve the generated `.wasm` + JS glue from an `index.html` with a `<canvas>` element.
