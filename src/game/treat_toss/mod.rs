//! Treat toss mini-game — slingshot treats toward the horse!

use bevy::prelude::*;

use bevy::state::state_scoped::DespawnOnExit;

use crate::assets::Palette;
use crate::game::effects::spawn_celebration_sparkles;
use crate::game::{GameState, HorseSelections, horse_layer_asset_path, horse_layer_visible};
use face_gen::HorseLayer;

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct TreatTossPlugin;

impl Plugin for TreatTossPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreatScore>()
            .init_resource::<LaunchState>()
            .add_systems(
                OnEnter(GameState::TreatToss),
                (spawn_treat_toss_ui, spawn_treat_toss_horse, spawn_slingshot),
            )
            .add_systems(
                Update,
                (
                    update_slingshot_drag,
                    update_tether_visual,
                    update_treats,
                    check_treat_collision,
                    update_score_label,
                )
                    .run_if(in_state(GameState::TreatToss)),
            );
    }
}

// ---------------------------------------------------------------------------
// Components & Resources
// ---------------------------------------------------------------------------

#[derive(Component)]
struct TreatTossHorseRoot;

#[derive(Component)]
struct Treat {
    velocity: Vec2,
}

#[derive(Component)]
struct ScoreLabel;

#[derive(Component)]
struct HorseMouthZone;

/// The fixed anchor point of the slingshot.
#[derive(Component)]
struct SlingAnchor;

/// The draggable handle the player pulls back.
#[derive(Component)]
struct SlingHandle;

/// The tether line sprite stretched between anchor and handle.
#[derive(Component)]
struct SlingTether;

#[derive(Resource, Default)]
struct TreatScore(u32);

#[derive(Resource)]
struct LaunchState {
    dragging: bool,
    anchor: Vec2,
}

impl Default for LaunchState {
    fn default() -> Self {
        Self {
            dragging: false,
            anchor: Vec2::new(ANCHOR_X, ANCHOR_Y),
        }
    }
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const GRAVITY: f32 = -400.0;
const LAUNCH_SCALE: f32 = 3.5;
const HORSE_SCALE: f32 = 0.6;
const HORSE_X_OFFSET: f32 = 200.0;
/// Mouth position in horse-local coords (origin = sprite center, y-up).
/// The horse faces left, so the mouth is upper-left: roughly x=-300, y=250
/// in the 1000×800 sprite.
const MOUTH_OFFSET: Vec2 = Vec2::new(-300.0, 200.0);
const MOUTH_RADIUS: f32 = 80.0;
const TREAT_SIZE: f32 = 14.0;

/// Slingshot anchor position in world space.
const ANCHOR_X: f32 = -280.0;
const ANCHOR_Y: f32 = -40.0;

/// Max drag distance from anchor.
const TETHER_RADIUS: f32 = 120.0;

/// Visual sizes.
const ANCHOR_SIZE: f32 = 16.0;
const HANDLE_SIZE: f32 = 28.0;
const TETHER_THICKNESS: f32 = 4.0;

// ---------------------------------------------------------------------------
// Spawn systems
// ---------------------------------------------------------------------------

fn spawn_treat_toss_ui(mut commands: Commands) {
    commands.insert_resource(TreatScore(0));
    commands.insert_resource(LaunchState::default());

    commands
        .spawn((
            DespawnOnExit(GameState::TreatToss),
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
            // Top bar: title + score
            root.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    Text::new("Feed Your Horse!"),
                    TextFont {
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(Palette::NeonPink.into()),
                ));

                row.spawn((
                    ScoreLabel,
                    Text::new("Treats Fed: 0"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Palette::SoftLavender.into()),
                ));
            });

            // Spacer for the world-space area
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

fn spawn_slingshot(mut commands: Commands) {
    let anchor = Vec2::new(ANCHOR_X, ANCHOR_Y);

    // Anchor dot (fixed)
    commands.spawn((
        SlingAnchor,
        DespawnOnExit(GameState::TreatToss),
        Sprite {
            color: Palette::SoftLavender.into(),
            custom_size: Some(Vec2::splat(ANCHOR_SIZE)),
            ..default()
        },
        Transform::from_translation(anchor.extend(2.0)),
    ));

    // Tether line (stretched sprite between anchor and handle)
    commands.spawn((
        SlingTether,
        DespawnOnExit(GameState::TreatToss),
        Sprite {
            color: Palette::SoftLavender.into(),
            custom_size: Some(Vec2::new(1.0, TETHER_THICKNESS)),
            ..default()
        },
        Transform::from_translation(anchor.extend(1.0)),
        Visibility::Hidden,
    ));

    // Draggable handle
    commands.spawn((
        SlingHandle,
        DespawnOnExit(GameState::TreatToss),
        Sprite {
            color: Palette::ElectricPurple.into(),
            custom_size: Some(Vec2::splat(HANDLE_SIZE)),
            ..default()
        },
        Transform::from_translation(anchor.extend(3.0)),
    ));
}

fn spawn_treat_toss_horse(
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
            TreatTossHorseRoot,
            DespawnOnExit(GameState::TreatToss),
            Transform::from_xyz(HORSE_X_OFFSET, 0.0, 0.0).with_scale(Vec3::splat(HORSE_SCALE)),
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

            // Invisible mouth zone (child of horse root)
            root.spawn((
                HorseMouthZone,
                Transform::from_xyz(MOUTH_OFFSET.x, MOUTH_OFFSET.y, 8.0),
                Visibility::Hidden,
            ));
        });
}

// ---------------------------------------------------------------------------
// Slingshot drag system (runs every frame)
// ---------------------------------------------------------------------------

fn update_slingshot_drag(
    mut launch: ResMut<LaunchState>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut handle_query: Query<(&mut Transform, &mut Sprite), With<SlingHandle>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_screen) = window.cursor_position() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera_query.single() else {
        return;
    };
    let Ok(cursor_world) = camera.viewport_to_world_2d(cam_transform, cursor_screen) else {
        return;
    };

    let Ok((mut handle_tf, mut handle_sprite)) = handle_query.single_mut() else {
        return;
    };

    let anchor = launch.anchor;
    let handle_pos = handle_tf.translation.truncate();

    // Start dragging when clicking near the handle
    if mouse.just_pressed(MouseButton::Left) {
        let dist_to_handle = cursor_world.distance(handle_pos);
        if dist_to_handle < HANDLE_SIZE * 1.5 {
            launch.dragging = true;
            handle_sprite.color = Palette::HotMagenta.into();
        }
    }

    // While dragging, move handle clamped to tether radius
    if launch.dragging {
        let offset = cursor_world - anchor;
        let clamped = if offset.length() > TETHER_RADIUS {
            anchor + offset.normalize() * TETHER_RADIUS
        } else {
            cursor_world
        };
        handle_tf.translation.x = clamped.x;
        handle_tf.translation.y = clamped.y;
    }

    // Release — launch the treat
    if mouse.just_released(MouseButton::Left) && launch.dragging {
        launch.dragging = false;
        handle_sprite.color = Palette::ElectricPurple.into();

        let pull_vec = anchor - handle_tf.translation.truncate();
        let velocity = pull_vec * LAUNCH_SCALE;

        if velocity.length() > 30.0 {
            spawn_treat(&mut commands, anchor, velocity);
        }

        // Snap handle back to anchor
        handle_tf.translation.x = anchor.x;
        handle_tf.translation.y = anchor.y;
    }
}

// ---------------------------------------------------------------------------
// Tether line visual
// ---------------------------------------------------------------------------

fn update_tether_visual(
    launch: Res<LaunchState>,
    handle_query: Query<&Transform, With<SlingHandle>>,
    mut tether_query: Query<
        (&mut Transform, &mut Sprite, &mut Visibility),
        (With<SlingTether>, Without<SlingHandle>),
    >,
) {
    let Ok(handle_tf) = handle_query.single() else {
        return;
    };
    let Ok((mut tether_tf, mut tether_sprite, mut tether_vis)) = tether_query.single_mut() else {
        return;
    };

    let anchor = launch.anchor;
    let handle_pos = handle_tf.translation.truncate();
    let diff = handle_pos - anchor;
    let dist = diff.length();

    if dist < 2.0 {
        *tether_vis = Visibility::Hidden;
        return;
    }

    *tether_vis = Visibility::Inherited;

    // Position at midpoint between anchor and handle
    let midpoint = (anchor + handle_pos) / 2.0;
    tether_tf.translation.x = midpoint.x;
    tether_tf.translation.y = midpoint.y;
    tether_tf.translation.z = 1.0;

    // Rotate to point from anchor to handle
    let angle = diff.y.atan2(diff.x);
    tether_tf.rotation = Quat::from_rotation_z(angle);

    // Stretch to span the distance
    if let Some(ref mut size) = tether_sprite.custom_size {
        size.x = dist;
    }
}

// ---------------------------------------------------------------------------
// Treat spawning & physics
// ---------------------------------------------------------------------------

fn spawn_treat(commands: &mut Commands, pos: Vec2, velocity: Vec2) {
    commands.spawn((
        Treat { velocity },
        Sprite {
            color: Palette::NeonPink.into(),
            custom_size: Some(Vec2::splat(TREAT_SIZE)),
            ..default()
        },
        Transform::from_translation(pos.extend(5.0)),
    ));
}

fn update_treats(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Treat, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut treat, mut transform) in &mut query {
        treat.velocity.y += GRAVITY * dt;
        transform.translation.x += treat.velocity.x * dt;
        transform.translation.y += treat.velocity.y * dt;

        // Despawn if fallen off screen
        if transform.translation.y < -400.0
            || transform.translation.x > 600.0
            || transform.translation.x < -600.0
        {
            commands.entity(entity).despawn();
        }
    }
}

fn check_treat_collision(
    mut commands: Commands,
    treat_query: Query<(Entity, &Transform), With<Treat>>,
    mouth_query: Query<&GlobalTransform, With<HorseMouthZone>>,
    mut score: ResMut<TreatScore>,
) {
    let Ok(mouth_gt) = mouth_query.single() else {
        return;
    };
    let mouth_pos = mouth_gt.translation().truncate();

    for (entity, transform) in &treat_query {
        let treat_pos = transform.translation.truncate();
        let dist = treat_pos.distance(mouth_pos);
        if dist < MOUTH_RADIUS {
            commands.entity(entity).despawn();
            score.0 += 1;
            spawn_celebration_sparkles(&mut commands, mouth_pos, 8);
        }
    }
}

fn update_score_label(score: Res<TreatScore>, mut query: Query<&mut Text, With<ScoreLabel>>) {
    if !score.is_changed() {
        return;
    }
    for mut text in &mut query {
        **text = format!("Treats Fed: {}", score.0);
    }
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
