#[derive(Debug)]
pub struct RainElement(f32);

impl RainElement {
    pub fn full() -> Self {
        RainElement(1.0)
    }

    pub fn half(self) -> Self {
        RainElement(self.0 * 0.5)
    }
}

impl RainElement {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
}

