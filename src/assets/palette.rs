use bevy::color::ColorToPacked;
use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::*};

/// Palette for the game's colour scheme — K-Pop Demon Hunter theme.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use kpop_horse::assets::Palette;
///
/// let col: Palette = Palette::NeonPink;
/// let col_val: Color = col.into();
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Palette {
    NeonPink,
    ElectricPurple,
    DeepViolet,
    HotMagenta,
    MidnightBlue,
    SoftLavender,
    WhiteSmoke,
    CarbonBlack,
    SkinToneLight,
    SkinToneMedium,
}

impl From<Palette> for Color {
    fn from(value: Palette) -> Self {
        value.color()
    }
}

impl Palette {
    /// Returns the `Color` value for this palette entry.
    ///
    /// Unlike `From<Palette> for Color`, this is a `const fn` so it can be
    /// used in `const` and `static` initialisers.
    pub const fn color(&self) -> Color {
        match self {
            Palette::NeonPink => Color::srgb_u8(255, 0, 127),
            Palette::ElectricPurple => Color::srgb_u8(153, 50, 204),
            Palette::HotMagenta => Color::srgb_u8(255, 29, 206),
            Palette::DeepViolet => Color::srgb_u8(75, 0, 130),
            Palette::MidnightBlue => Color::srgb_u8(18, 10, 40),
            Palette::SoftLavender => Color::srgb_u8(200, 180, 230),
            Palette::WhiteSmoke => Color::srgb_u8(245, 245, 245),
            Palette::CarbonBlack => Color::srgb_u8(31, 31, 31),
            Palette::SkinToneLight => Color::srgb_u8(255, 224, 189),
            Palette::SkinToneMedium => Color::srgb_u8(210, 160, 110),
        }
    }

    /// Returns a CarbonBlack colour with the given alpha, useful for overlays.
    pub const fn overlay(alpha: f32) -> Color {
        Color::srgba(31.0 / 255.0, 31.0 / 255.0, 31.0 / 255.0, alpha)
    }

    /// Convert the palette color into UV coordinates for the
    /// `ColourPalette` image texture.
    pub fn as_uvs(&self) -> [f32; 2] {
        match self {
            Palette::NeonPink => [1.0 / 8.0, 1.0 / 6.0],
            Palette::ElectricPurple => [3.0 / 8.0, 1.0 / 6.0],
            Palette::HotMagenta => [5.0 / 8.0, 1.0 / 6.0],
            Palette::DeepViolet => [7.0 / 8.0, 1.0 / 6.0],
            Palette::MidnightBlue => [1.0 / 8.0, 3.0 / 6.0],
            Palette::SoftLavender => [3.0 / 8.0, 3.0 / 6.0],
            Palette::WhiteSmoke => [5.0 / 8.0, 3.0 / 6.0],
            Palette::CarbonBlack => [7.0 / 8.0, 3.0 / 6.0],
            Palette::SkinToneLight => [1.0 / 8.0, 5.0 / 6.0],
            Palette::SkinToneMedium => [3.0 / 8.0, 5.0 / 6.0],
        }
    }
}

/// The `ColourPalette` resource.
///
/// Contains a `Handle<Image>` pointing to the palette image.
#[derive(Resource)]
pub struct ColourPalette {
    image: Handle<Image>,
}

impl ColourPalette {
    /// Instantiate a new `ColourPalette`, with image handle.
    pub fn new(images: &mut Assets<Image>) -> Self {
        let handle = images.add(create_palette_image());
        Self { image: handle }
    }

    /// Get a clone of the image handle.
    pub fn image(&self) -> Handle<Image> {
        self.image.clone()
    }
}

/// Create a new `Image` representing the colour palette.
///
/// The image is a 4x3 grid (12 slots). The first 10 are the palette colours;
/// the remaining 2 are padded with CarbonBlack.
pub fn create_palette_image() -> Image {
    let pad = Palette::CarbonBlack;
    let data = vec![
        Palette::NeonPink,
        Palette::ElectricPurple,
        Palette::HotMagenta,
        Palette::DeepViolet,
        Palette::MidnightBlue,
        Palette::SoftLavender,
        Palette::WhiteSmoke,
        Palette::CarbonBlack,
        Palette::SkinToneLight,
        Palette::SkinToneMedium,
        pad,
        pad,
    ]
    .into_iter()
    .flat_map(|p| Color::from(p).to_srgba().to_u8_array())
    .collect();
    Image::new(
        Extent3d {
            width: 4,
            height: 3,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}
