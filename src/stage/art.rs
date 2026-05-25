use crate::data;
use crate::stage::Stage;
use crate::typed;

pub struct ArtState {
    pub ch: i32,
    pub dy: i32,
}

impl ArtState {
    pub fn new(ch: i32) -> Self {
        Self { ch, dy: 0 }
    }

    pub fn done(&self, stage: &Stage) -> bool {
        self.dy >= stage.ascii_art_height
    }

    pub fn tick(&mut self, stage: &Stage) {
        if self.done(stage) {
            return;
        }
        let art = data::ARTS[self.ch as usize];
        stage.move_to(stage.ascii_art_x, stage.ascii_art_y + self.dy);
        typed!("{}", art[self.dy as usize]);
        self.dy += 1;
    }
}
