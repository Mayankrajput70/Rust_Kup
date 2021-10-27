#[cfg(test)]
pub mod test {
    use crate::ques2::custom_item::{GeometricSeries, Iterator};

    #[test]
    pub fn min_value_success() {
        use crate::ques1::two_values::values;
        assert_eq!(values(100, 101), Ok(100));
    }
    #[test]
    pub fn value_min_success_() {
        use crate::ques1::two_values::values;
        assert_eq!(values(50.5, 50.4), Ok(50.4));
    }
    #[test]
    pub fn value_min_success__() {
        use crate::ques1::two_values::values;
        assert_eq!(values('S', 'M'), Ok('M'));
    }
    #[test]
    pub fn element_sort_success() {
        use crate::ques1::sort_element::elements;
        let mut list = vec!['m', 'a', 'y', 'a', 'n', 'k'];
        assert_eq!(vec!['a', 'a', 'k', 'm', 'n', 'y'], elements(&mut list));
    }
    #[test]
    pub fn element_sort_success_() {
        use crate::ques1::sort_element::elements;
        let mut list = vec![2, 5, 6, 3, 4, 6, 8];
        assert_eq!(vec![2, 3, 4, 5, 6, 6, 8], elements(&mut list));
    }
    #[test]
    pub fn element_sort_success__() {
        use crate::ques1::sort_element::elements;
        let mut list = vec![20.4, 5.5, 6.9, -6.54, 0.08];
        assert_eq!(vec![-6.54, 0.08, 5.5, 6.9, 20.4], elements(&mut list));
    }
    #[test]
    fn iterator_failure() {
        let mut geometric_progression = GeometricSeries {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        assert_eq!(
            geometric_progression.geometric_series(0),
            Err("Invalid input".to_string())
        );
    }
     #[test]
    fn iterator_success__() {
        let mut geometric_progression = GeometricSeries {
            first_number: 1,
            current_number: 0,
            ratio: 1,
        };
        assert_eq!(
            geometric_progression.geometric_series(12).unwrap(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
    #[test]
    fn iterator_success() {
        let mut geometric_progression = GeometricSeries {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        assert_eq!(
            geometric_progression.geometric_series(0),
            Err("Invalid input".to_string())
        );
    }
}
