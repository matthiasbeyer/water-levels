use crate::backend::landscape_element::*;

pub struct Landscape {
    elements: Vec<LandscapeElement>,
}

impl Landscape {
    pub fn new(elements: Vec<usize>) -> Self {
        Landscape {
            elements: elements.into_iter().map(LandscapeElement::new).collect()
        }
    }

    pub fn rain(mut self, hours: usize) -> Self {
        for _ in 0..hours {
            for elem in self.elements.iter_mut() {
                elem.rain_one_hour();
            }
            crate::backend::algo::rearrange(&mut self.elements)
        }
        self
    }

    pub fn into_inner(self) -> Vec<(usize, f32)> {
        self.elements.into_iter()
            .map(LandscapeElement::into_inner)
            .collect()
    }

}

