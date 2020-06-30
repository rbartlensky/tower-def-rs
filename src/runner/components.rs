use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::tower::Debuff;

pub struct Runner {
    road: usize,
    pos: usize,
    hp: f32,
    speed: f32,
    debuffs: Vec<Debuff>,
}

impl Runner {
    pub fn new(road: usize, pos: usize) -> Self {
        Self {
            road,
            pos,
            hp: 100.0,
            speed: 32.0,
            debuffs: vec![],
        }
    }

    pub fn road(&self) -> usize {
        self.road
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn hp(&self) -> f32 {
        self.hp
    }

    pub fn deal_damage(&mut self, damage: f32) {
        self.hp -= damage;
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn apply_debuff(&mut self, debuff: Option<Debuff>) {
        if let Some(mut debuff) = debuff {
            debuff.start(self);
            self.debuffs.push(debuff);
        }
    }

    pub fn tick(&mut self, duration: f32) {
        let (not_done, done): (Vec<Debuff>, Vec<Debuff>) = self
            .debuffs
            .drain(..)
            .into_iter()
            .map(|mut d| {
                d.tick(duration);
                d
            })
            .partition(|d| d.duration() > 0.);
        self.debuffs = not_done;
        for mut debuff in done {
            debuff.end(self);
        }
    }
}

impl Component for Runner {
    type Storage = DenseVecStorage<Self>;
}
