use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage};

use crate::{GameState, map::Map, runner::Runner, tower::utils};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct RunnerSystem;

impl<'s> System<'s> for RunnerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Runner>,
        ReadStorage<'s, Map>,
        Read<'s, Time>,
        Read<'s, GameState>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, mut runners, map, time, state, entities): Self::SystemData) {
        if *state != GameState::Game {
            return;
        }
        let map = (&map).join().next().unwrap();
        let road = map.road();
        let time = time.delta_seconds();
        for (runner, transform, ent) in (&mut runners, &mut transforms, &entities).join() {
            runner.tick(time);
            let road = &road[runner.road()];
            let runner_pos = runner.pos();
            let next_pos = road.get(runner_pos + 1);
            if let Some(next_pos) = next_pos {
                let target_trans =
                    next_pos.to_trans(map.tile_width() as usize, map.tile_height() as usize);
                let mut norm = utils::normalize(transform, &target_trans);
                if utils::in_range(transform, 1.0, &target_trans) {
                    runner.set_pos(runner_pos + 1);
                } else {
                    norm.x *= time * runner.speed();
                    norm.y *= time * runner.speed();
                    transform.append_translation(norm);
                }
            } else {
                // we have reached the end!
                entities.delete(ent).unwrap();
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct SpawnSystem {
    spawn_timer: f32,
}

impl SpawnSystem {
    pub fn new() -> Self {
        Self { spawn_timer: 0.0 }
    }
}

impl<'s> System<'s> for SpawnSystem {
    type SystemData = (
        ReadStorage<'s, Map>,
        Read<'s, Time>,
        WriteStorage<'s, Runner>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, GameState>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (map, time, mut runners, mut trans, mut sprites, state, entities): Self::SystemData,
    ) {
        if *state != GameState::Game {
            return;
        }
        let map = (&map).join().next().unwrap();
        let time = time.delta_seconds();
        self.spawn_timer += time;
        if self.spawn_timer > 1.0 {
            let sprite = SpriteRender {
                sprite_sheet: map.sprite_sheet_handle(),
                sprite_number: 125,
            };
            let r = rand::random::<usize>() % map.road().len();
            entities
                .build_entity()
                .with(Runner::new(r, 0), &mut runners)
                .with(
                    map.road()[r][0]
                        .to_trans(map.tile_width() as usize, map.tile_height() as usize),
                    &mut trans,
                )
                .with(sprite, &mut sprites)
                .build();
            self.spawn_timer = 0.0;
        }
    }
}
