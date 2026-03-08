# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo run          # Run the game (uses dynamic linking for fast iteration)
cargo build        # Build without running
cargo check        # Type-check only (fastest feedback loop)
cargo test         # Run all tests (workspace-wide)
cargo test -p save_game  # Run tests for a specific crate

cargo run --features dev_console   # Run with developer console (F7 to toggle)
```

Bevy's `dynamic_linking` feature is enabled for dev builds — first compile is slow but incremental rebuilds are fast.

## Code Style

**Always import types at the top of the file** — never use inline embedded paths like `crate::game::Foo` in function bodies or struct fields. Import the type with `use` and refer to it by its short name.

**Always use the existing colour palette first instead of hard-coded colours** — colours should be pulled from `src\assets\palette.rs` except in rare instances. If there is no suitable colour there, then get confirmation from the user that an inline colour is suitable.

## Code Quality & Linting

**Always run `cargo fmt` after completing any code changes.** This ensures consistent formatting across the codebase.

**Run `cargo clippy` and carefully consider its suggestions.** However, not all clippy warnings should be fixed:

- **Complex query types**: Clippy often flags query types as overly complex. Adding type aliases typically makes the code more convoluted — ignore these warnings.
- **Too many arguments**: Systems may exceed clippy's argument count limit. This can be acceptable, but may also indicate the system has too much responsibility. Consider refactoring by:
  - Dispatching to one-shot systems for isolated tasks
  - Triggering events for observer subsystems to handle specific concerns

## Project Overview

KPop horses is an educational project to teach the designers daughter about game development with AI agents. The project is built with **Bevy 0.18** (Rust edition 2024). The game design is documented in `specs/game_design.md`.

## Bevy 0.18 API

**Bevy's API changes rapidly between versions.** Most online tutorials, examples, and AI training data target older versions — do not copy patterns from external sources without verifying against the project's reference document.

See **[`docs/bevy_reference.md`](docs/bevy_reference.md)** for the full API reference, migration checklist, and list of removed/renamed APIs. Key points:

- **Messages** (`MessageWriter`/`MessageReader`) for buffered signals; **Events** (`commands.trigger()` + observers) for one-off reactions. Do not use `EventWriter`/`EventReader`/`add_event`/`.send()`.
- **Pointer observers** (`Pointer<Click>`, `Pointer<Over>`, etc.) for all UI interaction. Do not use `Interaction` polling.
- `trigger.event_target()` — not `.target()`.
- `ChildSpawnerCommands` — not `ChildBuilder`.
- `handle.clone()` — not `.clone_weak()`.
- **UI nodes use `UiTransform`** (writable, post-layout visual transform) and **`UiGlobalTransform`** (read-only, computed). Use `UiTransform` for visual scale/rotation/translation without triggering relayout. Do not use `Transform`/`GlobalTransform` on UI entities.

## ECS Design Principles

### One concern per component

Components hold data for a single concern. Prefer many small components over few large ones — this keeps queries focused and systems decoupled.

```rust
// Good: separate concerns
#[derive(Component)] struct Health(f32);
#[derive(Component)] struct MineCount(u32);
#[derive(Component)] struct GridPosition { x: i32, y: i32 }

// Bad: god-component
#[derive(Component)] struct Tile { health: f32, mine_count: u32, x: i32, y: i32, revealed: bool, flagged: bool }
```

### Marker components over booleans

Use zero-sized marker components for entity states. This makes queries filter naturally without runtime checks.

```rust
#[derive(Component)] struct Revealed;
#[derive(Component)] struct Flagged;
#[derive(Component)] struct Mine;

// Query only unrevealed tiles — no if-checks needed
fn reveal_system(query: Query<Entity, (With<Tile>, Without<Revealed>)>) { /* ... */ }
```

### One plugin per feature domain

Each game feature gets its own plugin in its own module. Plugins register their own systems, resources, messages, and observers. `main.rs` only chains `add_plugins` calls.

```rust
// game/board.rs
pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TileClicked>()
            .add_systems(Update, handle_tile_click);
    }
}
```

### Systems do one thing

Keep systems small and focused on a single transformation. Chain ordering with `.before()`/`.after()` when systems depend on each other. Prefer many small systems over one large orchestrator.

### Resources for global state, components for per-entity state

Use `Resource` for singleton game-wide data (score, settings, RNG). Use `Component` for anything that varies per entity. Never store entity-specific data in a resource.

## Architecture

### Workspace Structure

- **`src/`** — Main game binary and library (`loot_sweeper`)
  - `assets/` — Asset loading plugins (sprites, audio, palette)
  - `game/` — Core game logic, state machine, animation system
