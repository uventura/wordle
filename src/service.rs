use gloo::console::log;

pub fn validate_string(word: String, secret: String) -> [i8;5] {
    let mut validated_word: [i8;5] = [0,0,0,0,0];

    for i in 0..5 {
        // num_string.chars().nth(i).unwrap()
        let word_letter: u8 = word.as_bytes()[i];
        let word_letter: char = word_letter as char;
        
        let secret_letter: u8 = secret.as_bytes()[i];
        let secret_letter: char = secret_letter as char;
        // log!(word.clone(),secret.clone());
        if word_letter == secret_letter {
            validated_word[i] = 2;
            continue;
        }
        else if secret.contains(word_letter) {
            validated_word[i] = 1;
            continue;
        }
        validated_word[i] = 0;
    }
    return validated_word;
}
