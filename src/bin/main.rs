fn main() {
    let plain = String::from("abcdefghijklmnopqrstuvxyz");
    let cipher = String::from("xyzabcdefghijklmnopqrstuv");
    
    if let Some(character) = monoalphabetic_cipher::e_char(&plain, &cipher, 'a') {
        println!("{}", character);
    };

    if let Some(character) = monoalphabetic_cipher::d_char(&plain, &cipher, 'b') {
        println!("{}", character);
    };

    if let Some(ciphertext) = monoalphabetic_cipher::e_string(&plain, &cipher, &String::from("holamellamojuan")) {
        println!("{}", ciphertext);

        if let Some(plaintext) = monoalphabetic_cipher::d_string(&plain, &cipher, &ciphertext) {
            println!("{}", plaintext);
        };
    };
}


