mod landscape;
mod rain;
mod sim;

pub use landscape::Landscape;

#[cfg(test)]
mod tests {
    use super::landscape::*;
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
