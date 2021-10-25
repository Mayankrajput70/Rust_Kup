#[cfg(test)]
pub mod test {
    use crate::ques1::even_number::number_test;

    #[test]
    pub fn number_test_Success() {
        assert_eq!(number_test(4), "Even");
    }

    #[test]
   pub fn number_fail() {
        assert_ne!(number_test(0), "Odd");
    }
    #[test]
    pub fn number_test_fail_() {

        assert_ne!(number_test(5), "Even");
    }

    #[test]
    pub fn number_test_fail__() {
        assert_ne!(number_test(-33), "Even");
    }
}
