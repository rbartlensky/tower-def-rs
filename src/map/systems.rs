use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::ui::UiText;

use crate::map::Map;

#[derive(SystemDesc)]
pub struct FontSystem {
    font_timer: f32,
}

impl FontSystem {
    pub fn new() -> Self {
        Self { font_timer: 0.0 }
    }
}

impl<'s> System<'s> for FontSystem {
    type SystemData = (
        ReadStorage<'s, Map>,
        Read<'s, Time>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (mut map, time, mut texts): Self::SystemData) {
        let map = (&mut map).join().next().unwrap();
        let time = time.delta_seconds();
        let error_text = texts.get_mut(map.error_text()).unwrap();
        if error_text.color[3] >= 1.0 {
            self.font_timer = 3.;
        }
        if self.font_timer > 0. {
            self.font_timer -= time;
        }
        error_text.color[3] = self.font_timer / 3.0;
    }
}
