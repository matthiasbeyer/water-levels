#[derive(Debug)]
pub struct LandscapeElement {
    base_height: usize,
    water_level: f32,
}

impl LandscapeElement {
    pub fn new(base_height: usize) -> Self {
        Self { base_height, water_level: 0.0 }
    }

    pub fn rain_one_hour(&mut self) {
        self.water_level += 1.0;
    }

    pub fn current_height(&self) -> f32 {
        (self.base_height as f32) + self.water_level
    }

    pub fn increase_height(&mut self, h: f32) {
        self.water_level += h;
    }

    pub fn decrease_height(&mut self, h: f32) {
        self.water_level -= h;
    }

    pub fn into_inner(self) -> (usize, f32) {
        (self.base_height, self.water_level)
    }
}
