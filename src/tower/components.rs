use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::Coord;

pub const MISSLE_SPEED: f32 = 10.0;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TowerKind {
    Simple,
    Frost,
}

impl TowerKind {
    pub fn radius(&self) -> f32 {
        match self {
            TowerKind::Simple => 20.0,
            TowerKind::Frost => 1.2,
        }
    }

    pub fn damage(&self) -> f32 {
        match self {
            TowerKind::Simple => 20.0,
            TowerKind::Frost => 15.0,
        }
    }
}

pub struct Tower {
    kind: TowerKind,
    pos: Coord,
    cd: f32,
}

impl Tower {
    pub fn new(kind: TowerKind, pos: Coord) -> Tower {
        Tower { kind, pos, cd: 1.0 }
    }

    pub fn radius(&self) -> f32 {
        self.kind.radius()
    }

    pub fn damage(&self) -> f32 {
        self.kind.damage()
    }

    pub fn kind(&self) -> TowerKind {
        self.kind
    }

    pub fn pos(&self) -> Coord {
        self.pos
    }

    pub fn cd(&self) -> f32 {
        self.cd
    }

    pub fn set_cd(&mut self, cd: f32) {
        self.cd = cd;
    }
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}

pub struct BuildPoint {
    pos: Coord,
}

impl BuildPoint {
    pub fn new(pos: Coord) -> Self {
        Self { pos }
    }

    pub fn pos(&self) -> Coord {
        self.pos
    }
}

impl Component for BuildPoint {
    type Storage = DenseVecStorage<Self>;
}

pub struct Missle {
    target: u32,
    damage: f32,
}

impl Missle {
    pub fn new(target: u32, damage: f32) -> Self {
        Self { target, damage }
    }

    pub fn target(&self) -> u32 {
        self.target
    }

    pub fn damage(&self) -> f32 {
        self.damage
    }
}

impl Component for Missle {
    type Storage = DenseVecStorage<Self>;
}
