//! Spawn the main level.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;

use crate::{asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    track: Handle<Aseprite>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        let track: Handle<Aseprite> = assets.load("images/tracks.aseprite");
        println!("what, {:?}", track);
        Self {
            track: assets.load_with_settings(
                "images/tracks.aseprite",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    println!("lol?");
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            (
                AseSlice {
                    name: "tracks".into(),
                    aseprite: level_assets.track.clone(),
                },
                Sprite::default(),
                Transform::from_translation(Vec3::new(32., 0., 0.)),
            ),
            (
                AseSlice {
                    name: "tracks".into(),
                    aseprite: level_assets.track.clone(),
                },
                Sprite {
                    custom_size: Some(Vec2::new(128., 128.)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(32., 32., 0.)),
            )
        ],
    ));
}
