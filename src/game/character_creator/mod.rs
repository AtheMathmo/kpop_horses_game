//! Character creation screen — lets the player design their K-Pop Demon Hunter and horse.

use bevy::prelude::*;

use crate::assets::Palette;
use face_gen::{
    ALL_COAT_COLOURS, ALL_COAT_STYLES, ALL_EYE_STYLES, ALL_FACE_SHAPES, ALL_HAIR_STYLES,
    ALL_MANE_STYLES, ALL_MOUTH_STYLES, ALL_SKIN_TONES, ALL_TACK_STYLES, CoatColour, CoatStyle,
    EyeStyle, FaceLayer, FaceShape, HairStyle, HorseLayer, ManeStyle, MouthStyle, SkinTone,
    TackStyle, has_markings, has_tack,
};

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct CharacterCreatorPlugin;

impl Plugin for CharacterCreatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterSelections>()
            .init_resource::<HorseSelections>()
            .init_resource::<ActiveCategory>()
            .init_resource::<ActiveTab>()
            .add_systems(Startup, (spawn_ui, spawn_face_sprites, spawn_horse_sprites))
            .add_systems(
                Update,
                (
                    update_labels,
                    update_category_highlights,
                    update_tab_highlights,
                    update_face_preview,
                    update_horse_preview,
                    update_preview_visibility,
                    rebuild_categories_on_tab_change,
                ),
            );
    }
}

// ---------------------------------------------------------------------------
// Data model
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

impl Cyclable for CoatColour {
    fn variants() -> &'static [Self] {
        ALL_COAT_COLOURS
    }
    fn index(&self) -> usize {
        ALL_COAT_COLOURS.iter().position(|v| v == self).unwrap()
    }
    fn display_label(&self) -> &'static str {
        self.display()
    }
}

impl Cyclable for CoatStyle {
    fn variants() -> &'static [Self] {
        ALL_COAT_STYLES
    }
    fn index(&self) -> usize {
        ALL_COAT_STYLES.iter().position(|v| v == self).unwrap()
    }
    fn display_label(&self) -> &'static str {
        self.display()
    }
}

impl Cyclable for ManeStyle {
    fn variants() -> &'static [Self] {
        ALL_MANE_STYLES
    }
    fn index(&self) -> usize {
        ALL_MANE_STYLES.iter().position(|v| v == self).unwrap()
    }
    fn display_label(&self) -> &'static str {
        self.display()
    }
}

impl Cyclable for TackStyle {
    fn variants() -> &'static [Self] {
        ALL_TACK_STYLES
    }
    fn index(&self) -> usize {
        ALL_TACK_STYLES.iter().position(|v| v == self).unwrap()
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

/// Current horse customisation selections.
#[derive(Resource, Debug, Clone, Default)]
pub struct HorseSelections {
    pub coat_colour: CoatColour,
    pub coat_style: CoatStyle,
    pub mane: ManeStyle,
    pub tack: TackStyle,
}

/// Top-level tab: Character or Horse.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ActiveTab {
    #[default]
    Character,
    Horse,
}

/// Which feature category the player is currently editing.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ActiveCategory {
    // Character categories
    #[default]
    Face,
    Eyes,
    Hair,
    Mouth,
    Skin,
    // Horse categories
    CoatColour,
    CoatStyle,
    Mane,
    Tack,
}

impl ActiveCategory {
    fn label(&self) -> &'static str {
        match self {
            Self::Face => "Face Shape",
            Self::Eyes => "Eye Style",
            Self::Hair => "Hair Style",
            Self::Mouth => "Mouth",
            Self::Skin => "Skin Tone",
            Self::CoatColour => "Coat Colour",
            Self::CoatStyle => "Coat Style",
            Self::Mane => "Mane Style",
            Self::Tack => "Tack",
        }
    }
}

const CHARACTER_CATEGORIES: &[ActiveCategory] = &[
    ActiveCategory::Face,
    ActiveCategory::Eyes,
    ActiveCategory::Hair,
    ActiveCategory::Mouth,
    ActiveCategory::Skin,
];

const HORSE_CATEGORIES: &[ActiveCategory] = &[
    ActiveCategory::CoatColour,
    ActiveCategory::CoatStyle,
    ActiveCategory::Mane,
    ActiveCategory::Tack,
];

// ---------------------------------------------------------------------------
// Marker components
// ---------------------------------------------------------------------------

#[derive(Component)]
struct TabButton(ActiveTab);

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

#[derive(Component)]
struct HorsePreviewRoot;

#[derive(Component)]
struct HorseSpriteLayer(HorseLayer);

/// Marker for the category panel container so we can despawn/rebuild its children.
#[derive(Component)]
struct CategoryPanel;

// ---------------------------------------------------------------------------
// UI spawning
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

            // Tab row (Character | Horse)
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            })
            .with_children(|row| {
                spawn_tab_button(row, ActiveTab::Character, "Character");
                spawn_tab_button(row, ActiveTab::Horse, "Horse");
            });

            // Main content row
            root.spawn(Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            })
            .with_children(|main_row| {
                spawn_category_panel(main_row);
                // Empty spacer where the world-space sprites render
                main_row.spawn(Node {
                    width: Val::Px(400.0),
                    height: Val::Px(380.0),
                    ..default()
                });
                spawn_option_panel(main_row);
            });

            // Done button
            spawn_done_button(root);
        });
}

fn spawn_tab_button(parent: &mut ChildSpawnerCommands, tab: ActiveTab, label: &str) {
    let is_active = tab == ActiveTab::Character; // default
    parent
        .spawn((
            TabButton(tab),
            Node {
                padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(if is_active {
                Palette::NeonPink.into()
            } else {
                Palette::ElectricPurple.into()
            }),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Palette::WhiteSmoke.into()),
            ));
        })
        .observe(on_tab_click)
        .observe(on_button_over)
        .observe(on_button_out);
}

fn spawn_category_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            CategoryPanel,
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                row_gap: Val::Px(8.0),
                width: Val::Px(160.0),
                ..default()
            },
        ))
        .with_children(|panel| {
            for cat in CHARACTER_CATEGORIES {
                spawn_category_button(panel, *cat);
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
                            min_width: Val::Px(120.0),
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
                root.spawn((
                    FaceSpriteLayer(layer),
                    Sprite {
                        image: asset_server.load(face_layer_asset_path(layer, &selections)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, z),
                ));
            }
        });
}

fn face_layer_asset_path(layer: FaceLayer, selections: &CharacterSelections) -> String {
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
// World-space horse sprites
// ---------------------------------------------------------------------------

fn spawn_horse_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selections: Res<HorseSelections>,
) {
    let layers = [
        (HorseLayer::Body, 0.0),
        (HorseLayer::Markings, 1.0),
        (HorseLayer::Mane, 2.0),
        (HorseLayer::BodyFront, 3.0),
        (HorseLayer::Tack, 4.0),
    ];

    commands
        .spawn((
            HorsePreviewRoot,
            Transform::from_scale(Vec3::splat(SPRITE_SCALE)),
            Visibility::Hidden,
        ))
        .with_children(|root| {
            for (layer, z) in layers {
                let show = match layer {
                    HorseLayer::Markings => has_markings(selections.coat_style),
                    HorseLayer::Tack => has_tack(selections.tack),
                    _ => true,
                };

                let mut sprite = Sprite::default();
                if show {
                    sprite.image = asset_server.load(horse_layer_asset_path(layer, &selections));
                }

                root.spawn((
                    HorseSpriteLayer(layer),
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

fn horse_layer_asset_path(layer: HorseLayer, selections: &HorseSelections) -> String {
    match layer {
        HorseLayer::Body => format!("horses/body/{}.png", selections.coat_colour.label()),
        HorseLayer::Markings => format!(
            "horses/markings/{}_{}.png",
            selections.coat_style.label(),
            selections.coat_colour.label()
        ),
        HorseLayer::Mane => format!("horses/mane/{}.png", selections.mane.label()),
        HorseLayer::BodyFront => {
            format!("horses/body_front/{}.png", selections.coat_colour.label())
        }
        HorseLayer::Tack => format!("horses/tack/{}.png", selections.tack.label()),
    }
}

// ---------------------------------------------------------------------------
// Interaction observers
// ---------------------------------------------------------------------------

fn on_tab_click(
    trigger: On<Pointer<Click>>,
    query: Query<&TabButton>,
    mut active_tab: ResMut<ActiveTab>,
    mut active_cat: ResMut<ActiveCategory>,
) {
    let entity = trigger.event_target();
    if let Ok(tab_btn) = query.get(entity)
        && *active_tab != tab_btn.0
    {
        *active_tab = tab_btn.0;
        // Reset to first category of the new tab
        *active_cat = match tab_btn.0 {
            ActiveTab::Character => ActiveCategory::Face,
            ActiveTab::Horse => ActiveCategory::CoatColour,
        };
    }
}

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
    mut horse_sel: ResMut<HorseSelections>,
) {
    match *active {
        ActiveCategory::Face => selections.face = selections.face.prev(),
        ActiveCategory::Eyes => selections.eyes = selections.eyes.prev(),
        ActiveCategory::Hair => selections.hair = selections.hair.prev(),
        ActiveCategory::Mouth => selections.mouth = selections.mouth.prev(),
        ActiveCategory::Skin => selections.skin = selections.skin.prev(),
        ActiveCategory::CoatColour => horse_sel.coat_colour = horse_sel.coat_colour.prev(),
        ActiveCategory::CoatStyle => horse_sel.coat_style = horse_sel.coat_style.prev(),
        ActiveCategory::Mane => horse_sel.mane = horse_sel.mane.prev(),
        ActiveCategory::Tack => horse_sel.tack = horse_sel.tack.prev(),
    }
}

fn on_next_click(
    _trigger: On<Pointer<Click>>,
    active: Res<ActiveCategory>,
    mut selections: ResMut<CharacterSelections>,
    mut horse_sel: ResMut<HorseSelections>,
) {
    match *active {
        ActiveCategory::Face => selections.face = selections.face.next(),
        ActiveCategory::Eyes => selections.eyes = selections.eyes.next(),
        ActiveCategory::Hair => selections.hair = selections.hair.next(),
        ActiveCategory::Mouth => selections.mouth = selections.mouth.next(),
        ActiveCategory::Skin => selections.skin = selections.skin.next(),
        ActiveCategory::CoatColour => horse_sel.coat_colour = horse_sel.coat_colour.next(),
        ActiveCategory::CoatStyle => horse_sel.coat_style = horse_sel.coat_style.next(),
        ActiveCategory::Mane => horse_sel.mane = horse_sel.mane.next(),
        ActiveCategory::Tack => horse_sel.tack = horse_sel.tack.next(),
    }
}

fn on_done_click(
    _trigger: On<Pointer<Click>>,
    selections: Res<CharacterSelections>,
    horse_sel: Res<HorseSelections>,
) {
    info!(
        "Character ready! Face: {}, Eyes: {}, Hair: {}, Mouth: {}, Skin: {} | \
         Horse: Coat: {} ({}), Mane: {}, Tack: {}",
        selections.face.display(),
        selections.eyes.display(),
        selections.hair.display(),
        selections.mouth.display(),
        selections.skin.display(),
        horse_sel.coat_colour.display(),
        horse_sel.coat_style.display(),
        horse_sel.mane.display(),
        horse_sel.tack.display(),
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
    tab_query: Query<&TabButton>,
    done_query: Query<&DoneButton>,
    active: Res<ActiveCategory>,
    active_tab: Res<ActiveTab>,
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
    } else if let Ok(tab_btn) = tab_query.get(entity) {
        let color = if tab_btn.0 == *active_tab {
            Palette::NeonPink
        } else {
            Palette::ElectricPurple
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
    horse_sel: Res<HorseSelections>,
    mut cat_label: Query<&mut Text, (With<CategoryLabel>, Without<OptionLabel>)>,
    mut opt_label: Query<&mut Text, (With<OptionLabel>, Without<CategoryLabel>)>,
) {
    if !active.is_changed() && !selections.is_changed() && !horse_sel.is_changed() {
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
        ActiveCategory::CoatColour => horse_sel.coat_colour.display_label(),
        ActiveCategory::CoatStyle => horse_sel.coat_style.display_label(),
        ActiveCategory::Mane => horse_sel.mane.display_label(),
        ActiveCategory::Tack => horse_sel.tack.display_label(),
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

fn update_tab_highlights(
    active_tab: Res<ActiveTab>,
    mut query: Query<(&TabButton, &mut BackgroundColor)>,
) {
    if !active_tab.is_changed() {
        return;
    }

    for (tab_btn, mut bg) in &mut query {
        let color = if tab_btn.0 == *active_tab {
            Palette::NeonPink
        } else {
            Palette::ElectricPurple
        };
        *bg = BackgroundColor(color.into());
    }
}

/// Rebuild the category button panel when the tab changes.
fn rebuild_categories_on_tab_change(
    active_tab: Res<ActiveTab>,
    mut commands: Commands,
    panel_query: Query<(Entity, &Children), With<CategoryPanel>>,
) {
    if !active_tab.is_changed() {
        return;
    }

    let categories = match *active_tab {
        ActiveTab::Character => CHARACTER_CATEGORIES,
        ActiveTab::Horse => HORSE_CATEGORIES,
    };

    for (panel_entity, children) in &panel_query {
        // Despawn existing category buttons
        for child in children.iter() {
            commands.entity(child).despawn();
        }
        commands.entity(panel_entity).with_children(|panel| {
            for cat in categories {
                spawn_category_button(panel, *cat);
            }
        });
    }
}

fn update_face_preview(
    selections: Res<CharacterSelections>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&FaceSpriteLayer, &mut Sprite)>,
) {
    if !selections.is_changed() {
        return;
    }

    for (layer, mut sprite) in &mut query {
        let path = face_layer_asset_path(layer.0, &selections);
        sprite.image = asset_server.load(&path);
    }
}

fn update_horse_preview(
    selections: Res<HorseSelections>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&HorseSpriteLayer, &mut Sprite, &mut Visibility)>,
) {
    if !selections.is_changed() {
        return;
    }

    for (layer, mut sprite, mut vis) in &mut query {
        let show = match layer.0 {
            HorseLayer::Markings => has_markings(selections.coat_style),
            HorseLayer::Tack => has_tack(selections.tack),
            _ => true,
        };

        if show {
            *vis = Visibility::Inherited;
            let path = horse_layer_asset_path(layer.0, &selections);
            sprite.image = asset_server.load(&path);
        } else {
            *vis = Visibility::Hidden;
        }
    }
}

/// Show/hide face and horse previews based on active tab.
fn update_preview_visibility(
    active_tab: Res<ActiveTab>,
    mut face_query: Query<&mut Visibility, (With<FacePreviewRoot>, Without<HorsePreviewRoot>)>,
    mut horse_query: Query<&mut Visibility, (With<HorsePreviewRoot>, Without<FacePreviewRoot>)>,
) {
    if !active_tab.is_changed() {
        return;
    }

    for mut vis in &mut face_query {
        *vis = if *active_tab == ActiveTab::Character {
            Visibility::default()
        } else {
            Visibility::Hidden
        };
    }

    for mut vis in &mut horse_query {
        *vis = if *active_tab == ActiveTab::Horse {
            Visibility::default()
        } else {
            Visibility::Hidden
        };
    }
}
