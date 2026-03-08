//! Audio systems and resources (stub — no audio assets yet).

use bevy::prelude::*;

/// Plugin for setting up the game's audio systems and resources.
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, _app: &mut App) {
        // Audio will be wired up once sound files are added to assets/audio/
    }
}
