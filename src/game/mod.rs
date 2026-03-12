mod character_creator;
mod demon_hunt;
pub mod effects;
mod game_select;
mod horse_brushing;
mod treat_toss;

use bevy::prelude::*;

use self::character_creator::CharacterCreatorPlugin;
use self::demon_hunt::DemonHuntPlugin;
use self::effects::EffectsPlugin;
use self::game_select::GameSelectPlugin;
use self::horse_brushing::HorseBrushingPlugin;
use self::treat_toss::TreatTossPlugin;
use face_gen::{
    BridleStyle, CoatColour, CoatStyle, HorseLayer, ManeStyle, SaddleStyle, TailStyle, has_bridle,
    has_markings, has_saddle,
};

// ---------------------------------------------------------------------------
// Game state
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    CharacterCreator,
    GameSelect,
    HorseBrushing,
    TreatToss,
    DemonHunt,
}

// ---------------------------------------------------------------------------
// Shared horse data
// ---------------------------------------------------------------------------

/// Current horse customisation selections — shared between character creator
/// and gameplay screens.
#[derive(Resource, Debug, Clone, Default)]
pub struct HorseSelections {
    pub coat_colour: CoatColour,
    pub coat_style: CoatStyle,
    pub mane: ManeStyle,
    pub saddle: SaddleStyle,
    pub bridle: BridleStyle,
    pub tail: TailStyle,
}

/// Returns the asset path for a horse sprite layer given current selections.
pub fn horse_layer_asset_path(layer: HorseLayer, selections: &HorseSelections) -> String {
    match layer {
        HorseLayer::Tail => format!("horses/tail/{}.png", selections.tail.label()),
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
        HorseLayer::Saddle => format!("horses/saddle/{}.png", selections.saddle.label()),
        HorseLayer::Bridle => format!("horses/bridle/{}.png", selections.bridle.label()),
    }
}

/// Returns whether a horse layer should be visible given current selections.
pub fn horse_layer_visible(layer: HorseLayer, selections: &HorseSelections) -> bool {
    match layer {
        HorseLayer::Markings => has_markings(selections.coat_style),
        HorseLayer::Saddle => has_saddle(selections.saddle),
        HorseLayer::Bridle => has_bridle(selections.bridle),
        _ => true,
    }
}

// ---------------------------------------------------------------------------
// Top-level game plugin
// ---------------------------------------------------------------------------

/// Top-level game plugin — chains all feature plugins.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<HorseSelections>()
            .add_plugins((
                CharacterCreatorPlugin,
                DemonHuntPlugin,
                EffectsPlugin,
                GameSelectPlugin,
                HorseBrushingPlugin,
                TreatTossPlugin,
            ));
    }
}
