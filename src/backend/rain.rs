#[derive(Debug, std::ops::Div)]
pub struct RainElement(f32);

impl RainElement {
    pub fn full() -> Self {
        RainElement(1)
    }
}

impl RainElement {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
}

