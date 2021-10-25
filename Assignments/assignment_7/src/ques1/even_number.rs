/// number function is used to check a given input number is even or not
///
/// #Arguments
///
/// value : value is a i32 integer type input number whose use to check even or not
///
/// #Return
///
/// return result enum which is used to handle error and value
pub fn number(value: i32) -> Result<String, String> {
    if value % 2 == 0 {
        Ok("Even".to_string())
    } else {
        Err("Not even".to_string())
    }
}
/// number_test function is used handle response of calling function.
///
/// #Arguments
///
/// value : value is a i32 integer type input number whose use to check even or not with handle error and value
///
/// #Return
///
/// return String which handle error and value of output of function.
pub fn number_test(value: i32) -> String {
    let result = number(value);

    match result {
        Ok(result) => result,

        Err(_) => "Invalid".to_string(),
    }
}
