# Bevy 0.18 API Reference

> **Bevy's API changes significantly between versions.** Most online tutorials, examples, and AI training data target older versions. Do not copy patterns from external sources without verifying them against this document. When in doubt, check [docs.rs/bevy](https://docs.rs/bevy/latest/bevy/) for the current API.

For best practice, the bevy examples are kept up to date and show idiomatic code usage for short snippets. These are stored locally in `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy-0.18.0/examples`. Only if a viable pattern is not established in the codebase, you can check the examples to identify a good approach.

Building the docs is time consuming and unnecessary. You can search the source registry directly, which also contains the doc strings. Note that bevy subcrates will be stored in, e.g., `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_math-0.18.0/`. In the rare cases that you cannot find what you need, and only then, you can ask the user to confirm the references or search the web yourself to confirm.

## Messages vs Events (0.17+)

Bevy splits communication into two distinct systems:

### Messages (`Message` trait) — buffered, polled each frame

Use for high-frequency game signals (damage ticks, score updates, audio triggers).

```rust
#[derive(Message, Clone)]
struct PlaySFX { sfx: GameSFX }

// Writing
fn shoot(mut sfx: MessageWriter<PlaySFX>) {
    sfx.write(PlaySFX { sfx: GameSFX::Click });
}

// Reading
fn play_audio(mut reader: MessageReader<PlaySFX>) {
    for ev in reader.read() { /* ... */ }
}

// Registration
app.add_message::<PlaySFX>();
```

### Events (`Event` trait) — triggered and observed

Use for one-off reactions (entity death, state transitions, UI interactions). No registration needed.

```rust
#[derive(Event)]
struct LevelCompleted;

// Trigger globally
commands.trigger(LevelCompleted);

// Observe
app.add_observer(|_trigger: On<LevelCompleted>, mut next: ResMut<NextState<GameState>>| {
    next.set(GameState::PrestigeMenu);
});
```

### Entity Events (`EntityEvent` trait) — triggered on a specific entity

Must include an `#[event_target]` entity field.

```rust
#[derive(EntityEvent)]
struct TileRevealed {
    #[event_target]
    tile: Entity,
    was_mine: bool,
}

commands.trigger(TileRevealed { tile: entity, was_mine: false });
```

### Removed APIs

**Do not use:** `EventWriter`, `EventReader`, `add_event`, `.send()` — these are pre-0.17 APIs that no longer exist.

---

## UI Interaction — Pointer observers

**Do not use** `Interaction` component queries, `Changed<Interaction>`, or `Button` marker for click detection. These are legacy patterns. Use **pointer event observers**:

```rust
// Observe clicks on a specific entity
commands.spawn((MyButton, Node { .. }))
    .observe(|trigger: On<Pointer<Click>>, mut commands: Commands| {
        if trigger.event().button == PointerButton::Primary {
            // left click
        } else if trigger.event().button == PointerButton::Secondary {
            // right click
        }
    });

// Observe hover for visual feedback
commands.spawn((MyButton, Node { .. }))
    .observe(|_trigger: On<Pointer<Over>>, mut bg: Query<&mut BackgroundColor>| {
        // highlight
    })
    .observe(|_trigger: On<Pointer<Out>>, mut bg: Query<&mut BackgroundColor>| {
        // un-highlight
    });
```

Available pointer events: `Click`, `Press`, `Release`, `Over`, `Out`, `Move`, `DragStart`, `Drag`, `DragEnd`, `DragEnter`, `DragOver`, `DragDrop`, `DragLeave`.

`Pointer<E>` is an `EntityEvent` — it bubbles up the hierarchy and carries `.button` (`PointerButton`), `.hit` (`HitData`), and `.pointer_location`. It derefs to the inner event type.

**Do not use:** `Interaction`, `Changed<Interaction>`, `ButtonInput<MouseButton>` for UI click detection.

---

## Observer API Details

- Use `trigger.event_target()` to get the entity an observer is attached to. **Not** `.target()` — that method does not exist.
- Use `trigger.event()` to access the event payload.

---

## Hierarchy & Child Spawning (0.16+)

`ChildBuilder` was renamed to `ChildSpawnerCommands`. Use `ChildSpawnerCommands` in function signatures that accept the closure argument from `with_children`:

```rust
fn spawn_button(parent: &mut ChildSpawnerCommands, label: &str) {
    parent.spawn((Node { .. }, Text::new(label)));
}
```

**Do not use:** `ChildBuilder` — this type no longer exists.

---

## Asset Handles

- Use `.clone()` on handles. **Not** `.clone_weak()` — that method was removed.
- Load assets via `Res<AssetServer>` in Startup systems, not `app.world().resource::<AssetServer>()` in `Plugin::build`.

---

## UI Transforms

UI entities (`Node`) do **not** use `Transform` / `GlobalTransform`. They have two dedicated transform types:

- **`UiTransform`** — Writable local transform for visual-only modifications (scale, rotation, translation). Applied **post-layout** so it does not trigger relayout. Fields: `translation: Val2`, `scale: Vec2`, `rotation: Rot2`.
- **`UiGlobalTransform`** — Read-only computed global transform (automatically maintained by the engine). Use for reading a node's final screen position.

```rust
use bevy::ui::{UiTransform, UiGlobalTransform};

// Visual-only scale (no relayout):
fn scale_node(mut q: Query<&mut UiTransform>) {
    for mut t in &mut q {
        t.scale = Vec2::splat(1.1);
    }
}

// Reading a UI node's computed position (read-only, in physical pixels):
fn read_pos(q: Query<&UiGlobalTransform>) {
    for transform in &q {
        let center = transform.affine().translation;
    }
}
```

**Do not use:** `Transform` or `GlobalTransform` on UI entities — they are not present and queries requiring them will silently match zero entities.

---

## Components

- Use `#[require(ComponentType)]` on component structs to declare required components, instead of manually inserting bundles of related components.
- `BorderColor` uses `BorderColor::all(color)`, not `BorderColor(color)`.
- `BorderRadius` is a **field on `Node`**, not a separate component. Set it as `border_radius: BorderRadius::all(Val::Px(8.0))` inside the `Node` struct. **Do not** spawn it as a separate tuple element.
- `despawn_recursive()` no longer exists. Use `despawn()` — it despawns the entity and all descendants by default.

---

## Quick Migration Checklist

| Old (pre-0.17) | Current (0.18) |
|---|---|
| `EventReader<T>` / `EventWriter<T>` | `MessageReader<T>` / `MessageWriter<T>` (for buffered) or `On<T>` observer (for triggered) |
| `add_event::<T>()` | `add_message::<T>()` (buffered) or nothing (triggered events need no registration) |
| `.send(event)` | `.write(msg)` (messages) or `commands.trigger(event)` (events) |
| `trigger_targets(event, entity)` | `EntityEvent` with `#[event_target]` field + `commands.trigger(event)` |
| `Interaction::Pressed` / `Changed<Interaction>` | `Pointer<Click>` observer |
| `trigger.target()` | `trigger.event_target()` |
| `ChildBuilder` | `ChildSpawnerCommands` |
| `handle.clone_weak()` | `handle.clone()` |
| `BorderColor(color)` | `BorderColor::all(color)` |
| `Transform` / `GlobalTransform` on UI nodes | `UiTransform` (writable, visual-only) / `UiGlobalTransform` (read-only, computed) |
