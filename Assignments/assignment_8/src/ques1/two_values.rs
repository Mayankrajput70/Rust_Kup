/// values function gives the two number.
///
/// #Argument
///
/// number_1 and number_2 both are generic type.
///
/// #Return
///
/// returns the min number between the two number.
pub fn values<N: PartialOrd>(number_1: N, number_2: N) -> Result<N, String> {
    if number_1 < number_2 {
        Ok(number_1)
    } else if number_1 == number_2 {
        Err("Invalid number".to_string())
    } else {
        Ok(number_2)
    }
}
