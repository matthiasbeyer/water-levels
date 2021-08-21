use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use resiter::Map;

use crate::backend::landscape::LandscapeElement;
use crate::backend::rain::*;

#[derive(getset::Getters, getset::Setters)]
pub struct ElementRainingSimulation {
    #[getset(get = "pub")]
    element: Arc<RwLock<LandscapeElement>>,

    #[getset(set = "pub")]
    left_neighbor: Option<Arc<RwLock<LandscapeElement>>>,
    #[getset(set = "pub")]
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

        unimplemented!()

        Ok(self)
    }
}
