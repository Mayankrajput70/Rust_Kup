#[cfg(test)]
pub mod test {
    use crate::ques1::even_number::number_test;

    #[test]
    fn check_even_success_first() {
        assert_eq!(number_test(98), "Even");
    }
    #[test]
    fn check_even_success_second() {
        assert_eq!(number_test(-14), "Even");
    }
    #[test]
    fn check_even_failure_first() {
        assert_eq!(number_test(3), "Invalid");
    }
    #[test]
    fn check_even_failure_second() {
        assert_eq!(number_test(7), "Invalid");
    }
    #[test]
    fn check_even_success_third() {
        assert_eq!(number_test(0), "Even");
    }
    #[test]
    fn check_even_failure_third() {
        assert_eq!(number_test(-25), "Invalid");
    }
}
