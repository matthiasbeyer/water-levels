
/// Helper type for iterating with &mut access to over a slice with a window of size 3
///
/// This is not very generic, but enough for us here.  This is also not a real iterator, because a
/// real iterator wouldn't be possible due to lifetime overlap in the next() implementation.
///
pub struct ThreeElemWindowMut<'a, T> {
    slice: &'a mut [T],
    lower_bound: usize,
}


impl<'a, T> ThreeElemWindowMut<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        ThreeElemWindowMut { slice, lower_bound: 0 }
    }

    pub fn next(&mut self) -> Option<&mut [T]> {
        if self.slice.len() > (self.lower_bound + 3 - 1) {
            let this_lower_bound = self.lower_bound;
            self.lower_bound += 1; // for next iteration
            Some(&mut self.slice[this_lower_bound..(this_lower_bound + 3)])
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut v = vec![] as Vec<()>;
        let mut windows = ThreeElemWindowMut::new(&mut v);

        // can't use `for`; not an iterator
        while let Some(_) = windows.next() {
            assert!(false, "This cannot happen as slice is too short");
        }
    }

    #[test]
    fn test_one_element() {
        let mut v = vec![()];
        let mut windows = ThreeElemWindowMut::new(&mut v);

        while let Some(_) = windows.next() {
            assert!(false, "This cannot happen as slice is too short");
        }
    }

    #[test]
    fn test_two_elements() {
        let mut v = vec![(), ()];
        let mut windows = ThreeElemWindowMut::new(&mut v);

        while let Some(_) = windows.next() {
            assert!(false, "This cannot happen as slice is too short");
        }
    }

    #[test]
    fn test_three_elements() {
        let mut v = vec![(), (), ()];
        let mut windows = ThreeElemWindowMut::new(&mut v);

        let mut cnt = 0;
        while let Some(_) = windows.next() {
            assert_eq!(cnt, 0, "Iterated more than once, cannot happen as we only have three elements");
            cnt += 1;
        }
    }

    #[test]
    fn test_four_elements() {
        let mut v = vec![(), (), (), ()];
        let mut windows = ThreeElemWindowMut::new(&mut v);

        let mut cnt = 0;
        while let Some(_) = windows.next() {
            assert!(cnt <= 1, "Iterated more than twice, cannot happen as we only have four elements");
            cnt += 1;
        }
    }

    quickcheck::quickcheck! {
        fn test_n_elements(v: Vec<()>) -> bool {
            let mut v = v; // rebind
            if v.len() <= 4 { // already tested this above, no need to invest time to "do it right" here
                return true;
            }

            // we always expect n - 2 windows if window size is 3
            let number_of_expected_windows = v.len() - 2;
            let mut windows = ThreeElemWindowMut::new(&mut v);

            let mut cnt = 0;
            while let Some(_) = windows.next() {
                if !(cnt <= number_of_expected_windows) {
                    eprintln!("Iterated more than {} times, cannot happen as we have {} elements", number_of_expected_windows, v.len());
                    return false
                }
                cnt += 1;
            }

            true
        }
    }

    #[test]
    fn test_mutable_access() {
        let mut v = vec![1, 1, 1, 1];
        let mut windows = ThreeElemWindowMut::new(&mut v);

        while let Some(window) = windows.next() {
            for elem in window.iter_mut() {
                *elem += 1;
            }
        }

        assert_eq!(v, &[2, 3, 3, 2]);
    }

    #[test]
    fn test_mutable_access_2() {
        let mut v = vec![1, 1, 1, 1, 1];
        let mut windows = ThreeElemWindowMut::new(&mut v);

        while let Some(window) = windows.next() {
            for elem in window.iter_mut() {
                *elem += 1;
            }
        }

        assert_eq!(v, &[2, 3, 4, 3, 2]);
    }
}

