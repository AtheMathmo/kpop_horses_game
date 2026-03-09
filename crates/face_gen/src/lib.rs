//! Programmatic SVG face generator for the K-Pop Demon Hunter character creator.
//!
//! Generates layered SVG faces from component selections, then rasterizes to PNG.

mod eyes;
mod face;
mod hair;
mod horse_body;
mod horse_mane;
mod horse_tack;
mod horse_tail;
mod mouth;
mod render;

pub use render::{rasterize_svg, rasterize_svg_to_png};

/// Canvas dimensions matching the character preview area.
pub const CANVAS_W: f32 = 300.0;
pub const CANVAS_H: f32 = 380.0;

/// Center of the face on the canvas.
pub const FACE_CX: f32 = CANVAS_W / 2.0;
pub const FACE_CY: f32 = CANVAS_H / 2.0 + 10.0;

// ---------------------------------------------------------------------------
// Face component enums (mirrors character_creator types)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FaceShape {
    #[default]
    Oval,
    Round,
    Square,
    Heart,
    Long,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EyeStyle {
    #[default]
    Round,
    Almond,
    Cat,
    Wide,
    Narrow,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum HairStyle {
    #[default]
    Short,
    Long,
    Ponytail,
    Spiky,
    Bangs,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MouthStyle {
    #[default]
    Smile,
    Neutral,
    Pout,
    Open,
    Smirk,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SkinTone {
    #[default]
    Light,
    Medium,
    Tan,
    Dark,
    Pale,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FaceLayer {
    HairBack,
    Face,
    Eyes,
    Mouth,
    HairFront,
}

// ---------------------------------------------------------------------------
// Horse component enums
// ---------------------------------------------------------------------------

/// Canvas dimensions for horse sprites.
pub const HORSE_W: f32 = 500.0;
pub const HORSE_H: f32 = 400.0;

/// Approximate center of the horse body on the canvas.
pub const HORSE_CX: f32 = HORSE_W / 2.0;
pub const HORSE_CY: f32 = HORSE_H / 2.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CoatColour {
    #[default]
    Chestnut,
    Black,
    White,
    Dapple,
    Palomino,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CoatStyle {
    #[default]
    Plain,
    Socks,
    Blaze,
    Painted,
    Starry,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ManeStyle {
    #[default]
    Flowing,
    Braided,
    Flowers,
    Ribbons,
    Mohawk,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TackStyle {
    #[default]
    None,
    WesternSaddle,
    EnglishSaddle,
    Blanket,
    Bridle,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TailStyle {
    #[default]
    Plain,
    Flowers,
    Braided,
    Ribbons,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HorseLayer {
    /// Tail — rendered behind the body so the rump overlaps the tail base.
    Tail,
    Body,
    Markings,
    Mane,
    /// Near ear and face details — rendered in front of the mane.
    BodyFront,
    Tack,
}

impl CoatColour {
    pub fn hex(self) -> &'static str {
        match self {
            Self::Chestnut => "#8B4513",
            Self::Black => "#1A1A1A",
            Self::White => "#F5F0E8",
            Self::Dapple => "#A0A0A0",
            Self::Palomino => "#E8C860",
        }
    }

    pub fn shadow_hex(self) -> &'static str {
        match self {
            Self::Chestnut => "#5C2E0A",
            Self::Black => "#0A0A0A",
            Self::White => "#D8D0C0",
            Self::Dapple => "#707070",
            Self::Palomino => "#C0A040",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Chestnut => "chestnut",
            Self::Black => "black",
            Self::White => "white",
            Self::Dapple => "dapple",
            Self::Palomino => "palomino",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Chestnut => "Chestnut",
            Self::Black => "Black",
            Self::White => "White",
            Self::Dapple => "Dapple",
            Self::Palomino => "Palomino",
        }
    }
}

impl CoatStyle {
    pub fn label(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Socks => "socks",
            Self::Blaze => "blaze",
            Self::Painted => "painted",
            Self::Starry => "starry",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Plain => "Plain",
            Self::Socks => "Socks",
            Self::Blaze => "Blaze",
            Self::Painted => "Painted",
            Self::Starry => "Starry",
        }
    }
}

impl ManeStyle {
    pub fn label(self) -> &'static str {
        match self {
            Self::Flowing => "flowing",
            Self::Braided => "braided",
            Self::Flowers => "flowers",
            Self::Ribbons => "ribbons",
            Self::Mohawk => "mohawk",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Flowing => "Flowing",
            Self::Braided => "Braided",
            Self::Flowers => "Flowers",
            Self::Ribbons => "Ribbons",
            Self::Mohawk => "Mohawk",
        }
    }
}

impl TackStyle {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::WesternSaddle => "western",
            Self::EnglishSaddle => "english",
            Self::Blanket => "blanket",
            Self::Bridle => "bridle",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::WesternSaddle => "Western Saddle",
            Self::EnglishSaddle => "English Saddle",
            Self::Blanket => "Blanket",
            Self::Bridle => "Bridle",
        }
    }
}

impl TailStyle {
    pub fn label(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Flowers => "flowers",
            Self::Braided => "braided",
            Self::Ribbons => "ribbons",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Plain => "Plain",
            Self::Flowers => "Flowers",
            Self::Braided => "Braided",
            Self::Ribbons => "Ribbons",
        }
    }
}

pub const ALL_COAT_COLOURS: &[CoatColour] = &[
    CoatColour::Chestnut,
    CoatColour::Black,
    CoatColour::White,
    CoatColour::Dapple,
    CoatColour::Palomino,
];

pub const ALL_COAT_STYLES: &[CoatStyle] = &[
    CoatStyle::Plain,
    CoatStyle::Socks,
    CoatStyle::Blaze,
    CoatStyle::Painted,
    CoatStyle::Starry,
];

pub const ALL_MANE_STYLES: &[ManeStyle] = &[
    ManeStyle::Flowing,
    ManeStyle::Braided,
    ManeStyle::Flowers,
    ManeStyle::Ribbons,
    ManeStyle::Mohawk,
];

pub const ALL_TACK_STYLES: &[TackStyle] = &[
    TackStyle::None,
    TackStyle::WesternSaddle,
    TackStyle::EnglishSaddle,
    TackStyle::Blanket,
    TackStyle::Bridle,
];

pub const ALL_TAIL_STYLES: &[TailStyle] = &[
    TailStyle::Plain,
    TailStyle::Flowers,
    TailStyle::Braided,
    TailStyle::Ribbons,
];

impl SkinTone {
    pub fn hex(self) -> &'static str {
        match self {
            Self::Light => "#FFE0BD",
            Self::Medium => "#D2A06E",
            Self::Tan => "#B48250",
            Self::Dark => "#784032",
            Self::Pale => "#FFF0E6",
        }
    }

    /// A slightly darker shade for subtle shading.
    pub fn shadow_hex(self) -> &'static str {
        match self {
            Self::Light => "#E8C8A0",
            Self::Medium => "#B8884E",
            Self::Tan => "#966838",
            Self::Dark => "#5C2E22",
            Self::Pale => "#E8D8CC",
        }
    }
}

impl FaceShape {
    pub fn label(self) -> &'static str {
        match self {
            Self::Oval => "oval",
            Self::Round => "round",
            Self::Square => "square",
            Self::Heart => "heart",
            Self::Long => "long",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Oval => "Oval",
            Self::Round => "Round",
            Self::Square => "Square",
            Self::Heart => "Heart",
            Self::Long => "Long",
        }
    }
}

impl EyeStyle {
    pub fn label(self) -> &'static str {
        match self {
            Self::Round => "round",
            Self::Almond => "almond",
            Self::Cat => "cat",
            Self::Wide => "wide",
            Self::Narrow => "narrow",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Round => "Round",
            Self::Almond => "Almond",
            Self::Cat => "Cat",
            Self::Wide => "Wide",
            Self::Narrow => "Narrow",
        }
    }
}

impl HairStyle {
    pub fn label(self) -> &'static str {
        match self {
            Self::Short => "short",
            Self::Long => "long",
            Self::Ponytail => "ponytail",
            Self::Spiky => "spiky",
            Self::Bangs => "bangs",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Short => "Short",
            Self::Long => "Long",
            Self::Ponytail => "Ponytail",
            Self::Spiky => "Spiky",
            Self::Bangs => "Bangs",
        }
    }
}

impl MouthStyle {
    pub fn label(self) -> &'static str {
        match self {
            Self::Smile => "smile",
            Self::Neutral => "neutral",
            Self::Pout => "pout",
            Self::Open => "open",
            Self::Smirk => "smirk",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Smile => "Smile",
            Self::Neutral => "Neutral",
            Self::Pout => "Pout",
            Self::Open => "Open",
            Self::Smirk => "Smirk",
        }
    }
}

impl SkinTone {
    pub fn label(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Medium => "medium",
            Self::Tan => "tan",
            Self::Dark => "dark",
            Self::Pale => "pale",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Medium => "Medium",
            Self::Tan => "Tan",
            Self::Dark => "Dark",
            Self::Pale => "Pale",
        }
    }
}

// ---------------------------------------------------------------------------
// All variants (for iteration)
// ---------------------------------------------------------------------------

pub const ALL_FACE_SHAPES: &[FaceShape] = &[
    FaceShape::Oval,
    FaceShape::Round,
    FaceShape::Square,
    FaceShape::Heart,
    FaceShape::Long,
];

pub const ALL_EYE_STYLES: &[EyeStyle] = &[
    EyeStyle::Round,
    EyeStyle::Almond,
    EyeStyle::Cat,
    EyeStyle::Wide,
    EyeStyle::Narrow,
];

pub const ALL_HAIR_STYLES: &[HairStyle] = &[
    HairStyle::Short,
    HairStyle::Long,
    HairStyle::Ponytail,
    HairStyle::Spiky,
    HairStyle::Bangs,
];

pub const ALL_MOUTH_STYLES: &[MouthStyle] = &[
    MouthStyle::Smile,
    MouthStyle::Neutral,
    MouthStyle::Pout,
    MouthStyle::Open,
    MouthStyle::Smirk,
];

pub const ALL_SKIN_TONES: &[SkinTone] = &[
    SkinTone::Light,
    SkinTone::Medium,
    SkinTone::Tan,
    SkinTone::Dark,
    SkinTone::Pale,
];

// ---------------------------------------------------------------------------
// Full-face SVG composition
// ---------------------------------------------------------------------------

/// Character selections for a complete face.
#[derive(Clone, Copy, Debug, Default)]
pub struct FaceConfig {
    pub face: FaceShape,
    pub eyes: EyeStyle,
    pub hair: HairStyle,
    pub mouth: MouthStyle,
    pub skin: SkinTone,
}

/// Generate a complete SVG document string for a face.
pub fn generate_face_svg(config: &FaceConfig) -> String {
    let mut body = String::with_capacity(4096);

    // Defs: gradients and filters
    body.push_str(&svg_defs(config));

    // Layer order: hair-back → face → eyes → mouth → hair-front
    body.push_str(&hair::hair_back_svg(config.hair));
    body.push_str(&face::face_svg(config.face, config.skin));
    body.push_str(&eyes::eyes_svg(config.eyes, config.skin));
    body.push_str(&mouth::mouth_svg(config.mouth));
    body.push_str(&hair::hair_front_svg(config.hair));

    wrap_svg(&body)
}

/// Generate an SVG showing just one component (for individual inspection).
pub fn generate_component_svg(config: &FaceConfig, component: &str) -> String {
    let mut body = String::with_capacity(2048);
    body.push_str(&svg_defs(config));

    match component {
        "face" => body.push_str(&face::face_svg(config.face, config.skin)),
        "eyes" => {
            body.push_str(&face::face_svg(config.face, config.skin));
            body.push_str(&eyes::eyes_svg(config.eyes, config.skin));
        }
        "hair" => {
            body.push_str(&hair::hair_back_svg(config.hair));
            body.push_str(&face::face_svg(config.face, config.skin));
            body.push_str(&hair::hair_front_svg(config.hair));
        }
        "mouth" => {
            body.push_str(&face::face_svg(config.face, config.skin));
            body.push_str(&mouth::mouth_svg(config.mouth));
        }
        _ => {}
    }

    wrap_svg(&body)
}

/// Generate an SVG for a single isolated layer (for sprite export).
pub fn generate_layer_svg(config: &FaceConfig, layer: FaceLayer) -> String {
    let mut body = String::with_capacity(2048);
    body.push_str(&svg_defs(config));

    match layer {
        FaceLayer::HairBack => body.push_str(&hair::hair_back_svg(config.hair)),
        FaceLayer::Face => body.push_str(&face::face_svg(config.face, config.skin)),
        FaceLayer::Eyes => body.push_str(&eyes::eyes_svg(config.eyes, config.skin)),
        FaceLayer::Mouth => body.push_str(&mouth::mouth_svg(config.mouth)),
        FaceLayer::HairFront => body.push_str(&hair::hair_front_svg(config.hair)),
    }

    wrap_svg(&body)
}

/// Returns true if the given hair style has a front layer (bangs overlay).
pub fn has_front_layer(style: HairStyle) -> bool {
    style == HairStyle::Bangs
}

// ---------------------------------------------------------------------------
// Horse SVG composition
// ---------------------------------------------------------------------------

/// Character selections for a complete horse.
#[derive(Clone, Copy, Debug, Default)]
pub struct HorseConfig {
    pub coat_colour: CoatColour,
    pub coat_style: CoatStyle,
    pub mane: ManeStyle,
    pub tack: TackStyle,
    pub tail: TailStyle,
}

/// Generate a complete SVG document string for a horse.
pub fn generate_horse_svg(config: &HorseConfig) -> String {
    let mut body = String::with_capacity(4096);
    body.push_str(&horse_defs(config));

    // Layer order: tail → body → markings → mane → body-front (ear/face) → tack
    body.push_str(&horse_tail::tail_svg(config.tail));
    body.push_str(&horse_body::body_svg(config.coat_colour));
    body.push_str(&horse_body::markings_svg(
        config.coat_style,
        config.coat_colour,
    ));
    body.push_str(&horse_mane::mane_svg(config.mane));
    body.push_str(&horse_body::body_front_svg(config.coat_colour));
    body.push_str(&horse_tack::tack_svg(config.tack));

    wrap_horse_svg(&body)
}

/// Generate an SVG for a single horse layer (for sprite export).
pub fn generate_horse_layer_svg(config: &HorseConfig, layer: HorseLayer) -> String {
    let mut body = String::with_capacity(2048);
    body.push_str(&horse_defs(config));

    match layer {
        HorseLayer::Tail => body.push_str(&horse_tail::tail_svg(config.tail)),
        HorseLayer::Body => body.push_str(&horse_body::body_svg(config.coat_colour)),
        HorseLayer::Markings => {
            body.push_str(&horse_body::markings_svg(
                config.coat_style,
                config.coat_colour,
            ));
        }
        HorseLayer::Mane => body.push_str(&horse_mane::mane_svg(config.mane)),
        HorseLayer::BodyFront => body.push_str(&horse_body::body_front_svg(config.coat_colour)),
        HorseLayer::Tack => body.push_str(&horse_tack::tack_svg(config.tack)),
    }

    wrap_horse_svg(&body)
}

/// Returns true if the horse has visible markings for the given coat style.
pub fn has_markings(style: CoatStyle) -> bool {
    style != CoatStyle::Plain
}

/// Returns true if the horse has visible tack.
pub fn has_tack(style: TackStyle) -> bool {
    style != TackStyle::None
}

fn wrap_svg(body: &str) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {CANVAS_W} {CANVAS_H}" width="{CANVAS_W}" height="{CANVAS_H}">
{body}
</svg>"#
    )
}

fn wrap_horse_svg(body: &str) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {HORSE_W} {HORSE_H}" width="{HORSE_W}" height="{HORSE_H}">
{body}
</svg>"#
    )
}

fn svg_defs(config: &FaceConfig) -> String {
    let skin = config.skin.hex();
    let shadow = config.skin.shadow_hex();

    format!(
        r##"<defs>
  <radialGradient id="face-grad" cx="40%" cy="35%" r="60%">
    <stop offset="0%" stop-color="{skin}"/>
    <stop offset="100%" stop-color="{shadow}"/>
  </radialGradient>
  <radialGradient id="cheek-grad" cx="50%" cy="50%" r="50%">
    <stop offset="0%" stop-color="#FF8888" stop-opacity="0.35"/>
    <stop offset="100%" stop-color="#FF8888" stop-opacity="0"/>
  </radialGradient>
  <filter id="soft-shadow" x="-10%" y="-10%" width="130%" height="130%">
    <feDropShadow dx="0" dy="3" stdDeviation="4" flood-color="#000" flood-opacity="0.2"/>
  </filter>
</defs>
"##
    )
}

fn horse_defs(config: &HorseConfig) -> String {
    let coat = config.coat_colour.hex();
    let shadow = config.coat_colour.shadow_hex();

    format!(
        r##"<defs>
  <radialGradient id="coat-grad" gradientUnits="userSpaceOnUse" cx="200" cy="140" r="260">
    <stop offset="0%" stop-color="{coat}"/>
    <stop offset="100%" stop-color="{shadow}"/>
  </radialGradient>
  <filter id="soft-shadow" x="-10%" y="-10%" width="130%" height="130%">
    <feDropShadow dx="0" dy="3" stdDeviation="4" flood-color="#000" flood-opacity="0.2"/>
  </filter>
</defs>
"##
    )
}
