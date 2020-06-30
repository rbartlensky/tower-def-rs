use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::{runner::Runner, Coord};

pub const MISSLE_SPEED: f32 = 64.0;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TowerKind {
    Simple,
    Frost,
}

impl TowerKind {
    pub fn radius(&self) -> f32 {
        match self {
            TowerKind::Simple => 30.0,
            TowerKind::Frost => 20.0,
        }
    }

    pub fn damage(&self) -> f32 {
        match self {
            TowerKind::Simple => 20.0,
            TowerKind::Frost => 10.0,
        }
    }

    pub fn sprite_number(&self) -> usize {
        match self {
            TowerKind::Simple => 23,
            TowerKind::Frost => 24,
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

    pub fn sprite_number(&self) -> usize {
        self.kind.sprite_number()
    }

    pub fn debuff(&self) -> Option<Debuff> {
        if let TowerKind::Simple = self.kind {
            None
        } else {
            Some(Debuff::new(
                1.0,
                Box::new(|r| r.set_speed(r.speed() * 0.5)),
                Box::new(|r| r.set_speed(r.speed() * 2.)),
            ))
        }
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

type DebuffClosure = Box<dyn FnOnce(&mut Runner) + Send + Sync>;

pub struct Debuff {
    duration: f32,
    on_start: Option<DebuffClosure>,
    on_end: Option<DebuffClosure>,
}

impl Debuff {
    pub fn new(duration: f32, on_start: DebuffClosure, on_end: DebuffClosure) -> Self {
        Self {
            duration,
            on_start: Some(on_start),
            on_end: Some(on_end),
        }
    }

    pub fn start(&mut self, r: &mut Runner) {
        if let Some(on_start) = self.on_start.take() {
            (on_start)(r);
        }
    }

    pub fn tick(&mut self, duration: f32) {
        self.duration -= duration;
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn end(&mut self, r: &mut Runner) {
        if let Some(on_end) = self.on_end.take() {
            (on_end)(r);
        }
    }
}

pub struct Missle {
    target: u32,
    damage: f32,
    debuff: Option<Debuff>,
}

impl Missle {
    pub fn new(target: u32, damage: f32, debuff: Option<Debuff>) -> Self {
        Self {
            target,
            damage,
            debuff,
        }
    }

    pub fn target(&self) -> u32 {
        self.target
    }

    pub fn damage(&self) -> f32 {
        self.damage
    }

    pub fn debuff(&mut self) -> Option<Debuff> {
        self.debuff.take()
    }
}

impl Component for Missle {
    type Storage = DenseVecStorage<Self>;
}
