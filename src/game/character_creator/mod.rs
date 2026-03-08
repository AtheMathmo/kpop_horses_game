//! Character creation screen — lets the player design their K-Pop Demon Hunter.

use bevy::prelude::*;

use crate::assets::Palette;

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct CharacterCreatorPlugin;

impl Plugin for CharacterCreatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterSelections>()
            .init_resource::<ActiveCategory>()
            .add_systems(Startup, spawn_ui)
            .add_systems(
                Update,
                (update_labels, update_category_highlights, update_preview),
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

    fn label(&self) -> &'static str;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FaceShape {
    #[default]
    Oval,
    Round,
    Square,
    Heart,
    Long,
}

impl Cyclable for FaceShape {
    fn variants() -> &'static [Self] {
        &[
            Self::Oval,
            Self::Round,
            Self::Square,
            Self::Heart,
            Self::Long,
        ]
    }
    fn index(&self) -> usize {
        *self as usize
    }
    fn label(&self) -> &'static str {
        match self {
            Self::Oval => "Oval",
            Self::Round => "Round",
            Self::Square => "Square",
            Self::Heart => "Heart",
            Self::Long => "Long",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EyeStyle {
    #[default]
    Round,
    Almond,
    Cat,
    Wide,
    Narrow,
}

impl Cyclable for EyeStyle {
    fn variants() -> &'static [Self] {
        &[
            Self::Round,
            Self::Almond,
            Self::Cat,
            Self::Wide,
            Self::Narrow,
        ]
    }
    fn index(&self) -> usize {
        *self as usize
    }
    fn label(&self) -> &'static str {
        match self {
            Self::Round => "Round",
            Self::Almond => "Almond",
            Self::Cat => "Cat",
            Self::Wide => "Wide",
            Self::Narrow => "Narrow",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HairStyle {
    #[default]
    Short,
    Long,
    Ponytail,
    Spiky,
    Bangs,
}

impl Cyclable for HairStyle {
    fn variants() -> &'static [Self] {
        &[
            Self::Short,
            Self::Long,
            Self::Ponytail,
            Self::Spiky,
            Self::Bangs,
        ]
    }
    fn index(&self) -> usize {
        *self as usize
    }
    fn label(&self) -> &'static str {
        match self {
            Self::Short => "Short",
            Self::Long => "Long",
            Self::Ponytail => "Ponytail",
            Self::Spiky => "Spiky",
            Self::Bangs => "Bangs",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MouthStyle {
    #[default]
    Smile,
    Neutral,
    Pout,
    Open,
    Smirk,
}

impl Cyclable for MouthStyle {
    fn variants() -> &'static [Self] {
        &[
            Self::Smile,
            Self::Neutral,
            Self::Pout,
            Self::Open,
            Self::Smirk,
        ]
    }
    fn index(&self) -> usize {
        *self as usize
    }
    fn label(&self) -> &'static str {
        match self {
            Self::Smile => "Smile",
            Self::Neutral => "Neutral",
            Self::Pout => "Pout",
            Self::Open => "Open",
            Self::Smirk => "Smirk",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SkinTone {
    #[default]
    Light,
    Medium,
    Tan,
    Dark,
    Pale,
}

impl Cyclable for SkinTone {
    fn variants() -> &'static [Self] {
        &[Self::Light, Self::Medium, Self::Tan, Self::Dark, Self::Pale]
    }
    fn index(&self) -> usize {
        *self as usize
    }
    fn label(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Medium => "Medium",
            Self::Tan => "Tan",
            Self::Dark => "Dark",
            Self::Pale => "Pale",
        }
    }
}

impl SkinTone {
    fn color(&self) -> Color {
        match self {
            Self::Light => Palette::SkinToneLight.color(),
            Self::Medium => Palette::SkinToneMedium.color(),
            Self::Tan => Color::srgb_u8(180, 130, 80),
            Self::Dark => Color::srgb_u8(120, 80, 50),
            Self::Pale => Color::srgb_u8(255, 240, 230),
        }
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
struct PreviewArea;

#[derive(Component)]
struct CategoryLabel;

#[derive(Component)]
struct OptionLabel;

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
                spawn_preview_area(main_row);
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

fn spawn_preview_area(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        PreviewArea,
        Node {
            width: Val::Px(300.0),
            height: Val::Px(380.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(16.0)),
            ..default()
        },
        BackgroundColor(Palette::CarbonBlack.into()),
    ));
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
        selections.face.label(),
        selections.eyes.label(),
        selections.hair.label(),
        selections.mouth.label(),
        selections.skin.label(),
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
        ActiveCategory::Face => selections.face.label(),
        ActiveCategory::Eyes => selections.eyes.label(),
        ActiveCategory::Hair => selections.hair.label(),
        ActiveCategory::Mouth => selections.mouth.label(),
        ActiveCategory::Skin => selections.skin.label(),
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
    mut commands: Commands,
    selections: Res<CharacterSelections>,
    preview_query: Query<Entity, With<PreviewArea>>,
    children_query: Query<&Children>,
) {
    if !selections.is_changed() {
        return;
    }

    let Ok(preview_entity) = preview_query.single() else {
        return;
    };

    // Despawn existing children
    if let Ok(children) = children_query.get(preview_entity) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    // Rebuild the face preview
    commands.entity(preview_entity).with_children(|parent| {
        spawn_face_preview(parent, &selections);
    });
}

// ---------------------------------------------------------------------------
// Face preview rendering
// ---------------------------------------------------------------------------

fn spawn_face_preview(parent: &mut ChildSpawnerCommands, selections: &CharacterSelections) {
    let skin_color = selections.skin.color();

    let (face_w, face_h, face_radius) = match selections.face {
        FaceShape::Oval => (180.0, 240.0, Val::Percent(45.0)),
        FaceShape::Round => (210.0, 210.0, Val::Percent(50.0)),
        FaceShape::Square => (200.0, 220.0, Val::Percent(12.0)),
        FaceShape::Heart => (190.0, 230.0, Val::Percent(40.0)),
        FaceShape::Long => (160.0, 260.0, Val::Percent(45.0)),
    };

    parent
        .spawn((
            Node {
                width: Val::Px(face_w),
                height: Val::Px(face_h),
                position_type: PositionType::Relative,
                border_radius: BorderRadius::all(face_radius),
                ..default()
            },
            BackgroundColor(skin_color),
        ))
        .with_children(|face| {
            spawn_hair(face, selections.hair, face_w);
            spawn_eyes(face, selections.eyes);
            spawn_mouth(face, selections.mouth);
        });
}

fn spawn_hair(parent: &mut ChildSpawnerCommands, style: HairStyle, face_w: f32) {
    let hair_color: Color = Palette::DeepViolet.into();

    match style {
        HairStyle::Short => {
            parent.spawn((
                Node {
                    width: Val::Px(face_w + 20.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(-15.0),
                    left: Val::Px(-10.0),
                    border_radius: BorderRadius::top(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(hair_color),
            ));
        }
        HairStyle::Long => {
            parent.spawn((
                Node {
                    width: Val::Px(face_w + 30.0),
                    height: Val::Px(60.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(-20.0),
                    left: Val::Px(-15.0),
                    border_radius: BorderRadius::top(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(hair_color),
            ));
            parent.spawn((
                Node {
                    width: Val::Px(25.0),
                    height: Val::Px(200.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(30.0),
                    left: Val::Px(-15.0),
                    border_radius: BorderRadius::left(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(hair_color),
            ));
            parent.spawn((
                Node {
                    width: Val::Px(25.0),
                    height: Val::Px(200.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(30.0),
                    right: Val::Px(-15.0),
                    border_radius: BorderRadius::right(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(hair_color),
            ));
        }
        HairStyle::Ponytail => {
            parent.spawn((
                Node {
                    width: Val::Px(face_w + 10.0),
                    height: Val::Px(45.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(-12.0),
                    left: Val::Px(-5.0),
                    border_radius: BorderRadius::top(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(hair_color),
            ));
            parent.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(120.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(-10.0),
                    right: Val::Px(-25.0),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(hair_color),
            ));
        }
        HairStyle::Spiky => {
            for i in 0..5 {
                let offset = i as f32 * (face_w / 5.0) + 5.0;
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(45.0),
                        position_type: PositionType::Absolute,
                        top: Val::Px(-35.0),
                        left: Val::Px(offset),
                        border_radius: BorderRadius::top(Val::Percent(50.0)),
                        ..default()
                    },
                    BackgroundColor(hair_color),
                ));
            }
        }
        HairStyle::Bangs => {
            parent.spawn((
                Node {
                    width: Val::Px(face_w + 20.0),
                    height: Val::Px(55.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(-18.0),
                    left: Val::Px(-10.0),
                    border_radius: BorderRadius::top(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(hair_color),
            ));
            parent.spawn((
                Node {
                    width: Val::Px(face_w * 0.7),
                    height: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(15.0),
                    left: Val::Px(face_w * 0.05),
                    border_radius: BorderRadius::bottom(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(hair_color),
            ));
        }
    }
}

fn spawn_eyes(parent: &mut ChildSpawnerCommands, style: EyeStyle) {
    let eye_color: Color = Palette::CarbonBlack.into();
    let pupil_color: Color = Palette::WhiteSmoke.into();

    let (eye_w, eye_h, eye_radius, pupil_w, pupil_h) = match style {
        EyeStyle::Round => (28.0, 28.0, Val::Percent(50.0), 12.0, 12.0),
        EyeStyle::Almond => (32.0, 18.0, Val::Percent(50.0), 12.0, 12.0),
        EyeStyle::Cat => (30.0, 16.0, Val::Percent(40.0), 10.0, 14.0),
        EyeStyle::Wide => (36.0, 30.0, Val::Percent(50.0), 14.0, 14.0),
        EyeStyle::Narrow => (28.0, 10.0, Val::Percent(50.0), 8.0, 8.0),
    };

    spawn_single_eye(
        parent,
        eye_w,
        eye_h,
        eye_radius,
        pupil_w,
        pupil_h,
        eye_color,
        pupil_color,
        Val::Percent(22.0),
    );
    spawn_single_eye(
        parent,
        eye_w,
        eye_h,
        eye_radius,
        pupil_w,
        pupil_h,
        eye_color,
        pupil_color,
        Val::Percent(55.0),
    );
}

fn spawn_single_eye(
    parent: &mut ChildSpawnerCommands,
    w: f32,
    h: f32,
    radius: Val,
    pupil_w: f32,
    pupil_h: f32,
    eye_color: Color,
    pupil_color: Color,
    left: Val,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(w),
                height: Val::Px(h),
                position_type: PositionType::Absolute,
                top: Val::Percent(38.0),
                left,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(radius),
                ..default()
            },
            BackgroundColor(eye_color),
        ))
        .with_children(|eye| {
            eye.spawn((
                Node {
                    width: Val::Px(pupil_w),
                    height: Val::Px(pupil_h),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(pupil_color),
            ));
        });
}

fn spawn_mouth(parent: &mut ChildSpawnerCommands, style: MouthStyle) {
    let mouth_color: Color = Palette::NeonPink.into();

    let (w, h, radius) = match style {
        MouthStyle::Smile => (50.0, 16.0, BorderRadius::bottom(Val::Percent(50.0))),
        MouthStyle::Neutral => (40.0, 6.0, BorderRadius::all(Val::Px(3.0))),
        MouthStyle::Pout => (22.0, 22.0, BorderRadius::all(Val::Percent(50.0))),
        MouthStyle::Open => (30.0, 28.0, BorderRadius::all(Val::Percent(50.0))),
        MouthStyle::Smirk => (36.0, 12.0, BorderRadius::bottom_right(Val::Percent(50.0))),
    };

    parent.spawn((
        Node {
            width: Val::Px(w),
            height: Val::Px(h),
            position_type: PositionType::Absolute,
            top: Val::Percent(72.0),
            left: Val::Percent(50.0 - (w / 4.0)),
            border_radius: radius,
            ..default()
        },
        BackgroundColor(mouth_color),
    ));
}
