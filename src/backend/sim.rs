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
