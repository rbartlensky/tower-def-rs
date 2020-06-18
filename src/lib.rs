use amethyst::{core::transform::Transform, prelude::*, renderer::Camera};

mod map;
pub mod runner;
pub mod states;
pub mod tower;
pub use states::TowerDefState;

pub const MAP_HEIGHT: usize = 700;
pub const MAP_WIDTH: usize = 700;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();

    transform.set_translation_xyz((MAP_WIDTH / 2) as f32, (MAP_HEIGHT / 2) as f32, 100.0);
    world
        .create_entity()
        .with(Camera::standard_2d(MAP_WIDTH as f32, MAP_HEIGHT as f32))
        .with(transform)
        .build();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn to_trans(&self, tile_w: usize, tile_h: usize) -> Transform {
        let mut pos = Transform::default();
        let offset_x = (tile_w / 2) as f32;
        let offset_y = (tile_h / 2) as f32;
        pos.set_translation_xyz(
            (self.x * tile_w) as f32 + offset_x,
            (self.y * tile_h) as f32 + offset_y,
            1.1,
        );
        pos
    }

    pub fn distance(&self, other: &Coord) -> f32 {
        (((self.x as isize - other.x as isize).pow(2) + (self.y as isize - other.y as isize).pow(2))
            as f32)
            .sqrt()
    }
}
