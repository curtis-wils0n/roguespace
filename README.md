# RogueSpace

A roguelike game built with Rust, featuring classic dungeon exploration, turn-based combat, and procedural generation.

## Features

- Procedurally generated dungeons
- Turn-based combat system
- Inventory and item management
- Monster AI
- Field of view and visibility system
- Save/Load game functionality
- Multiple dungeon levels

## Prerequisites

- Rust 1.70 or later (install from [rustup.rs](https://rustup.rs/))
- A terminal or console that supports your operating system

## Installation

### Clone the Repository

```bash
git clone https://github.com/yourusername/roguespace.git
cd roguespace
```

### Build from Source

Build the project using Cargo:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/roguespace`.

## Running the Game

### Development Mode

Run directly with Cargo:

```bash
cargo run
```

### Release Mode

For better performance, run the optimized release build:

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/roguespace
```

## Controls

- Arrow keys or numpad: Move/attack
- G: Pick up item
- I: Open inventory
- D: Drop item
- Escape: Main menu

## Dependencies

This project uses the following main dependencies:

- **rltk** (0.8.7): Roguelike toolkit for rendering and terminal handling
- **specs** (0.20.0): Entity Component System framework
- **serde** (1.0): Serialization framework for save/load functionality

## Project Structure

```
roguespace/
├── src/
│   ├── main.rs              # Entry point and game loop
│   ├── components.rs        # ECS components
│   ├── map.rs              # Map generation and management
│   ├── player.rs           # Player input and actions
│   ├── spawner.rs          # Entity spawning logic
│   ├── visibility_system.rs # FOV calculations
│   ├── monster_ai_system.rs # Monster behavior
│   ├── melee_combat_system.rs # Combat logic
│   ├── damage_system.rs    # Damage application
│   ├── inventory_system.rs # Item management
│   ├── gui.rs              # User interface
│   ├── gamelog.rs          # Message logging
│   └── saveload_system.rs  # Save/load functionality
└── resources/              # Game assets (fonts, shaders)
```

## Save Files

Game saves are stored in `savegame.json` in the project root directory.

## License

Copyright (c) 2024 Curtis Wilson

## Acknowledgments

Built using the excellent [RLTK](https://github.com/amethyst/bracket-lib) (Roguelike Toolkit) library.

