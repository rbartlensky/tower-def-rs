use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteRender},
};

use std::collections::HashMap;

use super::Coord;
use crate::{
    map::{self, Map},
    tower::BuildPoint,
};

#[derive(Default)]
pub struct TowerDefState {}

impl TowerDefState {
    fn initialise_camera(&mut self, world: &mut World, map: &tiled::Map) {
        let mut transform = Transform::default();
        let width = (map.width * map.tile_width) as f32;
        let height = (map.height * map.tile_height) as f32;

        transform.set_translation_xyz(width / 2., height / 2., 1.0);
        world
            .create_entity()
            .with(Camera::standard_2d(width, height))
            .with(transform)
            .build();
    }

    fn load_map(&mut self, world: &mut World) {
        // parse the map
        let map_file = std::fs::File::open("assets/tower-def.tmx").unwrap();
        let reader = std::io::BufReader::new(map_file);
        let map = tiled::parse(reader).unwrap();
        let tile_set = map.get_tileset_by_gid(1).unwrap();
        self.initialise_camera(world, &map);

        let sprite_sheets = map::create_sprite_sheets(tile_set, world);
        let sprite_sheet_handle = &sprite_sheets[0];

        // which tile ids are our roads
        let mut roads = vec![];
        // which tile ids are our construction points
        let mut construction_points = vec![];
        // which tile id is the starting point
        let mut start_point = None;
        let mut end_point = None;
        let mut directions = HashMap::new();
        for tile in map.tilesets[0].tiles.iter() {
            if tile.properties.contains_key("road") {
                roads.push(tile.id);
                let mut dir: u8 = 0b0000;
                if let Some(tiled::PropertyValue::BoolValue(true)) = tile.properties.get("up") {
                    dir |= 0b0001;
                }
                if let Some(tiled::PropertyValue::BoolValue(true)) = tile.properties.get("right") {
                    dir |= 0b0010;
                }
                if let Some(tiled::PropertyValue::BoolValue(true)) = tile.properties.get("down") {
                    dir |= 0b0100;
                }
                if let Some(tiled::PropertyValue::BoolValue(true)) = tile.properties.get("left") {
                    dir |= 0b1000;
                }
                directions.insert(tile.id, dir);
            } else if tile.properties.contains_key("construction-point") {
                construction_points.push(tile.id);
            } else if tile.properties.contains_key("start-point") {
                start_point = Some(tile.id);
            } else if tile.properties.contains_key("end-point") {
                end_point = Some(tile.id);
            }
        }
        assert!(start_point.is_some(), "No tile defined as starting point!");
        assert!(end_point.is_some(), "No tile defined as end point!");
        let start_point = start_point.unwrap();
        let end_point = end_point.unwrap();
        let (tile_width, tile_height) = (tile_set.tile_width, tile_set.tile_height);
        // each entry represents whether [x][y] can be walked on
        let mut road_map: Vec<Vec<u8>> = vec![vec![0; map.width as usize]; map.height as usize];
        let mut start_coord = None;
        let mut end_coord = None;
        for layer in map.layers.iter().rev() {
            for (y, row) in layer.tiles.iter().rev().enumerate().clone() {
                for (x, &tile) in row.iter().enumerate() {
                    // Do nothing with empty tiles
                    if tile.gid == 0 {
                        continue;
                    }

                    // Tile ids start from 1 but tileset sprites start from 0
                    let tile_id = tile.gid - 1;
                    if roads.binary_search(&tile_id).is_ok() {
                        road_map[x][y] = directions[&tile_id];
                    } else if tile_id == start_point {
                        start_coord = Some(Coord::new(x, y));
                        road_map[x][y] = 0b1111;
                    } else if tile_id == end_point {
                        end_coord = Some(Coord::new(x, y));
                        road_map[x][y] = 0b1111;
                    }

                    // Sprite for the tile
                    let tile_sprite = SpriteRender {
                        sprite_sheet: sprite_sheet_handle.clone(),
                        sprite_number: tile_id as usize,
                    };

                    // Where should we draw the tile?
                    let mut tile_transform = Transform::default();
                    let x_coord = x * tile_width as usize;
                    let y_coord = (y as f32 * tile_height as f32) + tile_height as f32;
                    // Offset the positions by half the tile size so they're nice and snuggly on the screen
                    // Alternatively could use the Sprite offsets instead: [-32.0, 32.0]. Depends on the use case I guess.
                    let offset_x = tile_width as f32 / 2.0;
                    let offset_y = -(tile_height as f32) / 2.0;

                    tile_transform.set_translation_xyz(
                        offset_x + x_coord as f32,
                        offset_y + y_coord as f32,
                        -1.0,
                    );

                    // Create the tile entity
                    let entity = world
                        .create_entity()
                        .with(tile_transform)
                        .with(tile_sprite.clone());
                    // if it is a build point, make sure to add that component as well
                    if construction_points.binary_search(&tile_id).is_ok() {
                        entity.with(BuildPoint::new(Coord::new(x, y)))
                    } else {
                        entity
                    }
                    .build();
                }
            }
        }
        let paths = gather_paths(vec![start_coord.unwrap()], end_coord.unwrap(), &road_map);
        world
            .create_entity()
            .with(Map::new(map, paths, sprite_sheet_handle.clone()))
            .build();
    }
}

impl SimpleState for TowerDefState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.load_map(world);
    }
}

/// Gets all valid paths that a runner can take.
fn gather_paths(path: Vec<Coord>, dest: Coord, map: &Vec<Vec<u8>>) -> Vec<Vec<Coord>> {
    static DIRECTIONS: [(isize, isize, u8); 4] = [
        (0, 1, 0b0001),
        (0, -1, 0b0100),
        (1, 0, 0b0010),
        (-1, 0, 0b1000),
    ];
    // where we are coming from
    let origin = *path.iter().last().unwrap();
    // did we just finish a path? if so return it
    if origin == dest {
        return vec![path];
    }
    let mut neighbours = Vec::with_capacity(4);
    // up, down, right, left
    for offset in &DIRECTIONS {
        let new_c = (origin.x as isize + offset.0, origin.y as isize + offset.1);
        // if we can go in that direction
        if new_c.0 >= 0
            && new_c.1 >= 0
            && (new_c.0 as usize) < map.len()
            && (new_c.1 as usize) < map[0].len()
            // if we can actually move in that direction!
            && (map[origin.x][origin.y] & offset.2) > 0
        {
            let new_coord = Coord::new(new_c.0 as usize, new_c.1 as usize);
            // make sure we don't create a cycle!
            if !path.contains(&new_coord) {
                neighbours.push(Coord::new(new_c.0 as usize, new_c.1 as usize));
            }
        }
    }
    let mut final_paths = vec![];
    for n in neighbours {
        // a new path from what we have so far + a neighbour that we can visit
        let new_path = path.iter().cloned().chain(std::iter::once(n)).collect();
        let paths = gather_paths(new_path, dest.clone(), map);
        if path.len() > 0 {
            final_paths.extend(paths);
        }
    }
    final_paths
}
