pub fn change_values(array: &mut Vec<i16>) -> &mut Vec<i16> {
    for i in 0..array.len() {
        array[i] *= 2
    }

    array
}
