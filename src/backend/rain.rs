pub enum RainElement {
    FullRainElement,
    HalfRainElement
}

impl RainElement {
    pub fn as_f32(&self) -> f32 {
        match self {
            RainElement::FullRainElement => 1.0,
            RainElement::HalfRainElement => 0.5,
        }
    }
}

