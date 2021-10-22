#[cfg(test)]
pub mod test {
    use crate::ques1::even_number::number_test;

    #[test]
    pub fn test1() {
        assert_eq!(number_test(4), "Even");
    }

    #[test]
   pub fn test2() {
        assert_ne!(number_test(0), "Odd");
    }
    #[test]
    pub fn test3() {

        assert_ne!(number_test(5), "Even");
    }

    #[test]
    pub fn test4() {
        assert_ne!(number_test(-33), "Even");
    }
}
