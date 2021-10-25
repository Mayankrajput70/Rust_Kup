/// number function check the number is "Even".
///
/// #Arguments
///
/// value : value is a integer type input number whose use to check even or not.
///
/// #Return
///
/// Result enum which is used to handle error and value.
pub fn number(value: i32) -> Result<String, String> {
    if value % 2 == 0 {
        Ok("Even".to_string())
    } else {
        Err("Not even".to_string())
    }
}
/// number_test function check the value object.
///
/// #Arguments
///
/// result : result is a integer type input number whose use to check even.
///
/// #Return
///
/// return String type which handle error.
pub fn number_test(value: i32) -> String {
    let result = number(value);

    match result {
        Ok(result) => result,

        Err(_) => "Invalid".to_string(),
    }
}