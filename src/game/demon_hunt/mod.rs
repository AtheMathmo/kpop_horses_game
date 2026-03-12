//! Demon Hunt mini-game — arcade pop-up shooter!
//!
//! Demons spawn and grow toward the player. Click to zap them with purple
//! lightning. Wave-based progression with 3 lives (hearts).

use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;

use crate::assets::Palette;
use crate::game::GameState;
use crate::game::effects::spawn_celebration_sparkles;
use face_gen::DemonType;

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct DemonHuntPlugin;

impl Plugin for DemonHuntPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DemonHuntState>()
            .add_systems(
                OnEnter(GameState::DemonHunt),
                (spawn_demon_hunt_ui, setup_level),
            )
            .add_systems(
                Update,
                (
                    spawn_demons,
                    grow_demons,
                    update_lightning_bolts,
                    check_level_complete,
                    update_level_transition,
                    update_score_ui,
                    update_hearts_ui,
                )
                    .run_if(in_state(GameState::DemonHunt)),
            );
    }
}

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[derive(Component)]
struct Demon {
    size: f32,
    approach_speed: f32,
}

#[derive(Component)]
struct LightningBolt {
    lifetime: Timer,
}

#[derive(Component)]
struct ScoreLabel;

#[derive(Component)]
struct LevelLabel;

#[derive(Component)]
struct Heart(u32);

#[derive(Component)]
struct OverlayText;

// ---------------------------------------------------------------------------
// Resource
// ---------------------------------------------------------------------------

#[derive(Resource)]
struct DemonHuntState {
    score: u32,
    level: u32,
    lives: u32,
    demons_remaining: u32,
    demons_spawned: u32,
    demons_total: u32,
    spawn_timer: Timer,
    game_over: bool,
    level_complete: bool,
    level_transition_timer: Option<Timer>,
    /// Simple pseudo-RNG state.
    rng_state: u64,
}

impl Default for DemonHuntState {
    fn default() -> Self {
        Self {
            score: 0,
            level: 1,
            lives: MAX_LIVES,
            demons_remaining: 0,
            demons_spawned: 0,
            demons_total: 0,
            spawn_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
            game_over: false,
            level_complete: false,
            level_transition_timer: None,
            rng_state: 12345,
        }
    }
}

impl DemonHuntState {
    /// Simple xorshift64 PRNG.
    fn next_random(&mut self) -> f32 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        (self.rng_state % 10000) as f32 / 10000.0
    }
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const MAX_LIVES: u32 = 3;
const MAX_LEVEL: u32 = 10;
const DEMON_START_SCALE: f32 = 0.15;
const DEMON_MAX_SCALE: f32 = 1.0;
const LEVEL_TRANSITION_SECS: f32 = 2.0;
/// World-space bounds for demon spawning.
const SPAWN_X_RANGE: (f32, f32) = (-350.0, 350.0);
const SPAWN_Y_RANGE: (f32, f32) = (-150.0, 150.0);

// ---------------------------------------------------------------------------
// Level configuration
// ---------------------------------------------------------------------------

struct LevelConfig {
    demon_count: u32,
    spawn_interval: f32,
    approach_speed: f32,
}

fn level_config(level: u32) -> LevelConfig {
    let l = (level.min(MAX_LEVEL) - 1) as f32;
    LevelConfig {
        demon_count: 5 + (l * 2.2) as u32,
        spawn_interval: (2.5 - l * 0.19).max(0.8),
        approach_speed: 0.08 + l * 0.02,
    }
}

fn demon_types_for_level(level: u32) -> &'static [DemonType] {
    match level {
        1 => &[DemonType::Blob],
        2 => &[DemonType::Blob, DemonType::Puff],
        3 => &[DemonType::Blob, DemonType::Puff, DemonType::Bat],
        _ => face_gen::ALL_DEMON_TYPES,
    }
}

// ---------------------------------------------------------------------------
// Spawn UI
// ---------------------------------------------------------------------------

fn spawn_demon_hunt_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            DespawnOnExit(GameState::DemonHunt),
            // Let clicks pass through to world-space demon sprites.
            Pickable::IGNORE,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
        ))
        .with_children(|root| {
            // Top bar
            root.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|top| {
                // Level — large and centred
                top.spawn((
                    LevelLabel,
                    Text::new("Level 1"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Palette::HotMagenta.into()),
                ));

                // Score + hearts row
                top.spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(24.0),
                    ..default()
                })
                .with_children(|row| {
                    // Score
                    row.spawn((
                        ScoreLabel,
                        Text::new("Demons: 0"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Palette::WhiteSmoke.into()),
                    ));

                    // Hearts
                    row.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(6.0),
                        ..default()
                    })
                    .with_children(|hearts| {
                        for i in 0..MAX_LIVES {
                            hearts.spawn((
                                Heart(i),
                                Node {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                    ..default()
                                },
                                ImageNode::new(asset_server.load("demons/heart_full.png")),
                            ));
                        }
                    });
                });
            });

            // Spacer
            root.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });

            // Back button
            root.spawn((
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(Palette::ElectricPurple.into()),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("Back to Games"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Palette::WhiteSmoke.into()),
                ));
            })
            .observe(on_back_click)
            .observe(on_button_over)
            .observe(on_button_out);
        });
}

// ---------------------------------------------------------------------------
// Level setup
// ---------------------------------------------------------------------------

fn setup_level(mut state: ResMut<DemonHuntState>) {
    let config = level_config(1);
    *state = DemonHuntState {
        score: 0,
        level: 1,
        lives: MAX_LIVES,
        demons_remaining: config.demon_count,
        demons_spawned: 0,
        demons_total: config.demon_count,
        spawn_timer: Timer::from_seconds(config.spawn_interval, TimerMode::Repeating),
        game_over: false,
        level_complete: false,
        level_transition_timer: None,
        rng_state: 98765,
    };
}

fn advance_to_level(state: &mut DemonHuntState) {
    let config = level_config(state.level);
    state.demons_remaining = config.demon_count;
    state.demons_spawned = 0;
    state.demons_total = config.demon_count;
    state.spawn_timer = Timer::from_seconds(config.spawn_interval, TimerMode::Repeating);
    state.level_complete = false;
    state.level_transition_timer = None;
}

// ---------------------------------------------------------------------------
// Spawn demons
// ---------------------------------------------------------------------------

fn spawn_demons(
    time: Res<Time>,
    mut state: ResMut<DemonHuntState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if state.game_over || state.level_complete {
        return;
    }
    if state.demons_spawned >= state.demons_total {
        return;
    }

    state.spawn_timer.tick(time.delta());
    if !state.spawn_timer.just_finished() {
        return;
    }

    let config = level_config(state.level);
    let types = demon_types_for_level(state.level);
    let type_idx = (state.next_random() * types.len() as f32) as usize % types.len();
    let demon_type = types[type_idx];

    let x = SPAWN_X_RANGE.0 + state.next_random() * (SPAWN_X_RANGE.1 - SPAWN_X_RANGE.0);
    let y = SPAWN_Y_RANGE.0 + state.next_random() * (SPAWN_Y_RANGE.1 - SPAWN_Y_RANGE.0);

    let path = format!("demons/{}.png", demon_type.label());

    commands
        .spawn((
            Demon {
                size: DEMON_START_SCALE,
                approach_speed: config.approach_speed,
            },
            DespawnOnExit(GameState::DemonHunt),
            // Pickable is required for the sprite picking backend to detect clicks.
            Pickable::default(),
            Sprite {
                image: asset_server.load(path),
                custom_size: Some(Vec2::splat(200.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 5.0).with_scale(Vec3::splat(DEMON_START_SCALE)),
        ))
        .observe(on_demon_click);

    state.demons_spawned += 1;
}

// ---------------------------------------------------------------------------
// Grow demons
// ---------------------------------------------------------------------------

fn grow_demons(
    time: Res<Time>,
    mut commands: Commands,
    mut state: ResMut<DemonHuntState>,
    mut query: Query<(Entity, &mut Demon, &mut Transform)>,
) {
    if state.game_over {
        return;
    }

    let dt = time.delta_secs();
    let mut lost_life = false;

    for (entity, mut demon, mut transform) in &mut query {
        demon.size += demon.approach_speed * dt;
        let scale = demon.size.min(DEMON_MAX_SCALE);
        transform.scale = Vec3::splat(scale);

        if demon.size >= DEMON_MAX_SCALE {
            let pos = transform.translation.truncate();
            commands.entity(entity).despawn();
            spawn_celebration_sparkles(&mut commands, pos, 6);
            state.demons_remaining = state.demons_remaining.saturating_sub(1);
            lost_life = true;
        }
    }

    if lost_life {
        state.lives = state.lives.saturating_sub(1);
        if state.lives == 0 {
            state.game_over = true;
            spawn_overlay_text(&mut commands, "Game Over!", Palette::NeonPink);
        }
    }
}

// ---------------------------------------------------------------------------
// Click interaction
// ---------------------------------------------------------------------------

fn on_demon_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    demon_query: Query<(&Demon, &GlobalTransform)>,
    mut state: ResMut<DemonHuntState>,
) {
    if state.game_over || state.level_complete {
        return;
    }

    let entity = trigger.event_target();
    let Ok((_demon, global_tf)) = demon_query.get(entity) else {
        return;
    };

    let demon_pos = global_tf.translation().truncate();

    // Spawn lightning bolt
    let click_pos = demon_pos + Vec2::new(0.0, 200.0); // lightning comes from above
    spawn_lightning(&mut commands, click_pos, demon_pos);

    // Sparkles at demon position
    spawn_celebration_sparkles(&mut commands, demon_pos, 8);

    // Despawn demon
    commands.entity(entity).despawn();

    // Update state
    state.score += 1;
    state.demons_remaining = state.demons_remaining.saturating_sub(1);
}

// ---------------------------------------------------------------------------
// Lightning effect
// ---------------------------------------------------------------------------

fn spawn_lightning(commands: &mut Commands, from: Vec2, to: Vec2) {
    let diff = to - from;
    let dist = diff.length();
    let midpoint = (from + to) / 2.0;
    let angle = diff.y.atan2(diff.x);

    // Main bolt
    commands.spawn((
        LightningBolt {
            lifetime: Timer::from_seconds(0.3, TimerMode::Once),
        },
        Sprite {
            color: Palette::ElectricPurple.into(),
            custom_size: Some(Vec2::new(dist, 4.0)),
            ..default()
        },
        Transform::from_translation(midpoint.extend(15.0))
            .with_rotation(Quat::from_rotation_z(angle)),
    ));

    // Fork branch
    let branch_end = to + Vec2::new(20.0, -15.0);
    let branch_diff = branch_end - to;
    let branch_dist = branch_diff.length();
    let branch_mid = (to + branch_end) / 2.0;
    let branch_angle = branch_diff.y.atan2(branch_diff.x);

    commands.spawn((
        LightningBolt {
            lifetime: Timer::from_seconds(0.25, TimerMode::Once),
        },
        Sprite {
            color: Palette::HotMagenta.into(),
            custom_size: Some(Vec2::new(branch_dist, 2.5)),
            ..default()
        },
        Transform::from_translation(branch_mid.extend(15.0))
            .with_rotation(Quat::from_rotation_z(branch_angle)),
    ));
}

fn update_lightning_bolts(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut LightningBolt, &mut Sprite)>,
) {
    for (entity, mut bolt, mut sprite) in &mut query {
        bolt.lifetime.tick(time.delta());
        let alpha = 1.0 - bolt.lifetime.fraction();
        sprite.color = sprite.color.with_alpha(alpha);
        if bolt.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ---------------------------------------------------------------------------
// Level completion
// ---------------------------------------------------------------------------

fn check_level_complete(
    mut commands: Commands,
    mut state: ResMut<DemonHuntState>,
    demons: Query<&Demon>,
) {
    if state.game_over || state.level_complete {
        return;
    }
    if state.demons_remaining > 0 || !demons.is_empty() {
        return;
    }
    // All demons for this level are gone
    if state.demons_spawned < state.demons_total {
        return;
    }

    state.level_complete = true;

    if state.level >= MAX_LEVEL {
        spawn_overlay_text(&mut commands, "You Win!", Palette::HotMagenta);
    } else {
        spawn_overlay_text(&mut commands, "Level Complete!", Palette::SoftLavender);
        state.level_transition_timer =
            Some(Timer::from_seconds(LEVEL_TRANSITION_SECS, TimerMode::Once));
    }
}

fn update_level_transition(
    time: Res<Time>,
    mut commands: Commands,
    mut state: ResMut<DemonHuntState>,
    overlay_query: Query<Entity, With<OverlayText>>,
) {
    let Some(ref mut timer) = state.level_transition_timer else {
        return;
    };
    timer.tick(time.delta());
    if !timer.is_finished() {
        return;
    }

    // Clean up overlay
    for entity in &overlay_query {
        commands.entity(entity).despawn();
    }

    state.level += 1;
    advance_to_level(&mut state);
}

// ---------------------------------------------------------------------------
// UI updates
// ---------------------------------------------------------------------------

fn update_score_ui(
    state: Res<DemonHuntState>,
    mut score_query: Query<&mut Text, (With<ScoreLabel>, Without<LevelLabel>)>,
    mut level_query: Query<&mut Text, (With<LevelLabel>, Without<ScoreLabel>)>,
) {
    if !state.is_changed() {
        return;
    }
    for mut text in &mut score_query {
        **text = format!("Demons: {}", state.score);
    }
    for mut text in &mut level_query {
        **text = format!("Level {}", state.level);
    }
}

fn update_hearts_ui(
    state: Res<DemonHuntState>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Heart, &mut ImageNode)>,
) {
    if !state.is_changed() {
        return;
    }
    for (heart, mut image) in &mut query {
        if heart.0 < state.lives {
            image.image = asset_server.load("demons/heart_full.png");
        } else {
            image.image = asset_server.load("demons/heart_empty.png");
        }
    }
}

// ---------------------------------------------------------------------------
// Overlay text helper
// ---------------------------------------------------------------------------

fn spawn_overlay_text(commands: &mut Commands, message: &str, palette: Palette) {
    commands.spawn((
        OverlayText,
        DespawnOnExit(GameState::DemonHunt),
        Text::new(message),
        TextFont {
            font_size: 64.0,
            ..default()
        },
        TextColor(palette.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
    ));
}

// ---------------------------------------------------------------------------
// Navigation observers
// ---------------------------------------------------------------------------

fn on_back_click(_trigger: On<Pointer<Click>>, mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::GameSelect);
}

fn on_button_over(trigger: On<Pointer<Over>>, mut query: Query<&mut BackgroundColor>) {
    let entity = trigger.event_target();
    if let Ok(mut bg) = query.get_mut(entity) {
        *bg = BackgroundColor(Palette::HotMagenta.into());
    }
}

fn on_button_out(trigger: On<Pointer<Out>>, mut query: Query<&mut BackgroundColor>) {
    let entity = trigger.event_target();
    if let Ok(mut bg) = query.get_mut(entity) {
        *bg = BackgroundColor(Palette::ElectricPurple.into());
    }
}
