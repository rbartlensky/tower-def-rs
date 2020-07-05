use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::{assets::Handle, renderer::SpriteSheet};

use crate::Coord;

pub struct Map {
    tiled_map: tiled::Map,
    road: Vec<Vec<Coord>>,
    sprite_sheet_handle: Handle<SpriteSheet>,
    gold: usize,
    gold_text: Entity,
    error_text: Entity,
}

impl Map {
    pub fn new(
        tiled_map: tiled::Map,
        road: Vec<Vec<Coord>>,
        sprite_sheet_handle: Handle<SpriteSheet>,
        gold_text: Entity,
        error_text: Entity,
    ) -> Self {
        Self {
            tiled_map,
            road,
            sprite_sheet_handle,
            gold: 100,
            gold_text,
            error_text,
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

    pub fn gold(&self) -> usize {
        self.gold
    }

    pub fn add_gold(&mut self, gold: usize) {
        self.gold += gold;
    }

    pub fn remove_gold(&mut self, gold: usize) {
        self.gold -= gold;
    }

    pub fn gold_text(&self) -> Entity {
        self.gold_text.clone()
    }

    pub fn error_text(&self) -> Entity {
        self.error_text.clone()
    }
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}
