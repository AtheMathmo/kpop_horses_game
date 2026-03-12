//! Horse brushing mini-game — drag to brush dirt off your horse!

use bevy::prelude::*;

use bevy::state::state_scoped::DespawnOnExit;

use crate::assets::Palette;
use crate::game::effects::{SpawnSparkles, spawn_celebration_sparkles};
use crate::game::{GameState, HorseSelections, horse_layer_asset_path, horse_layer_visible};
use face_gen::HorseLayer;

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct HorseBrushingPlugin;

impl Plugin for HorseBrushingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BrushActive>()
            .init_resource::<BrushingProgress>()
            .add_systems(
                OnEnter(GameState::HorseBrushing),
                (spawn_brushing_ui, spawn_brushing_horse),
            )
            .add_systems(
                Update,
                (update_dirt_visuals, update_progress, check_completion)
                    .run_if(in_state(GameState::HorseBrushing)),
            );
    }
}

// ---------------------------------------------------------------------------
// Components & Resources
// ---------------------------------------------------------------------------

#[derive(Component)]
struct DirtSpot {
    opacity: f32,
}

#[derive(Component)]
struct BrushingHorseRoot;

#[derive(Component)]
struct DirtOverlay;

#[derive(Component)]
struct ProgressBarFill;

#[derive(Component)]
struct ProgressLabel;

#[derive(Component)]
struct CelebrationText;

#[derive(Resource, Default)]
struct BrushActive(bool);

#[derive(Resource, Default)]
struct BrushingProgress {
    total: u32,
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const DIRT_SPOT_COUNT: u32 = 25;
const DIRT_FADE_RATE: f32 = 0.5;
const HORSE_SCALE: f32 = 0.75;
const CELEBRATION_SPARKLE_COUNT: u32 = 25;

/// The overlay area dimensions (matches the UI spacer for the horse).
const OVERLAY_W: f32 = 420.0;
const OVERLAY_H: f32 = 380.0;

/// Ellipse radii for scattering dirt spots within the horse body.
const BODY_RX: f32 = 140.0;
const BODY_RY: f32 = 100.0;

/// Min/max diameter for dirt circles.
const SPOT_MIN_SIZE: f32 = 35.0;
const SPOT_MAX_SIZE: f32 = 60.0;

// ---------------------------------------------------------------------------
// Simple pseudo-random number generator (no rand crate needed)
// ---------------------------------------------------------------------------

struct SimpleRng(u64);

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        // LCG parameters from Numerical Recipes
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.0
    }

    /// Returns a float in [0, 1).
    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Returns a float in [lo, hi).
    fn range_f32(&mut self, lo: f32, hi: f32) -> f32 {
        lo + self.next_f32() * (hi - lo)
    }
}

/// Generate random dirt spot positions and sizes inside an elliptical body region.
/// Shrinks the sampling ellipse by each spot's radius so the full circle stays in bounds.
fn generate_dirt_spots(count: u32) -> Vec<(f32, f32, f32)> {
    let mut rng = SimpleRng::new(0xDEAD_BEEF_CAFE);
    let mut spots = Vec::with_capacity(count as usize);
    let center_x = OVERLAY_W / 2.0;
    let center_y = OVERLAY_H / 2.0;

    for _ in 0..count {
        let size = rng.range_f32(SPOT_MIN_SIZE, SPOT_MAX_SIZE);
        let half = size / 2.0;
        // Shrink the ellipse so the full circle stays inside
        let rx = (BODY_RX - half).max(0.0);
        let ry = (BODY_RY - half).max(0.0);
        // Rejection-sample inside the shrunken ellipse
        loop {
            let x = rng.range_f32(-rx, rx);
            let y = rng.range_f32(-ry, ry);
            if rx > 0.0 && ry > 0.0 && (x / rx).powi(2) + (y / ry).powi(2) <= 1.0 {
                spots.push((center_x + x, center_y + y, size));
                break;
            }
        }
    }
    spots
}

// ---------------------------------------------------------------------------
// Spawn systems (OnEnter)
// ---------------------------------------------------------------------------

fn spawn_brushing_ui(mut commands: Commands) {
    commands.insert_resource(BrushingProgress {
        total: DIRT_SPOT_COUNT,
    });
    commands.insert_resource(BrushActive(false));

    let dirt_spots = generate_dirt_spots(DIRT_SPOT_COUNT);

    commands
        .spawn((
            DespawnOnExit(GameState::HorseBrushing),
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
            // Top bar: title + back button
            root.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    Text::new("Time to Brush!"),
                    TextFont {
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(Palette::NeonPink.into()),
                ));

                // Back button
                row.spawn((
                    Node {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Palette::ElectricPurple.into()),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Back"),
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

            // Overlay container — absolutely positioned circles over the horse
            root.spawn((
                DirtOverlay,
                Node {
                    width: Val::Px(OVERLAY_W),
                    height: Val::Px(OVERLAY_H),
                    position_type: PositionType::Relative,
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                },
            ))
            .observe(on_overlay_press)
            .observe(on_overlay_release)
            .with_children(|overlay| {
                for (x, y, size) in &dirt_spots {
                    let half = size / 2.0;
                    overlay
                        .spawn((
                            DirtSpot { opacity: 1.0 },
                            Node {
                                width: Val::Px(*size),
                                height: Val::Px(*size),
                                position_type: PositionType::Absolute,
                                left: Val::Px(x - half),
                                top: Val::Px(y - half),
                                border_radius: BorderRadius::all(Val::Percent(50.0)),
                                ..default()
                            },
                            BackgroundColor(Palette::overlay(0.55)),
                        ))
                        .observe(on_dirt_press)
                        .observe(on_dirt_over)
                        .observe(on_dirt_move);
                }
            });

            // Progress bar
            root.spawn(Node {
                width: Val::Percent(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(6.0),
                ..default()
            })
            .with_children(|bar_area| {
                // Track
                bar_area
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(24.0),
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(Palette::DeepViolet.into()),
                    ))
                    .with_children(|track| {
                        // Fill
                        track.spawn((
                            ProgressBarFill,
                            Node {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                border_radius: BorderRadius::all(Val::Px(12.0)),
                                ..default()
                            },
                            BackgroundColor(Palette::NeonPink.into()),
                        ));
                    });

                // Label
                bar_area.spawn((
                    ProgressLabel,
                    Text::new("Cleanliness: 0%"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Palette::SoftLavender.into()),
                ));
            });
        });
}

fn spawn_brushing_horse(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selections: Res<HorseSelections>,
) {
    let layers = [
        (HorseLayer::Tail, -1.0),
        (HorseLayer::Body, 0.0),
        (HorseLayer::Markings, 1.0),
        (HorseLayer::Mane, 2.0),
        (HorseLayer::BodyFront, 3.0),
        (HorseLayer::Saddle, 4.0),
        (HorseLayer::Bridle, 5.0),
    ];

    commands
        .spawn((
            BrushingHorseRoot,
            DespawnOnExit(GameState::HorseBrushing),
            Transform::from_scale(Vec3::splat(HORSE_SCALE)),
            Visibility::default(),
        ))
        .with_children(|root| {
            for (layer, z) in layers {
                let show = horse_layer_visible(layer, &selections);
                let mut sprite = Sprite::default();
                if show {
                    sprite.image = asset_server.load(horse_layer_asset_path(layer, &selections));
                }

                root.spawn((
                    sprite,
                    Transform::from_xyz(0.0, 0.0, z),
                    if show {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    },
                ));
            }
        });
}

// ---------------------------------------------------------------------------
// Brush interaction observers
// ---------------------------------------------------------------------------

fn on_overlay_press(_trigger: On<Pointer<Press>>, mut brush: ResMut<BrushActive>) {
    brush.0 = true;
}

fn on_overlay_release(_trigger: On<Pointer<Release>>, mut brush: ResMut<BrushActive>) {
    brush.0 = false;
}

fn clean_dirt_spot(entity: Entity, dirt_query: &mut Query<&mut DirtSpot>, commands: &mut Commands) {
    if let Ok(mut dirt) = dirt_query.get_mut(entity) {
        if dirt.opacity <= 0.0 {
            return;
        }
        dirt.opacity = (dirt.opacity - DIRT_FADE_RATE).max(0.0);
        commands.trigger(SpawnSparkles { target: entity });
        if dirt.opacity <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Clean the spot the player first clicks on (Over already fired before Press).
fn on_dirt_press(
    trigger: On<Pointer<Press>>,
    mut brush: ResMut<BrushActive>,
    mut dirt_query: Query<&mut DirtSpot>,
    mut commands: Commands,
) {
    brush.0 = true;
    let entity = trigger.event_target();
    clean_dirt_spot(entity, &mut dirt_query, &mut commands);
}

fn on_dirt_over(
    trigger: On<Pointer<Over>>,
    brush: Res<BrushActive>,
    mut dirt_query: Query<&mut DirtSpot>,
    mut commands: Commands,
) {
    if !brush.0 {
        return;
    }
    let entity = trigger.event_target();
    clean_dirt_spot(entity, &mut dirt_query, &mut commands);
}

fn on_dirt_move(
    trigger: On<Pointer<Move>>,
    brush: Res<BrushActive>,
    mut dirt_query: Query<&mut DirtSpot>,
    mut commands: Commands,
) {
    if !brush.0 {
        return;
    }
    let entity = trigger.event_target();
    clean_dirt_spot(entity, &mut dirt_query, &mut commands);
}

// ---------------------------------------------------------------------------
// Update systems
// ---------------------------------------------------------------------------

fn update_dirt_visuals(mut query: Query<(&DirtSpot, &mut BackgroundColor)>) {
    for (dirt, mut bg) in &mut query {
        *bg = BackgroundColor(Palette::overlay(0.55 * dirt.opacity));
    }
}

fn update_progress(
    remaining_query: Query<(), With<DirtSpot>>,
    progress: Res<BrushingProgress>,
    mut fill_query: Query<&mut Node, With<ProgressBarFill>>,
    mut label_query: Query<&mut Text, With<ProgressLabel>>,
) {
    let remaining = remaining_query.iter().count() as u32;
    let cleaned = progress.total.saturating_sub(remaining);
    let pct = if progress.total > 0 {
        (cleaned as f32 / progress.total as f32 * 100.0).min(100.0)
    } else {
        0.0
    };

    for mut node in &mut fill_query {
        node.width = Val::Percent(pct);
    }

    for mut text in &mut label_query {
        **text = format!("Cleanliness: {:.0}%", pct);
    }
}

fn check_completion(
    remaining_query: Query<(), With<DirtSpot>>,
    celebration_query: Query<(), With<CelebrationText>>,
    mut commands: Commands,
    horse_query: Query<&GlobalTransform, With<BrushingHorseRoot>>,
) {
    if remaining_query.iter().count() > 0 {
        return;
    }
    if !celebration_query.is_empty() {
        return;
    }

    // Celebration burst of sparkles from horse center
    if let Ok(horse_gt) = horse_query.single() {
        let center = horse_gt.translation().truncate();
        spawn_celebration_sparkles(&mut commands, center, CELEBRATION_SPARKLE_COUNT);
    }

    // "Good Job!" text
    commands.spawn((
        CelebrationText,
        DespawnOnExit(GameState::HorseBrushing),
        Text::new("Good Job!"),
        TextFont {
            font_size: 64.0,
            ..default()
        },
        TextColor(Palette::HotMagenta.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(35.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-120.0)),
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
