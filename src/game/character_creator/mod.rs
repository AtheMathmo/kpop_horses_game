//! Character creation screen — lets the player design their K-Pop Demon Hunter.

use bevy::prelude::*;

use crate::assets::Palette;
use face_gen::{
    ALL_EYE_STYLES, ALL_FACE_SHAPES, ALL_HAIR_STYLES, ALL_MOUTH_STYLES, ALL_SKIN_TONES, EyeStyle,
    FaceLayer, FaceShape, HairStyle, MouthStyle, SkinTone, has_front_layer,
};

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct CharacterCreatorPlugin;

impl Plugin for CharacterCreatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterSelections>()
            .init_resource::<ActiveCategory>()
            .add_systems(Startup, (spawn_ui, spawn_face_sprites))
            .add_systems(
                Update,
                (update_labels, update_category_highlights, update_preview),
            );
    }
}

// ---------------------------------------------------------------------------
// Data model — types re-exported from face_gen
// ---------------------------------------------------------------------------

/// Trait for enums that can cycle forward/backward through their variants.
trait Cyclable: Sized + Copy + 'static {
    fn variants() -> &'static [Self];
    fn index(&self) -> usize;

    fn next(self) -> Self {
        let v = Self::variants();
        v[(self.index() + 1) % v.len()]
    }

    fn prev(self) -> Self {
        let v = Self::variants();
        v[(self.index() + v.len() - 1) % v.len()]
    }

    fn display_label(&self) -> &'static str;
}

impl Cyclable for FaceShape {
    fn variants() -> &'static [Self] {
        ALL_FACE_SHAPES
    }
    fn index(&self) -> usize {
        ALL_FACE_SHAPES.iter().position(|v| v == self).unwrap()
    }
    fn display_label(&self) -> &'static str {
        self.display()
    }
}

impl Cyclable for EyeStyle {
    fn variants() -> &'static [Self] {
        ALL_EYE_STYLES
    }
    fn index(&self) -> usize {
        ALL_EYE_STYLES.iter().position(|v| v == self).unwrap()
    }
    fn display_label(&self) -> &'static str {
        self.display()
    }
}

impl Cyclable for HairStyle {
    fn variants() -> &'static [Self] {
        ALL_HAIR_STYLES
    }
    fn index(&self) -> usize {
        ALL_HAIR_STYLES.iter().position(|v| v == self).unwrap()
    }
    fn display_label(&self) -> &'static str {
        self.display()
    }
}

impl Cyclable for MouthStyle {
    fn variants() -> &'static [Self] {
        ALL_MOUTH_STYLES
    }
    fn index(&self) -> usize {
        ALL_MOUTH_STYLES.iter().position(|v| v == self).unwrap()
    }
    fn display_label(&self) -> &'static str {
        self.display()
    }
}

impl Cyclable for SkinTone {
    fn variants() -> &'static [Self] {
        ALL_SKIN_TONES
    }
    fn index(&self) -> usize {
        ALL_SKIN_TONES.iter().position(|v| v == self).unwrap()
    }
    fn display_label(&self) -> &'static str {
        self.display()
    }
}

/// Current character customisation selections.
#[derive(Resource, Debug, Clone, Default)]
pub struct CharacterSelections {
    pub face: FaceShape,
    pub eyes: EyeStyle,
    pub hair: HairStyle,
    pub mouth: MouthStyle,
    pub skin: SkinTone,
}

/// Which feature category the player is currently editing.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ActiveCategory {
    #[default]
    Face,
    Eyes,
    Hair,
    Mouth,
    Skin,
}

impl ActiveCategory {
    fn label(&self) -> &'static str {
        match self {
            Self::Face => "Face Shape",
            Self::Eyes => "Eye Style",
            Self::Hair => "Hair Style",
            Self::Mouth => "Mouth",
            Self::Skin => "Skin Tone",
        }
    }
}

// ---------------------------------------------------------------------------
// Marker components
// ---------------------------------------------------------------------------

#[derive(Component)]
struct CategoryButton(ActiveCategory);

#[derive(Component)]
struct PrevOptionButton;

#[derive(Component)]
struct NextOptionButton;

#[derive(Component)]
struct DoneButton;

#[derive(Component)]
struct CategoryLabel;

#[derive(Component)]
struct OptionLabel;

#[derive(Component)]
struct FacePreviewRoot;

#[derive(Component)]
struct FaceSpriteLayer(FaceLayer);

// ---------------------------------------------------------------------------
// UI spawning (controls only — no preview rendering)
// ---------------------------------------------------------------------------

fn spawn_ui(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|root| {
            // Title
            root.spawn((
                Text::new("Create Your K-Pop Demon Hunter"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Palette::NeonPink.into()),
            ));

            // Main content row
            root.spawn(Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::vertical(Val::Px(20.0)),
                ..default()
            })
            .with_children(|main_row| {
                spawn_category_panel(main_row);
                // Empty spacer where the world-space sprites render
                main_row.spawn(Node {
                    width: Val::Px(300.0),
                    height: Val::Px(380.0),
                    ..default()
                });
                spawn_option_panel(main_row);
            });

            // Done button
            spawn_done_button(root);
        });
}

fn spawn_category_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(8.0),
            width: Val::Px(160.0),
            ..default()
        })
        .with_children(|panel| {
            let categories = [
                ActiveCategory::Face,
                ActiveCategory::Eyes,
                ActiveCategory::Hair,
                ActiveCategory::Mouth,
                ActiveCategory::Skin,
            ];
            for cat in categories {
                spawn_category_button(panel, cat);
            }
        });
}

fn spawn_category_button(parent: &mut ChildSpawnerCommands, category: ActiveCategory) {
    parent
        .spawn((
            CategoryButton(category),
            Node {
                padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Palette::DeepViolet.into()),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(category.label()),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Palette::WhiteSmoke.into()),
            ));
        })
        .observe(on_category_click)
        .observe(on_button_over)
        .observe(on_button_out);
}

fn spawn_option_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(16.0),
            width: Val::Px(200.0),
            ..default()
        })
        .with_children(|panel| {
            // Category label
            panel.spawn((
                CategoryLabel,
                Text::new("Face Shape"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Palette::SoftLavender.into()),
            ));

            // Arrow row
            panel
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|row| {
                    // Prev arrow
                    row.spawn((
                        PrevOptionButton,
                        Node {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(Palette::ElectricPurple.into()),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("<"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Palette::WhiteSmoke.into()),
                        ));
                    })
                    .observe(on_prev_click)
                    .observe(on_button_over)
                    .observe(on_button_out);

                    // Option label
                    row.spawn((
                        OptionLabel,
                        Text::new("Oval"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Palette::WhiteSmoke.into()),
                        Node {
                            min_width: Val::Px(80.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                    ));

                    // Next arrow
                    row.spawn((
                        NextOptionButton,
                        Node {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(Palette::ElectricPurple.into()),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(">"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Palette::WhiteSmoke.into()),
                        ));
                    })
                    .observe(on_next_click)
                    .observe(on_button_over)
                    .observe(on_button_out);
                });
        });
}

fn spawn_done_button(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            DoneButton,
            Node {
                padding: UiRect::axes(Val::Px(32.0), Val::Px(14.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Palette::NeonPink.into()),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new("Ready to Hunt!"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Palette::WhiteSmoke.into()),
            ));
        })
        .observe(on_done_click)
        .observe(on_button_over)
        .observe(on_button_out);
}

// ---------------------------------------------------------------------------
// World-space face sprites
// ---------------------------------------------------------------------------

/// Sprite scale factor — PNGs are 600×760, we want ~300×380 on screen.
const SPRITE_SCALE: f32 = 0.5;

fn spawn_face_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selections: Res<CharacterSelections>,
) {
    let layers = [
        (FaceLayer::HairBack, 0.0),
        (FaceLayer::Face, 1.0),
        (FaceLayer::Eyes, 2.0),
        (FaceLayer::Mouth, 3.0),
        (FaceLayer::HairFront, 4.0),
    ];

    commands
        .spawn((
            FacePreviewRoot,
            Transform::from_scale(Vec3::splat(SPRITE_SCALE)),
            Visibility::default(),
        ))
        .with_children(|root| {
            for (layer, z) in layers {
                let show = layer != FaceLayer::HairFront || has_front_layer(selections.hair);

                let mut sprite = Sprite::default();
                if show {
                    sprite.image = asset_server.load(&layer_asset_path(layer, &selections));
                }

                root.spawn((
                    FaceSpriteLayer(layer),
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

fn layer_asset_path(layer: FaceLayer, selections: &CharacterSelections) -> String {
    match layer {
        FaceLayer::HairBack => format!("faces/hair_back/{}.png", selections.hair.label()),
        FaceLayer::Face => format!(
            "faces/face/{}_{}.png",
            selections.face.label(),
            selections.skin.label()
        ),
        FaceLayer::Eyes => format!(
            "faces/eyes/{}_{}.png",
            selections.eyes.label(),
            selections.skin.label()
        ),
        FaceLayer::Mouth => format!("faces/mouth/{}.png", selections.mouth.label()),
        FaceLayer::HairFront => format!("faces/hair_front/{}.png", selections.hair.label()),
    }
}

// ---------------------------------------------------------------------------
// Interaction observers
// ---------------------------------------------------------------------------

fn on_category_click(
    trigger: On<Pointer<Click>>,
    query: Query<&CategoryButton>,
    mut active: ResMut<ActiveCategory>,
) {
    let entity = trigger.event_target();
    if let Ok(cat_btn) = query.get(entity) {
        *active = cat_btn.0;
    }
}

fn on_prev_click(
    _trigger: On<Pointer<Click>>,
    active: Res<ActiveCategory>,
    mut selections: ResMut<CharacterSelections>,
) {
    match *active {
        ActiveCategory::Face => selections.face = selections.face.prev(),
        ActiveCategory::Eyes => selections.eyes = selections.eyes.prev(),
        ActiveCategory::Hair => selections.hair = selections.hair.prev(),
        ActiveCategory::Mouth => selections.mouth = selections.mouth.prev(),
        ActiveCategory::Skin => selections.skin = selections.skin.prev(),
    }
}

fn on_next_click(
    _trigger: On<Pointer<Click>>,
    active: Res<ActiveCategory>,
    mut selections: ResMut<CharacterSelections>,
) {
    match *active {
        ActiveCategory::Face => selections.face = selections.face.next(),
        ActiveCategory::Eyes => selections.eyes = selections.eyes.next(),
        ActiveCategory::Hair => selections.hair = selections.hair.next(),
        ActiveCategory::Mouth => selections.mouth = selections.mouth.next(),
        ActiveCategory::Skin => selections.skin = selections.skin.next(),
    }
}

fn on_done_click(_trigger: On<Pointer<Click>>, selections: Res<CharacterSelections>) {
    info!(
        "Character ready! Face: {}, Eyes: {}, Hair: {}, Mouth: {}, Skin: {}",
        selections.face.display(),
        selections.eyes.display(),
        selections.hair.display(),
        selections.mouth.display(),
        selections.skin.display(),
    );
}

fn on_button_over(trigger: On<Pointer<Over>>, mut query: Query<&mut BackgroundColor>) {
    let entity = trigger.event_target();
    if let Ok(mut bg) = query.get_mut(entity) {
        *bg = BackgroundColor(Palette::HotMagenta.into());
    }
}

fn on_button_out(
    trigger: On<Pointer<Out>>,
    mut query: Query<&mut BackgroundColor>,
    cat_query: Query<&CategoryButton>,
    done_query: Query<&DoneButton>,
    active: Res<ActiveCategory>,
) {
    let entity = trigger.event_target();
    if let Ok(cat_btn) = cat_query.get(entity) {
        let color = if cat_btn.0 == *active {
            Palette::NeonPink
        } else {
            Palette::DeepViolet
        };
        if let Ok(mut bg) = query.get_mut(entity) {
            *bg = BackgroundColor(color.into());
        }
    } else if done_query.get(entity).is_ok() {
        if let Ok(mut bg) = query.get_mut(entity) {
            *bg = BackgroundColor(Palette::NeonPink.into());
        }
    } else if let Ok(mut bg) = query.get_mut(entity) {
        *bg = BackgroundColor(Palette::ElectricPurple.into());
    }
}

// ---------------------------------------------------------------------------
// Update systems
// ---------------------------------------------------------------------------

fn update_labels(
    active: Res<ActiveCategory>,
    selections: Res<CharacterSelections>,
    mut cat_label: Query<&mut Text, (With<CategoryLabel>, Without<OptionLabel>)>,
    mut opt_label: Query<&mut Text, (With<OptionLabel>, Without<CategoryLabel>)>,
) {
    if !active.is_changed() && !selections.is_changed() {
        return;
    }

    for mut text in &mut cat_label {
        **text = active.label().to_string();
    }

    let option_text = match *active {
        ActiveCategory::Face => selections.face.display_label(),
        ActiveCategory::Eyes => selections.eyes.display_label(),
        ActiveCategory::Hair => selections.hair.display_label(),
        ActiveCategory::Mouth => selections.mouth.display_label(),
        ActiveCategory::Skin => selections.skin.display_label(),
    };

    for mut text in &mut opt_label {
        **text = option_text.to_string();
    }
}

fn update_category_highlights(
    active: Res<ActiveCategory>,
    mut query: Query<(&CategoryButton, &mut BackgroundColor)>,
) {
    if !active.is_changed() {
        return;
    }

    for (cat_btn, mut bg) in &mut query {
        let color = if cat_btn.0 == *active {
            Palette::NeonPink
        } else {
            Palette::DeepViolet
        };
        *bg = BackgroundColor(color.into());
    }
}

fn update_preview(
    selections: Res<CharacterSelections>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&FaceSpriteLayer, &mut Sprite, &mut Visibility)>,
) {
    if !selections.is_changed() {
        return;
    }

    for (layer, mut sprite, mut vis) in &mut query {
        // Hide hair_front for styles that don't have one
        if layer.0 == FaceLayer::HairFront {
            if has_front_layer(selections.hair) {
                *vis = Visibility::Inherited;
            } else {
                *vis = Visibility::Hidden;
                continue;
            }
        }

        let path = layer_asset_path(layer.0, &selections);
        sprite.image = asset_server.load(&path);
    }
}
