use bevy::{image::ImageSamplerDescriptor, prelude::*};
use kpop_horse::{assets, assets::Palette, game};

fn main() {
    App::new()
        .insert_resource(ClearColor(Palette::SoftLavender.into()))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "K-Pop Demon Hunters".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::linear(),
                }),
        )
        .add_plugins(assets::AssetPlugin)
        .add_plugins(game::GamePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
