//! Game selection screen — pick a mini-game to play with your horse.

use bevy::prelude::*;

use bevy::state::state_scoped::DespawnOnExit;

use crate::assets::Palette;
use crate::game::{GameState, HorseSelections, horse_layer_asset_path, horse_layer_visible};
use face_gen::HorseLayer;

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct GameSelectPlugin;

impl Plugin for GameSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameSelect),
            (spawn_game_select_ui, spawn_game_select_horse),
        );
    }
}

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[derive(Component)]
struct GameSelectHorseRoot;

const HORSE_SCALE: f32 = 0.6;

// ---------------------------------------------------------------------------
// Spawn systems
// ---------------------------------------------------------------------------

fn spawn_game_select_ui(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::GameSelect),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
        ))
        .with_children(|root| {
            // Title
            root.spawn((
                Text::new("Choose Your Activity!"),
                TextFont {
                    font_size: 42.0,
                    ..default()
                },
                TextColor(Palette::NeonPink.into()),
            ));

            // Spacer for horse sprites (rendered in world space)
            root.spawn(Node {
                width: Val::Px(400.0),
                height: Val::Px(300.0),
                ..default()
            });

            // Game buttons row
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                ..default()
            })
            .with_children(|row| {
                spawn_game_button(row, "Brush Your Horse", GameState::HorseBrushing);
                spawn_game_button(row, "Treat Toss", GameState::TreatToss);
                spawn_game_button(row, "Demon Hunt", GameState::DemonHunt);
            });

            // Back button
            root.spawn((
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Palette::ElectricPurple.into()),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("Back to Creator"),
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

fn spawn_game_button(parent: &mut ChildSpawnerCommands, label: &str, target_state: GameState) {
    parent
        .spawn((
            GameButton(target_state),
            Node {
                padding: UiRect::axes(Val::Px(32.0), Val::Px(18.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Palette::ElectricPurple.into()),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Palette::WhiteSmoke.into()),
            ));
        })
        .observe(on_game_click)
        .observe(on_button_over)
        .observe(on_button_out);
}

#[derive(Component)]
struct GameButton(GameState);

fn spawn_game_select_horse(
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
            GameSelectHorseRoot,
            DespawnOnExit(GameState::GameSelect),
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
// Interaction observers
// ---------------------------------------------------------------------------

fn on_game_click(
    trigger: On<Pointer<Click>>,
    query: Query<&GameButton>,
    mut next: ResMut<NextState<GameState>>,
) {
    let entity = trigger.event_target();
    if let Ok(btn) = query.get(entity) {
        next.set(btn.0);
    }
}

fn on_back_click(_trigger: On<Pointer<Click>>, mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::CharacterCreator);
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
