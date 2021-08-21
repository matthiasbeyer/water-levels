use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use resiter::Map;

use crate::util::*;
use crate::backend::sim::*;
use crate::backend::rain::*;

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
    pub async fn let_it_rain(self, hours: usize) -> Result<RainyLandscape> {
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

                let el = window[0].element().clone();
                window[1].set_left_neighbor(Some(el));

                let el = window[1].element().clone();
                window[0].set_right_neighbor(Some(el));

                let el = window[2].element().clone();
                window[1].set_right_neighbor(Some(el));

                let el = window[1].element().clone();
                window[2].set_left_neighbor(Some(el));
            }
        } else if simulation_elements.len() == 2 {
            log::debug!("Having only two landscape elements in the simulation, pairing them now");
            let el = simulation_elements[1].element().clone();
            simulation_elements[0].set_right_neighbor(Some(el));

            let el = simulation_elements[0].element().clone();
            simulation_elements[1].set_left_neighbor(Some(el));
        }

        Ok(simulation_elements)
    }
}

#[derive(getset::Setters, getset::Getters)]
pub struct LandscapeElement {
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
