use crate::backend::landscape_element::*;

pub fn rearrange(land: &mut [LandscapeElement]) {
    use float_cmp::*;
    log::trace!("Rearrange: {:?}", land);

    if land.len() == 1 {
        return
    }

    let max_idx = find_land_max_idx(land);
    let max_idx = match max_idx {
        None => return, // no maximum, we're ready
        Some(m) => m,
    };
    log::trace!("Maximum at index {}", max_idx);

    // if all elements are of equal height as the current maximum, return
    if land.iter().all(|elem| approx_eq!(f32, elem.current_height(), land[max_idx].current_height(), F32Margin::default())) {
        return;
    }

    let has_left_neighbor = max_idx != 0;
    let has_right_neighbor = max_idx != (land.len() - 1);

    match (has_left_neighbor, has_right_neighbor) {
        (true, false) => {
            log::trace!("Have left neighbor");
            if left_neighbor_is_lower(land, max_idx) {
                log::trace!("Have left neighbor that is lower");
                land[max_idx - 1].increase_height(1.0);
                land[max_idx].decrease_height(1.0);

                rearrange(&mut land[0..max_idx]);
            }
        },
        (false, true) => {
            log::trace!("Have right neighbor");
            if right_neighbor_is_lower(land, max_idx) {
                log::trace!("Have right neighbor that is lower");
                land[max_idx + 1].increase_height(1.0);
                land[max_idx].decrease_height(1.0);

                let land_max = land.len() - 1;
                rearrange(&mut land[max_idx..land_max]);
           }
        },
        (true, true) => {
            log::trace!("Have both neighbors");
            let l_lower = left_neighbor_is_lower(land, max_idx);
            let r_lower = right_neighbor_is_lower(land, max_idx);
            match (l_lower, r_lower) {
                (true, true) => {
                    log::trace!("both neighbors lower");
                    land[max_idx - 1].increase_height(0.5);
                    land[max_idx + 1].increase_height(0.5);
                    land[max_idx].decrease_height(1.0);

                    let land_max = land.len() - 1;
                    rearrange(&mut land[0..max_idx]);
                    rearrange(&mut land[max_idx..land_max]);
                },
                (false, true) => {
                    log::trace!("right neighbor lower");
                    land[max_idx + 1].increase_height(1.0);
                    land[max_idx].decrease_height(1.0);

                    let land_max = land.len() - 1;
                    rearrange(&mut land[max_idx..land_max]);
                },
                (true, false) => {
                    log::trace!("left neighbor lower");
                    land[max_idx - 1].increase_height(1.0);
                    land[max_idx].decrease_height(1.0);

                    rearrange(&mut land[0..max_idx]);
                },
                (false, false) => {},
            }
        },
        (false, false) => {},
    };


    let land_max = land.len() - 1;
    rearrange(&mut land[0..max_idx]);
    rearrange(&mut land[max_idx..land_max]);
}

fn find_land_max_idx(land: &[LandscapeElement]) -> Option<usize> {
    land.iter()
        .enumerate()
        .max_by(|(_i, a), (_j, b)| {
            use std::cmp::Ordering;

            if float_cmp::approx_eq!(f32, a.current_height(), b.current_height(), float_cmp::F32Margin::default()) {
                Ordering::Equal
            } else {
                if a.current_height() > b.current_height() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        })
        .map(|(idx, _)| idx)
}

fn left_neighbor_is_lower(land: &[LandscapeElement], idx: usize) -> bool {
    if idx == 0 {
        false
    } else {
        land[idx].current_height() > land[idx - 1].current_height()
    }
}

fn right_neighbor_is_lower(land: &[LandscapeElement], idx: usize) -> bool {
    if idx == (land.len() - 1) {
        false
    } else {
        land[idx].current_height() > land[idx + 1].current_height()
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::landscape_element::LandscapeElement as LE;
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

    #[test]
    fn test_four_elements() {
        let _ = env_logger::try_init();
        let mut land: Vec<LE> = vec![LE::new(1), LE::new(2), LE::new(3), LE::new(4)];
        for elem in land.iter_mut() {
            elem.rain_one_hour();
        }

        rearrange(&mut land);

        float_cmp::assert_approx_eq!(f32, land[0].water_level, 3.33, ulps = 2);
        float_cmp::assert_approx_eq!(f32, land[1].water_level, 3.33, ulps = 2);
        float_cmp::assert_approx_eq!(f32, land[2].water_level, 3.33, ulps = 2);
        float_cmp::assert_approx_eq!(f32, land[0].water_level, 0.00, ulps = 2);
    }
}
