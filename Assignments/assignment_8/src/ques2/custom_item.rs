/// trait define Iterator.
pub trait Iterator {
    fn geometric_series(&mut self, size: i32) -> Result<Vec<i32>, String>;
}
/// struct gives variants type<i32>.
pub struct GeometricSeries {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}
/// geometric_series function used return next ( n = size)  number in GP Series.
///
/// #Argument
///
/// &mut Referenced mutable variable of type Self and size is a i32 integer type.
///
/// #Return
///
/// returns result enum which contains next n number in GP series.
impl Iterator for GeometricSeries {
    fn geometric_series(&mut self, size: i32) -> Result<Vec<i32>, String> {
        if size == 0 {
            return Err("Invalid input".to_string());
        }
        let mut process: Vec<i32> = Vec::new();
        for _index in 0..size {
            process.push(self.current_number);
            self.current_number *= self.ratio;
        }
        Ok(process)
    }
}
