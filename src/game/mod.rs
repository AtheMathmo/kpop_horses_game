mod character_creator;

use bevy::prelude::*;

use self::character_creator::CharacterCreatorPlugin;

/// Top-level game plugin — chains all feature plugins.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterCreatorPlugin);
    }
}
