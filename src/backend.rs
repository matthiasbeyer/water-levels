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

    fn decrease_height(&mut self, h: f32) {
        self.water_level -= h;
    }

    // subtract enough from own water level to balance with other, returning what needs to be added
    // to other
    fn balance(&mut self, other_height: f32) -> f32 {
        if other_height - self.current_height() > 1.0 {
            self.water_level -= 1.0;
            1.0
        } else {
            let diff = (other_height - self.current_height()) / 2.0;
            self.water_level -= diff;
            diff
        }
    }
}

fn rearrange(land: &mut [LandscapeElement]) {
    use float_cmp::*;
    log::trace!("Rearrange: {:?}", land);

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
    log::trace!("maximum at index {}", max_idx);

    if land.iter().all(|elem| approx_eq!(f32, elem.current_height(), land[max_idx].current_height(), F32Margin::default())) {
        log::trace!("All elements equal high, returning");
        return;
    }

    let land_len = land.len();

    let has_left_neighbor = max_idx != 0;
    let has_right_neighbor = max_idx != land_len - 1;

    match (has_left_neighbor, has_right_neighbor) {
        (true, true) => {
            log::trace!("Having both neighbors to recalculate");
            recalc_left(max_idx, land, 0.5);
            recalc_right(max_idx, land, 0.5);
            land[max_idx].decrease_height(1.0);
        },

        (false, true) => {
            log::trace!("Having right neighbor to recalculate");
            recalc_right(max_idx, land, 1.0);
            land[max_idx].decrease_height(1.0);
        },

        (true, false) => {
            log::trace!("Having left neighbor to recalculate");
            recalc_left(max_idx, land, 1.0);
            land[max_idx].decrease_height(1.0);
        },

        (false, false) => {
            // nothing to do
            log::trace!("No neighbor to recalculate");
            return;
        },
    }
    log::trace!("After rearrange step: {:?}", land);

    let land_len = land.len();
    rearrange(&mut land[0..max_idx]);
    rearrange(&mut land[max_idx..land_len]);
}

fn recalc_left(max_idx: usize, land: &mut [LandscapeElement], inc: f32) {
    let mut idx = max_idx;
    let mut iter_height = land[idx].current_height();
    loop {
        if idx == 0 {
            break;
        }

        if land[idx - 1].current_height() < iter_height {
            iter_height = land[idx - 1].current_height();
            idx -= 1;
        } else {
            break;
        }
    }

    land[idx].increase_height(inc);
}

fn recalc_right(max_idx: usize, land: &mut [LandscapeElement], inc: f32) {
    let mut idx = max_idx;
    let mut iter_height = land[idx].current_height();
    loop {
        if idx == land.len() - 1 {
            break;
        }

        if land[idx + 1].current_height() < iter_height {
            iter_height = land[idx + 1].current_height();
            idx += 1;
        } else {
            break;
        }
    }

    land[idx].increase_height(inc);
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
