use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Runner {
    road: usize,
    pos: usize,
    hp: f32,
}

impl Runner {
    pub fn new(road: usize, pos: usize) -> Self {
        Self {
            road,
            pos,
            hp: 40.0,
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
}

impl Component for Runner {
    type Storage = DenseVecStorage<Self>;
}
