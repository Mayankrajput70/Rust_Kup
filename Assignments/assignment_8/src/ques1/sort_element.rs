/// elements function given array of generic type.
///
/// #Argument
///
/// list - A vector of generic type.
///
/// #Return
///
/// return which is a generic type list sorted.
pub fn elements<T: PartialOrd + Copy>(list: &mut [T]) -> &[T] {
    let length = list.len();
    for before in 0..length {
        for after in before + 1..length {
            if list[before] > list[after] {
                let value;
                value = list[before];
                list[before] = list[after];
                list[after] = value;
            }
        }
    }
    list
}
