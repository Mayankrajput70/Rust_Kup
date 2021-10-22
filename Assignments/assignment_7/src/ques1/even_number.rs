/// number function check the number is "Even and Odd".
///
/// #Arguments
///
/// value : value object check the condition.
///
/// #Return
///
/// return : return handle error and use value.
pub fn number(value: i32) -> Result<String, String> {
    if value % 2 == 0 {
        Ok("Even".to_string())
    } else {
        Err("Not ever".to_string())
    }
}
/// number_test function check the value object.
///
/// #Arguments
///
/// result : result control the number-value
///
/// #Return
///
/// return : return handle error and give output String.
pub fn number_test(value: i32) -> String {
    let result = number(value);

    match result {
        Ok(result) => result,

        Err(_) => "Error".to_string(),
    }
}
