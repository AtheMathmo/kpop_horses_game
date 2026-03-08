//! Asset resources, events, and systems.

use bevy::prelude::*;

pub mod audio;
mod mipmaps;
mod palette;
pub use palette::{ColourPalette, Palette, create_palette_image};

use self::audio::GameAudioPlugin;
use self::mipmaps::MipmapPlugin;

/// Plugin that takes care of asset loading and related resources/systems.
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameAudioPlugin, MipmapPlugin))
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let palette = ColourPalette::new(&mut images);
    commands.insert_resource(palette);
}
