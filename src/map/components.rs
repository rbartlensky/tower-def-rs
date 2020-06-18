use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::{assets::Handle, renderer::SpriteSheet};

use crate::Coord;

pub struct Map {
    tiled_map: tiled::Map,
    road: Vec<Vec<Coord>>,
    sprite_sheet_handle: Handle<SpriteSheet>,
}

impl Map {
    pub fn new(
        tiled_map: tiled::Map,
        road: Vec<Vec<Coord>>,
        sprite_sheet_handle: Handle<SpriteSheet>,
    ) -> Self {
        Self {
            tiled_map,
            road,
            sprite_sheet_handle,
        }
    }

    pub fn tile_width(&self) -> u32 {
        self.tiled_map.tile_width
    }

    pub fn tile_height(&self) -> u32 {
        self.tiled_map.tile_height
    }

    pub fn road(&self) -> &Vec<Vec<Coord>> {
        &self.road
    }

    pub fn sprite_sheet_handle(&self) -> Handle<SpriteSheet> {
        self.sprite_sheet_handle.clone()
    }
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}
