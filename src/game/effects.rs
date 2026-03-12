//! Shared visual effects — sparkle bursts reused across mini-games.

use bevy::prelude::*;

use crate::assets::Palette;

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_sparkles)
            .add_observer(observe_spawn_sparkles);
    }
}

// ---------------------------------------------------------------------------
// Components & Events
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct Sparkle {
    pub lifetime: Timer,
    pub velocity: Vec2,
}

/// Trigger this event to spawn a burst of sparkles at the given entity's
/// position.
#[derive(Event)]
pub struct SpawnSparkles {
    pub target: Entity,
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const SPARKLE_LIFETIME: f32 = 0.5;
pub const SPARKLE_COUNT: u32 = 4;

// ---------------------------------------------------------------------------
// Observer
// ---------------------------------------------------------------------------

fn observe_spawn_sparkles(
    trigger: On<SpawnSparkles>,
    transform_query: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    let target = trigger.event().target;
    let Ok(global) = transform_query.get(target) else {
        return;
    };
    let pos = global.translation().truncate();
    let sparkle_colors = [
        Palette::NeonPink,
        Palette::HotMagenta,
        Palette::SoftLavender,
    ];

    for i in 0..SPARKLE_COUNT {
        let angle = std::f32::consts::TAU * (i as f32 / SPARKLE_COUNT as f32)
            + (pos.x * 13.7 + pos.y * 7.3).sin() * 0.5;
        let speed = 80.0 + (i as f32 * 20.0);
        let velocity = Vec2::new(angle.cos() * speed, angle.sin().abs() * speed + 40.0);
        let color = sparkle_colors[i as usize % sparkle_colors.len()];

        commands.spawn((
            Sparkle {
                lifetime: Timer::from_seconds(SPARKLE_LIFETIME, TimerMode::Once),
                velocity,
            },
            Sprite {
                color: color.into(),
                custom_size: Some(Vec2::splat(6.0)),
                ..default()
            },
            Transform::from_translation(pos.extend(10.0)),
        ));
    }
}

// ---------------------------------------------------------------------------
// Update system
// ---------------------------------------------------------------------------

fn update_sparkles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sparkle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut sparkle, mut transform, mut sprite) in &mut query {
        sparkle.lifetime.tick(time.delta());
        let dt = time.delta_secs();

        transform.translation.x += sparkle.velocity.x * dt;
        transform.translation.y += sparkle.velocity.y * dt;
        sparkle.velocity *= 0.95;

        let alpha = 1.0 - sparkle.lifetime.fraction();
        sprite.color = sprite.color.with_alpha(alpha);

        if sparkle.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawn a celebration burst of sparkles at a world position.
pub fn spawn_celebration_sparkles(commands: &mut Commands, center: Vec2, count: u32) {
    let sparkle_colors = [
        Palette::NeonPink,
        Palette::HotMagenta,
        Palette::SoftLavender,
    ];
    for i in 0..count {
        let angle = std::f32::consts::TAU * (i as f32 / count as f32);
        let speed = 120.0 + (i as f32 * 5.0);
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        let color = sparkle_colors[i as usize % sparkle_colors.len()];

        commands.spawn((
            Sparkle {
                lifetime: Timer::from_seconds(0.8, TimerMode::Once),
                velocity,
            },
            Sprite {
                color: color.into(),
                custom_size: Some(Vec2::splat(8.0)),
                ..default()
            },
            Transform::from_translation(center.extend(10.0)),
        ));
    }
}
