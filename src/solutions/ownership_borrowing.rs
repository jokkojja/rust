pub fn longest_word(some_string: &str) -> &str {
    let splited_stirngs = some_string.split_whitespace();
    let mut longest_word: &str = "";

    for word in splited_stirngs {
        if word.len() > longest_word.len() {
            longest_word = word
        }
    }

    longest_word
}
