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
            rearrange(&mut self.elements)
        }
        self
    }

}

#[derive(Debug)]
struct LandscapeElement {
    base_height: usize,
    water_level: f32,
}

impl LandscapeElement {
    fn new(base_height: usize) -> Self {
        Self { base_height, water_level: 0.0 }
    }

    fn rain_one_hour(&mut self) {
        self.water_level += 1.0;
    }

    fn current_height(&self) -> f32 {
        (self.base_height as f32) + self.water_level
    }

    fn increase_height(&mut self, h: f32) {
        self.water_level += h;
    }
}

fn rearrange(land: &mut [LandscapeElement]) {
    log::trace!("Rearrange: {:?}", land);

    if land.len() > 1 {
        let max_idx = land.iter()
            .enumerate()
            .max_by(|(_i, a), (_j, b)| {
                use float_cmp::*;
                use std::cmp::Ordering;

                if approx_eq!(f32, a.current_height(), b.current_height(), F32Margin::default()) {
                    Ordering::Equal
                } else {
                    if a.current_height() > b.current_height() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            })
            .map(|(idx, _)| idx);

        let max_idx = match max_idx {
            None => return, // no maximum, we're ready
            Some(m) => m,
        };
        log::trace!("Maximum at index {}", max_idx);

        {
            let mut idx = max_idx as usize;
            while idx >= 0 {
                if idx == 0 {
                    // no left element
                    land[idx].increase_height(0.5);
                    break;
                }

                match land.get(idx - 1) {
                    None => {
                        // no element on my left, I am the last element
                        log::trace!("No element on the left of {}, increasing at {}", idx, idx);
                        land[idx].increase_height(0.5);
                        break;
                    }

                    Some(one_to_left) => if one_to_left.current_height() > land[idx].current_height() {
                        // left to me is higher than I am, water stays with me
                        log::trace!("Element on the left of {} is higher, increasing {}", idx, idx);
                        land[idx].increase_height(0.5);
                    } else {
                        log::trace!("Element on the left of {} is lower, continue", idx);
                        // continue iterating
                    }
                }

                if idx == 0 {
                    break;
                } else {
                    idx -= 1;
                }
            }
        }

        // TODO: right side

        let land_len = land.len();
        rearrange(&mut land[0..max_idx]);
        rearrange(&mut land[max_idx..(land_len - 1)]);
    }
}

#[cfg(test)]
mod tests {
    use super::LandscapeElement as LE;
    use super::rearrange;

    #[test]
    fn test_one_element() {
        let _ = env_logger::try_init();
        let mut land: Vec<LE> = vec![LE::new(1)];
        for elem in land.iter_mut() {
            elem.rain_one_hour();
        }

        rearrange(&mut land);

        float_cmp::assert_approx_eq!(f32, land[0].water_level, 1.0, ulps = 2);
    }

    #[test]
    fn test_two_eq_elements() {
        let _ = env_logger::try_init();
        let mut land: Vec<LE> = vec![LE::new(1), LE::new(1)];
        for elem in land.iter_mut() {
            elem.rain_one_hour();
        }

        rearrange(&mut land);

        float_cmp::assert_approx_eq!(f32, land[0].water_level, 1.0, ulps = 2);
        float_cmp::assert_approx_eq!(f32, land[1].water_level, 1.0, ulps = 2);
    }

    #[test]
    fn test_three_eq_elements() {
        let _ = env_logger::try_init();
        let mut land: Vec<LE> = vec![LE::new(1), LE::new(1), LE::new(1)];
        for elem in land.iter_mut() {
            elem.rain_one_hour();
        }

        rearrange(&mut land);

        float_cmp::assert_approx_eq!(f32, land[0].water_level, 1.0, ulps = 2);
        float_cmp::assert_approx_eq!(f32, land[1].water_level, 1.0, ulps = 2);
        float_cmp::assert_approx_eq!(f32, land[2].water_level, 1.0, ulps = 2);
    }

    #[test]
    fn test_three_elements_unequal() {
        let _ = env_logger::try_init();
        let mut land: Vec<LE> = vec![LE::new(1), LE::new(2), LE::new(1)];
        for elem in land.iter_mut() {
            elem.rain_one_hour();
        }

        rearrange(&mut land);

        float_cmp::assert_approx_eq!(f32, land[0].water_level, 2.33, ulps = 2);
        float_cmp::assert_approx_eq!(f32, land[1].water_level, 2.33, ulps = 2);
        float_cmp::assert_approx_eq!(f32, land[2].water_level, 2.33, ulps = 2);
    }
}
