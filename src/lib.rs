
pub fn e_char(plain_alphabet: &str, cipher_alphabet: &str, letter: char ) -> Option<char> {
    let mut result = None;
    if let Some(index) = plain_alphabet.find(letter) {
        result = cipher_alphabet.chars().nth(index);
    }
    result
}

pub fn d_char(plain_alphabet: &str, cipher_alphabet: &str, letter: char) -> Option<char> {
    e_char(cipher_alphabet, plain_alphabet, letter)
}

pub fn e_string<'a>(plain_alphabet: &str, cipher_alphabet: &str, sentence: &'a str) -> Option<String> {
    let slice = sentence.chars()
        .filter_map(|a| e_char(plain_alphabet, cipher_alphabet, a)).collect::<String>();
    Some(slice)
}

pub fn d_string<'a>(plain_alphabet: &str, cipher_alphabet: &str, sentence: &'a str) -> Option<String> {
    let slice = sentence.chars()
        .filter_map(|a| d_char(plain_alphabet, cipher_alphabet, a)).collect::<String>();
    Some(slice)
}
