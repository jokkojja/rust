pub fn choose_even(vector: Vec<i16>) -> Vec<i16> {
    let mut even_numbers: Vec<i16> = vec![];

    for number in vector {
        if number % 2 == 0 {
            even_numbers.push(number);
        }
    }

    even_numbers
}
