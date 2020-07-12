use amethyst::assets::Handle;
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

use std::collections::HashMap;

use crate::tower::{utils, BuildPoint, Missle, Tower, TowerKind, MISSLE_SPEED};
use crate::{map::Map, runner::Runner, GameState};

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
        Read<'s, GameState>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut transforms, runners, mut towers, map, mut missles, mut sprites, time, state, entities): Self::SystemData,
    ) {
        if *state != GameState::Game {
            return;
        }
        let map = (&map).join().next().unwrap();
        let mut missle_comps = vec![];
        let time = time.delta_seconds();
        for (tower, t_trans) in (&mut towers, &transforms).join() {
            tower.tick(time);
            for (_, r_trans, ent) in (&runners, &transforms, &entities).join() {
                if utils::in_range(&t_trans, tower.radius(), &r_trans) && tower.cd() <= 0. {
                    let debuff = tower.debuff();
                    missle_comps.push((
                        Missle::new(ent.id(), tower.damage(), debuff),
                        t_trans.clone(),
                    ));
                    tower.reset_cd();
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
    selector: Vec<(Entity, TowerKind)>,
    menus: HashMap<TowerKind, Vec<(Entity, TowerKind)>>,
    shown: Vec<(Entity, TowerKind)>,
}

impl BuildPointSystem {
    pub fn new() -> Self {
        Self {
            selected: None,
            selector: Vec::with_capacity(2),
            menus: Default::default(),
            shown: vec![],
        }
    }

    fn maybe_init<'s>(
        &mut self,
        handle: Handle<amethyst::renderer::SpriteSheet>,
        sprites: &mut WriteStorage<'s, SpriteRender>,
        transforms: &mut WriteStorage<'s, Transform>,
        entities: &Entities<'s>,
    ) {
        if self.selector.len() == 0 {
            let mut tr = Transform::default();
            // out of sight
            tr.translation_mut().z = 2.0;
            for tk in [TowerKind::Simple, TowerKind::Frost].iter() {
                self.selector.push((
                    entities
                        .build_entity()
                        .with(
                            SpriteRender {
                                sprite_sheet: handle.clone(),
                                sprite_number: tk.sprite_number(),
                            },
                            sprites,
                        )
                        .with(tr.clone(), transforms)
                        .build(),
                    *tk,
                ));
            }
        }
    }

    fn show_selector<'s>(
        &mut self,
        mut trans: Transform,
        transforms: &mut WriteStorage<'s, Transform>,
    ) {
        self.hide_all(transforms);
        trans.translation_mut().z = 0.5;
        trans.translation_mut().y += 16.0;
        trans.translation_mut().x -= 24.0;
        for (button, tk) in &self.selector {
            trans.translation_mut().x += 16.0;
            *transforms.get_mut(button.clone()).unwrap() = trans.clone();
            self.shown.push((button.clone(), *tk));
        }
    }

    fn maybe_init_menu<'s>(
        &mut self,
        handle: Handle<amethyst::renderer::SpriteSheet>,
        sprites: &mut WriteStorage<'s, SpriteRender>,
        transforms: &mut WriteStorage<'s, Transform>,
        entities: &Entities<'s>,
    ) {
        for tk in [TowerKind::Simple, TowerKind::Frost, TowerKind::Turret].iter() {
            if !self.menus.contains_key(tk) {
                let mut tr = Transform::default();
                // out of sight
                tr.translation_mut().z = 2.0;
                self.menus.insert(
                    *tk,
                    tk.upgrades()
                        .into_iter()
                        .map(|tk| {
                            (
                                entities
                                    .build_entity()
                                    .with(
                                        SpriteRender {
                                            sprite_sheet: handle.clone(),
                                            sprite_number: tk.sprite_number(),
                                        },
                                        sprites,
                                    )
                                    .with(tr.clone(), transforms)
                                    .build(),
                                tk,
                            )
                        })
                        .collect(),
                );
            }
        }
    }

    fn show_menu<'s>(
        &mut self,
        tk: TowerKind,
        mut trans: Transform,
        transforms: &mut WriteStorage<'s, Transform>,
    ) {
        self.hide_all(transforms);
        trans.translation_mut().z = 0.5;
        trans.translation_mut().y += 16.0;
        trans.translation_mut().x -= 24.0;
        for (button, tk) in &self.menus[&tk] {
            trans.translation_mut().x += 16.0;
            *transforms.get_mut(button.clone()).unwrap() = trans.clone();
            self.shown.push((button.clone(), *tk));
        }
    }

    fn hide_all<'s>(&mut self, transforms: &mut WriteStorage<'s, Transform>) {
        // hide the menu
        let _ = self
            .shown
            .drain(..)
            .map(|(e, _)| transforms.get_mut(e).unwrap().translation_mut().z = 2.0)
            .collect::<Vec<()>>();
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
        WriteStorage<'s, Map>,
        WriteStorage<'s, UiText>,
        Read<'s, GameState>,
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
            mut map,
            mut texts,
            state,
        ): Self::SystemData,
    ) {
        if *state != GameState::Game {
            return;
        }
        let map = (&mut map).join().next().unwrap();
        let handle = map.sprite_sheet_handle();
        // initialise our hidden buttons (if they're not already)
        self.maybe_init(handle.clone(), &mut sprites, &mut transforms, &entities);
        self.maybe_init_menu(handle.clone(), &mut sprites, &mut transforms, &entities);

        let (camera, camera_trans) = (&camera, &transforms).join().next().unwrap();
        if let Some(mouse_trans) = utils::mouse_position(&input, &dim, camera, camera_trans) {
            // check if we clicked on any build points
            for (ent, _, trans) in (&entities, &points, &transforms).join() {
                if utils::in_range(trans, (map.tile_width() / 2) as f32, &mouse_trans) {
                    // mark the build point as the currently selected one
                    self.selected = Some(ent);
                    // show the selector
                    self.show_selector(trans.clone(), &mut transforms);
                    return;
                }
            }

            // check if we clicked on any towers
            for (ent, tower, trans) in (&entities, &towers, &transforms).join() {
                if utils::in_range(trans, (map.tile_width() / 2) as f32, &mouse_trans) {
                    // mark the tower as the currently selected one
                    self.selected = Some(ent);
                    // show the tower upgrade menu
                    self.show_menu(tower.kind(), trans.clone(), &mut transforms);
                    return;
                }
            }

            let mut tower = None;
            // did we click on a button that is enabled?
            for (button, tk) in &self.shown {
                let trans = transforms.get_mut(button.clone()).unwrap();
                if self.selected.is_some()
                    && tower.is_none()
                    && utils::in_range(trans, (map.tile_width() / 2) as f32, &mouse_trans)
                {
                    if tk.cost() <= map.gold() {
                        map.remove_gold(tk.cost());
                        texts.get_mut(map.gold_text()).unwrap().text =
                            format!("{} gold", map.gold());
                        let pos = if let Some(point) = points.get(self.selected.clone().unwrap()) {
                            point.pos()
                        } else {
                            // if it wasn't a build point, we must've upgraded a tower
                            towers.get(self.selected.clone().unwrap()).unwrap().pos()
                        };
                        tower = Some(Tower::new(*tk, pos));
                        // can't click on two buttons...
                        break;
                    } else {
                        // we don't want to hide the menu selector!
                        let error_text = texts.get_mut(map.error_text()).unwrap();
                        error_text.text = format!("Not enough resources!");
                        error_text.color[3] = 1.;
                        return;
                    }
                }
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
                // remove the previous tower/build point
                // XXX we could reuse the entity or some parts of it!
                entities.delete(self.selected.take().unwrap()).unwrap();
            }
            self.hide_all(&mut transforms);
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
        Read<'s, GameState>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut map, mut transforms, mut runners, mut missles, time, mut texts, state, entities): Self::SystemData,
    ) {
        if *state != GameState::Game {
            return;
        }
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
                    texts.get_mut(map.gold_text()).unwrap().text = format!("{} gold", map.gold());
                }
            } else {
                norm.x *= time * MISSLE_SPEED;
                norm.y *= time * MISSLE_SPEED;
                transforms.get_mut(ent).unwrap().append_translation(norm);
            }
        }
    }
}
