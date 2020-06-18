mod components;
pub use components::Map;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, Sprite, SpriteSheet, Texture},
};

pub fn create_sprite_sheets(
    tile_set: &tiled::Tileset,
    world: &mut World,
) -> Vec<Handle<SpriteSheet>> {
    let mut handles = Vec::with_capacity(tile_set.images.len());
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    let (tile_width, tile_height) = (tile_set.tile_width, tile_set.tile_height);
    for img in tile_set.images.iter() {
        // create a handle to the source file (i.e. the image)
        let texture_handle = loader.load(&img.source, ImageFormat::default(), (), &texture_storage);
        // The image is a sprite sheet, as such it can be divided into
        // (tile_width x tile_height) sections. Each section is a sprite
        let (img_width, img_height) = (img.width as u32, img.height as u32);
        let cols = img_width / tile_width;
        let rows = img_height / tile_height;
        let mut sprites = Vec::with_capacity((rows * cols) as usize);
        for x in 0..rows {
            for y in 0..cols {
                let offset_x = (y * tile_width) as u32;
                let offset_y = (x * tile_height) as u32;
                sprites.push(Sprite::from_pixel_values(
                    img_width,
                    img_height,
                    tile_width,
                    tile_height,
                    offset_x,
                    offset_y,
                    [0.0; 2],
                    false,
                    false,
                ));
            }
        }
        let sprite_sheet = SpriteSheet {
            texture: texture_handle,
            sprites,
        };
        handles.push(loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage));
    }
    handles
}
