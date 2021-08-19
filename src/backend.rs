use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use resiter::Map;

use crate::util::*;


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
    pub async fn let_it_rain(mut self, hours: usize) -> Result<RainyLandscape> {
        use futures::StreamExt;

        self.prepare_element_simulation()
            .await?
            .into_iter()
            .map(|el| el.let_it_rain(hours))
            .collect::<futures::stream::FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map_ok(ElementRainingSimulation::into_landscape_element)
            .collect::<Result<Vec<_>>>()
            .map(RainyLandscape::new)
    }

    async fn prepare_element_simulation(&self) -> Result<Vec<ElementRainingSimulation>> {
        let mut simulation_elements = self.elements
            .iter()
            .map(Arc::clone)
            .map(ElementRainingSimulation::new)
            .collect::<Vec<_>>();

        if simulation_elements.len() >= 3 {
            log::debug!("Having three or more landscape elements in the simulation, pairing them now");
            let mut windows = ThreeElemWindowMut::new(&mut simulation_elements);
            while let Some(mut window) = windows.next() {
                // we know from the ThreeElemWindowMut impl that 'window' always has three elements

                window[1].left_neighbor = Some(window[0].element.clone());
                window[0].right_neighbor = Some(window[1].element.clone());

                window[1].right_neighbor = Some(window[2].element.clone());
                window[2].left_neighbor = Some(window[1].element.clone());
            }
        } else if simulation_elements.len() == 2 {
            log::debug!("Having only two landscape elements in the simulation, pairing them now");
            simulation_elements[0].right_neighbor = Some(simulation_elements[1].element.clone());
            simulation_elements[1].left_neighbor = Some(simulation_elements[0].element.clone());
        }

        Ok(simulation_elements)
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

struct ElementRainingSimulation {
    element: Arc<RwLock<LandscapeElement>>,

    left_neighbor: Option<Arc<RwLock<LandscapeElement>>>,
    right_neighbor: Option<Arc<RwLock<LandscapeElement>>>,
}

impl ElementRainingSimulation {
    pub fn new(element: Arc<RwLock<LandscapeElement>>) -> Self {
        ElementRainingSimulation {
            element,
            left_neighbor: None,
            right_neighbor: None,
        }
    }

    pub fn into_landscape_element(self) -> Arc<RwLock<LandscapeElement>> {
        self.element
    }

    pub async fn let_it_rain(self, mut hours: usize) -> Result<Self> {
        log::debug!("Let it rain for {} hours", hours);

        while hours != 0 {
            match (self.left_neighbor.as_ref(), self.right_neighbor.as_ref()) {
                // simple case: this is the only element on the landscape, so it only rains on this
                // element
                (None, None) => {
                    log::debug!("No neighbors, catching all the rain in myself");
                    self.element.write().await.increase_rain_level(RainElement::FullRainElement)
                },

                // This element is the rightmost element in the landscape
                (Some(left), None) => {
                    log::debug!("Only a left neighbor");
                    let (mut left_writelock, mut this_writelock) = tokio::join!(left.write(), self.element.write());
                    log::trace!("left: {}, self: {}", left_writelock.get_current_height(), this_writelock.get_current_height());

                    if left_writelock.get_current_height() < this_writelock.get_current_height() {
                        log::debug!("Rain floats to my left");
                        left_writelock.increase_rain_level(RainElement::FullRainElement);
                    } else {
                        log::debug!("Rain floats to me");
                        this_writelock.increase_rain_level(RainElement::FullRainElement);
                    }
                }

                // This element is the leftmost element in the landscape
                (None, Some(right)) => {
                    log::debug!("Only a right neighbor");
                    let (mut this_writelock, mut right_writelock) = tokio::join!(self.element.write(), right.write());
                    log::trace!("self: {}, right: {}", this_writelock.get_current_height(), right_writelock.get_current_height());

                    if this_writelock.get_current_height() > right_writelock.get_current_height() {
                        log::debug!("Rain floats to my right");
                        right_writelock.increase_rain_level(RainElement::FullRainElement);
                    } else {
                        log::debug!("Rain floats to me");
                        this_writelock.increase_rain_level(RainElement::FullRainElement);
                    }
                }

                (Some(left), Some(right)) => {
                    log::debug!("Both neighbors");
                    let (mut left_writelock, mut this_writelock, mut right_writelock) = tokio::join!(left.write(), self.element.write(), right.write());

                    let l_h = left_writelock.get_current_height();
                    let t_h = this_writelock.get_current_height();
                    let r_h = right_writelock.get_current_height();
                    log::trace!("left: {}, self: {}, right: {}", l_h, t_h, r_h);

                    if l_h < t_h && r_h < t_h {
                        // we are higher than both our neighbors
                        log::debug!("self is highest, floating to both neighbors");
                        left_writelock.increase_rain_level(RainElement::HalfRainElement);
                        right_writelock.increase_rain_level(RainElement::HalfRainElement);
                    } else if l_h < t_h && r_h >= t_h {
                        // only left of us is lower than us
                        log::debug!("left is lower than me, floating to left");
                        left_writelock.increase_rain_level(RainElement::FullRainElement);
                    } else if l_h >= t_h && r_h < t_h {
                        // only right of us is lower than us
                        log::debug!("right is lower than me, floating to right");
                        right_writelock.increase_rain_level(RainElement::FullRainElement);
                    } else if l_h >= t_h && r_h >= t_h {
                        // both neighbors are higher than us
                        log::debug!("both are higher than me, floating to me");
                        this_writelock.increase_rain_level(RainElement::FullRainElement);
                    }
                }
            }

            hours -= 1;
            log::debug!("{} hours left", hours);
        }

        Ok(self)
    }
}

#[derive(getset::Getters)]
pub struct RainyLandscape {
    #[getset(get = "pub")]
    elements: Vec<Arc<RwLock<LandscapeElement>>>
}

impl RainyLandscape {
    fn new(elements: Vec<Arc<RwLock<LandscapeElement>>>) -> Self {
        RainyLandscape { elements }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    #[tokio::test]
    async fn let_it_rain_simple() {
        let _ = env_logger::try_init();
        let ls = Landscape::new(vec![1]);
        let rls = ls.let_it_rain(1).await.unwrap();

        assert_eq!(rls.elements().len(), 1);
        assert_float_eq!(rls.elements().get(0).unwrap().read().await.get_current_height(), 2.0, abs <= 0.000_1);
    }

    #[tokio::test]
    async fn let_it_rain_two_landscape_elements() {
        let _ = env_logger::try_init();
        let ls = Landscape::new(vec![3, 1]);
        let rls = ls.let_it_rain(1).await.unwrap();

        assert_eq!(rls.elements().len(), 2);
        let elements = rls.elements();

        assert_float_eq!(elements.get(0).unwrap().read().await.get_current_height(), 3.0, abs <= 0.000_1);
        assert_float_eq!(elements.get(1).unwrap().read().await.get_current_height(), 3.0, abs <= 0.000_1);
    }

    #[tokio::test]
    async fn let_it_rain_three_landscape_elements_one_hour() {
        let _ = env_logger::try_init();
        let ls = Landscape::new(vec![3, 1, 3]);
        let rls = ls.let_it_rain(1).await.unwrap();

        assert_eq!(rls.elements().len(), 3);
        let elements = rls.elements();

        let mut heights = vec![];
        heights.push(elements.get(0).unwrap().read().await.get_current_height());
        heights.push(elements.get(1).unwrap().read().await.get_current_height());
        heights.push(elements.get(2).unwrap().read().await.get_current_height());

        assert_float_eq!(heights[0], 3.0, abs <= 0.000_1);
        assert_float_eq!(heights[1], 3.0, abs <= 0.000_1);
        assert_float_eq!(heights[2], 3.0, abs <= 0.000_1);
    }

    #[tokio::test]
    async fn let_it_rain_three_landscape_elements_two_hours() {
        let _ = env_logger::try_init();
        let ls = Landscape::new(vec![3, 1, 3]);
        let rls = ls.let_it_rain(2).await.unwrap();

        assert_eq!(rls.elements().len(), 3);
        let elements = rls.elements();

        let mut heights = vec![];
        heights.push(elements.get(0).unwrap().read().await.get_current_height());
        heights.push(elements.get(1).unwrap().read().await.get_current_height());
        heights.push(elements.get(2).unwrap().read().await.get_current_height());

        assert_float_eq!(heights[0], 4.0, abs <= 0.000_1);
        assert_float_eq!(heights[1], 4.0, abs <= 0.000_1);
        assert_float_eq!(heights[2], 4.0, abs <= 0.000_1);
    }
}
