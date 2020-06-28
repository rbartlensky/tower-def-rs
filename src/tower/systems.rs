use amethyst::core::timing::Time;
use amethyst::core::{
    math::{Point3, Vector2},
    Transform,
};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage,
};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{Camera, SpriteRender};
use amethyst::window::ScreenDimensions;

use super::{utils, BuildPoint, Missle, Tower, TowerKind, MISSLE_SPEED};
use crate::{map::Map, runner::Runner};

#[derive(SystemDesc)]
pub struct TowerSystem;

impl<'s> System<'s> for TowerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Runner>,
        WriteStorage<'s, Tower>,
        ReadStorage<'s, Map>,
        WriteStorage<'s, Missle>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut transforms, runners, mut towers, map, mut missles, mut sprites, time, entities): Self::SystemData,
    ) {
        let map = (&map).join().next().unwrap();
        let mut missle_comps = vec![];
        let time = time.delta_seconds();
        for (tower, t_trans) in (&mut towers, &transforms).join() {
            tower.set_cd(tower.cd() + time);
            for (_, r_trans, ent) in (&runners, &transforms, &entities).join() {
                if utils::in_range(&t_trans, tower.radius(), &r_trans) && tower.cd() >= 1.0 {
                    missle_comps.push((Missle::new(ent.id(), tower.damage()), t_trans.clone()));
                    tower.set_cd(0.0);
                }
            }
        }
        for (missle, mut trans) in missle_comps {
            let sprite = SpriteRender {
                sprite_sheet: map.sprite_sheet_handle(),
                sprite_number: 143,
            };
            trans.translation_mut().z = 0.5;
            entities
                .build_entity()
                .with(missle, &mut missles)
                .with(trans, &mut transforms)
                .with(sprite, &mut sprites)
                .build();
        }
    }
}

#[derive(SystemDesc)]
pub struct BuildPointSystem;

impl<'s> System<'s> for BuildPointSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, BuildPoint>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        Entities<'s>,
        WriteStorage<'s, Tower>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Map>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            points,
            input,
            dim,
            camera,
            entities,
            mut towers,
            mut sprites,
            map,
        ): Self::SystemData,
    ) {
        let map = (&map).join().next().unwrap();
        if input.mouse_button_is_down(amethyst::winit::MouseButton::Left) {
            if let Some(m_pos) = input.mouse_position() {
                let (camera, camera_trans) = (&camera, &transforms).join().next().unwrap();
                let screen_dimensions = Vector2::new(dim.width(), dim.height());
                let mouse_pos = Point3::new(m_pos.0, m_pos.1, 0.0);
                let mouse_coords = camera.projection().screen_to_world_point(
                    mouse_pos,
                    screen_dimensions,
                    &camera_trans,
                );
                let mut mouse_trans = Transform::default();
                mouse_trans.set_translation_xyz(
                    mouse_coords.coords[0],
                    mouse_coords.coords[1],
                    1.0,
                );
                let mut tower = None;
                for (ent, bp, trans) in (&entities, &points, &transforms).join() {
                    if utils::in_range(trans, (map.tile_width() / 2) as f32, &mouse_trans) {
                        let mut trans = trans.clone();
                        trans.translation_mut().z = 0.5;
                        tower = Some((trans, bp.pos()));
                        entities.delete(ent).unwrap();
                        break;
                    }
                }
                if let Some(tower) = tower {
                    let handle = map.sprite_sheet_handle();
                    let sprite = SpriteRender {
                        sprite_sheet: handle,
                        sprite_number: 23,
                    };
                    entities
                        .build_entity()
                        .with(tower.0, &mut transforms)
                        .with(Tower::new(TowerKind::Simple, tower.1), &mut towers)
                        .with(sprite, &mut sprites)
                        .build();
                }
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct MissleSystem;

impl<'s> System<'s> for MissleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Runner>,
        WriteStorage<'s, Missle>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, mut runners, missles, time, entities): Self::SystemData) {
        let time = time.delta_seconds();
        for (missle, ent) in (&missles, &entities).join() {
            let target_ent = entities.entity(missle.target());
            let (mut norm, is_close) = {
                let target_tr = if let Some(trans) = transforms.get(target_ent) {
                    trans
                } else {
                    // our missle's target is gone, well we are going away as well
                    entities.delete(ent).unwrap();
                    continue;
                };
                let trans = transforms.get(ent).unwrap();
                (
                    utils::normalize(trans, &target_tr),
                    utils::in_range(trans, 8.0, &target_tr),
                )
            };
            if is_close {
                let runner = if let Some(runner) = runners.get_mut(target_ent) {
                    runner
                } else {
                    // our missle's target is gone, well we are going away as well
                    entities.delete(ent).unwrap();
                    continue;
                };
                runner.deal_damage(missle.damage());
                entities.delete(ent).unwrap();
                if runner.hp() <= 0.0 {
                    entities.delete(target_ent).unwrap();
                }
            } else {
                norm.x += time * MISSLE_SPEED;
                norm.y += time * MISSLE_SPEED;
                transforms.get_mut(ent).unwrap().append_translation(norm);
            }
        }
    }
}
