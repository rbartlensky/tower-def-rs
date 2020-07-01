use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage,
};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{Camera, SpriteRender};
use amethyst::ui::UiText;
use amethyst::window::ScreenDimensions;

use crate::tower::{utils, BuildPoint, Missle, Tower, TowerKind, MISSLE_SPEED};
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
                    let debuff = tower.debuff();
                    missle_comps.push((
                        Missle::new(ent.id(), tower.damage(), debuff),
                        t_trans.clone(),
                    ));
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
pub struct BuildPointSystem {
    // currently selected build point
    selected: Option<Entity>,
    buttons: Vec<(Entity, TowerKind)>,
}

impl BuildPointSystem {
    pub fn new() -> Self {
        Self {
            selected: None,
            buttons: vec![],
        }
    }
}

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
        let handle = map.sprite_sheet_handle();
        // initialise our hidden buttons
        if self.buttons.len() == 0 {
            let mut tr = Transform::default();
            // out of sight
            tr.translation_mut().z = 2.0;
            for tk in [TowerKind::Simple, TowerKind::Frost].iter() {
                self.buttons.push((
                    entities
                        .build_entity()
                        .with(
                            SpriteRender {
                                sprite_sheet: handle.clone(),
                                sprite_number: tk.sprite_number(),
                            },
                            &mut sprites,
                        )
                        .with(tr.clone(), &mut transforms)
                        .build(),
                    *tk,
                ));
            }
        }
        let (camera, camera_trans) = (&camera, &transforms).join().next().unwrap();
        if let Some(mouse_trans) = utils::mouse_position(&input, &dim, camera, camera_trans) {
            let mut saved_trans = None;
            // check if we clicked on any build points
            for (ent, _, trans) in (&entities, &points, &transforms).join() {
                if utils::in_range(trans, (map.tile_width() / 2) as f32, &mouse_trans) {
                    self.selected = Some(ent);
                    // save the trans
                    saved_trans = Some(trans.clone());
                    break;
                }
            }

            // we did click on a build point, this means we need to update
            // where our tower selector is placed
            if let Some(mut trans) = saved_trans {
                trans.translation_mut().z = 0.5;
                trans.translation_mut().y += 16.0;
                trans.translation_mut().x -= 24.0;
                for (button, _) in &self.buttons {
                    trans.translation_mut().x += 16.0;
                    *transforms.get_mut(button.clone()).unwrap() = trans.clone();
                }
                return;
            }
            let mut tower = None;
            // did we click on a button?
            for (button, tk) in &self.buttons {
                let trans = transforms.get_mut(button.clone()).unwrap();
                if self.selected.is_some()
                    && tower.is_none()
                    && utils::in_range(trans, (map.tile_width() / 2) as f32, &mouse_trans)
                {
                    tower = Some(Tower::new(
                        *tk,
                        points.get(self.selected.clone().unwrap()).unwrap().pos(),
                    ));
                }
                // in any case, we will hide the button:
                // * if we clicked on it, we will add a tower, therefore the
                //   selector needs to be hidden
                // * we didn't click on the button, therefore we need to hide it
                trans.translation_mut().z = 2.0;
            }
            if let Some(tower) = tower {
                let sprite = SpriteRender {
                    sprite_sheet: handle,
                    sprite_number: tower.sprite_number(),
                };
                // build our tower
                entities
                    .build_entity()
                    .with(
                        tower
                            .pos()
                            .to_trans(map.tile_width() as usize, map.tile_height() as usize),
                        &mut transforms,
                    )
                    .with(tower, &mut towers)
                    .with(sprite, &mut sprites)
                    .build();
                // remove the point
                entities.delete(self.selected.take().unwrap()).unwrap();
            }
            self.selected = None;
        }
    }
}

#[derive(SystemDesc)]
pub struct MissleSystem;

impl<'s> System<'s> for MissleSystem {
    type SystemData = (
        WriteStorage<'s, Map>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Runner>,
        WriteStorage<'s, Missle>,
        Read<'s, Time>,
        WriteStorage<'s, UiText>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut map, mut transforms, mut runners, mut missles, time, mut texts, entities): Self::SystemData,
    ) {
        let map = (&mut map).join().next().unwrap();
        let time = time.delta_seconds();
        for (missle, ent) in (&mut missles, &entities).join() {
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
                runner.apply_debuff(missle.debuff());
                entities.delete(ent).unwrap();
                if runner.hp() <= 0.0 {
                    map.add_gold(runner.bounty());
                    entities.delete(target_ent).unwrap();
                    // update labels as well
                    (&mut texts).join().next().unwrap().text = format!("{} gold", map.gold());
                }
            } else {
                norm.x *= time * MISSLE_SPEED;
                norm.y *= time * MISSLE_SPEED;
                transforms.get_mut(ent).unwrap().append_translation(norm);
            }
        }
    }
}
