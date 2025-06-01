//! Spawn the main level.

use std::collections::HashMap;

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;
use hexx::{Hex, HexLayout, shapes};

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

const SPRITE_SIZE: Vec2 = Vec2::new(28., 24.);

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

#[derive(Debug, Resource)]
struct HexGrid {
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    let layout = HexLayout::new(hexx::HexOrientation::Flat).with_rect_size(SPRITE_SIZE * 2.);
    let entities = shapes::flat_rectangle([-6, 6, -6, 6])
        .enumerate()
        .map(|(i, coord)| {
            let pos = layout.hex_to_world_pos(coord);
            let rotation = ((i % 3) as f32) * 60.0;
            let mut transformation = Transform::from_xyz(pos.x, pos.y, 0.);
            transformation.rotate_z(Rot2::degrees(rotation).as_radians());
            let entity = commands
                .spawn((
                    Sprite {
                        custom_size: Some(SPRITE_SIZE * 2.),
                        ..Default::default()
                    },
                    AseSlice {
                        name: "tracks".into(),
                        aseprite: level_assets.track.clone(),
                    },
                    transformation,
                ))
                .id();
            (coord, entity)
        })
        .collect();
    println!("lol?");
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
    ));
    commands.insert_resource(HexGrid { entities, layout });
}
