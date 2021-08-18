use anyhow::Result;
use std::sync::Arc;
use std::collections::LinkedList;
use tokio::sync::RwLock;

pub struct Landscape {
    elements: Vec<Arc<RwLock<LandscapeElement>>>,
}

impl Landscape {
    pub fn new(elements: Vec<usize>) -> Self {
        let elements = elements
            .into_iter()
            .map(LandscapeElement::new)
            .map(RwLock::new)
            .map(Arc::new)
            .collect();

        Landscape { elements }
    }

    // here is where the fun is
    pub fn let_it_rain(mut self, mut hours: usize) -> RainyLandscape {
        unimplemented!()
    }
}

enum RainElement {
    FullRainElement,
    HalfRainElement
}

impl RainElement {
    fn as_f32(&self) -> f32 {
        match self {
            RainElement::FullRainElement => 1.0,
            RainElement::HalfRainElement => 0.5,
        }
    }
}

#[derive(getset::Setters, getset::Getters)]
struct LandscapeElement {
    #[getset(get = "pub")]
    height: usize,

    #[getset(get = "pub")]
    rain_elements: Vec<RainElement>,
}

impl LandscapeElement {
    fn new(height: usize) -> Self {
        LandscapeElement {
            height,
            rain_elements: Vec::new(),
        }
    }

    pub fn get_current_height(&self) -> f32 {
        let rain: f32 = self.rain_elements.iter().map(|e| e.as_f32()).sum();
        (self.height as f32) + rain
    }

    pub fn increase_rain_level(&mut self, el: RainElement) {
        self.rain_elements.push(el);
    }
}

pub struct ElementRainingSimulation {
    element: Arc<RwLock<LandscapeElement>>,

    left_neighbor: Option<Arc<RwLock<LandscapeElement>>>,
    right_neighbor: Option<Arc<RwLock<LandscapeElement>>>,
}

impl ElementRainingSimulation {
    pub async fn let_it_rain(self, mut hours: usize) -> Result<()> {
        while hours != 0 {
            match (self.left_neighbor.as_ref(), self.right_neighbor.as_ref()) {
                // simple case: this is the only element on the landscape, so it only rains on this
                // element
                (None, None) => self.element.write().await.increase_rain_level(RainElement::FullRainElement),


                // This element is the rightmost element in the landscape
                (Some(left), None) => {
                    let (mut left_writelock, mut this_writelock) = tokio::join!(left.write(), self.element.write());

                    if left_writelock.get_current_height() < this_writelock.get_current_height() {
                        left_writelock.increase_rain_level(RainElement::FullRainElement);
                    } else {
                        this_writelock.increase_rain_level(RainElement::FullRainElement);
                    }
                }

                // This element is the leftmost element in the landscape
                (None, Some(right)) => {
                    let (mut this_writelock, mut right_writelock) = tokio::join!(self.element.write(), right.write());

                    if this_writelock.get_current_height() > right_writelock.get_current_height() {
                        right_writelock.increase_rain_level(RainElement::FullRainElement);
                    } else {
                        this_writelock.increase_rain_level(RainElement::FullRainElement);
                    }
                }

                (Some(left), Some(right)) => {
                    let (mut left_writelock, mut this_writelock, mut right_writelock) = tokio::join!(left.write(), self.element.write(), right.write());

                    let l_h = left_writelock.get_current_height();
                    let t_h = this_writelock.get_current_height();
                    let r_h = right_writelock.get_current_height();

                    if l_h < t_h && r_h < t_h {
                        // we are higher than both our neighbors
                        left_writelock.increase_rain_level(RainElement::HalfRainElement);
                        right_writelock.increase_rain_level(RainElement::HalfRainElement);
                    } else if l_h < t_h && r_h >= t_h {
                        // only left of us is lower than us
                        left_writelock.increase_rain_level(RainElement::FullRainElement);
                    } else if l_h >= t_h && r_h < t_h {
                        // only right of us is lower than us
                        right_writelock.increase_rain_level(RainElement::FullRainElement);
                    } else if l_h >= t_h && r_h >= t_h {
                        // both neighbors are higher than us
                        this_writelock.increase_rain_level(RainElement::FullRainElement);
                    }
                }
            }

            hours -= 1;
        }

        Ok(())
    }
}

#[derive(getset::Getters)]
pub struct RainyLandscape {
    #[getset(get = "pub")]
    elements: LinkedList<LandscapeElement>
}

#[cfg(test)]
mod tests {
    #[test]
    fn let_it_rain_simple() {
        let ls = Landscape::new(vec![1]);
        let rls = ls.let_it_rain(1);

        assert_eq!(rls.elements().len(), 1);
        assert_eq!(rls.elements().get(0).unwrap(), 2);
    }
}
