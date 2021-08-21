use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use resiter::Map;

use crate::backend::landscape::LandscapeElement;
use crate::backend::rain::*;

pub type Receiver = tokio::sync::mpsc::UnboundedReceiver<SimulationMessage>;
pub type Sender = tokio::sync::mpsc::UnboundedSender<SimulationMessage>;


#[derive(getset::Getters, getset::Setters)]
pub struct ElementRainingSimulation {
    #[getset(get = "pub")]
    element: Arc<RwLock<LandscapeElement>>,
    receiver: Receiver,

    left_neighbor: Option<Neighbor>,
    right_neighbor: Option<Neighbor>,
}

/// Helper struct for grouping an element and the communication channel to the simulation for the
/// element
struct Neighbor {
    element: Arc<RwLock<LandscapeElement>>,
    sender: Sender,
}

impl ElementRainingSimulation {
    pub fn new(element: Arc<RwLock<LandscapeElement>>) -> (Self, Sender) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let rsim = ElementRainingSimulation {
            element,
            receiver,
            left_neighbor: None,
            right_neighbor: None,
        };

        (rsim, sender)
    }

    pub fn set_left_neighbor(&mut self, element: Arc<RwLock<LandscapeElement>>, sender: Sender) {
        self.left_neighbor = Some(Neighbor { element, sender })
    }

    pub fn set_right_neighbor(&mut self, element: Arc<RwLock<LandscapeElement>>, sender: Sender) {
        self.right_neighbor = Some(Neighbor { element, sender })
    }

    pub fn into_landscape_element(self) -> Arc<RwLock<LandscapeElement>> {
        self.element
    }

    pub async fn let_it_rain(self, mut hours: usize) -> Result<Self> {
        log::debug!("Let it rain for {} hours", hours);

        unimplemented!()
    }
}

/// A message for exchanging data between simulation elements
#[derive(Debug)]
pub enum SimulationMessage {
}

