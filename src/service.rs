use std::fs::File;
use std::io::{self, BufRead};
use rand::seq::SliceRandom;
use rand::thread_rng;



fn validate_string(word: String, secret: String) -> [i8;5] {
    let mut validated_word: [i8;5] = [0,0,0,0,0];

    for i in 0..5 {
        // num_string.chars().nth(i).unwrap()
        let word_letter: u8 = word.as_bytes()[i];
        let word_letter: char = word_letter as char;
        
        let secret_letter: u8 = secret.as_bytes()[i];
        let secret_letter: char = secret_letter as char;

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
pub fn palavra_aleatoria() -> String {
    let file_path = "C:\\Users\\Eduli\\novaspalavras.txt";
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return "Erro ao abrir o arquivo".to_string(),
    };

    let reader = io::BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => words.extend(line.split_whitespace().map(String::from)),
            Err(_) => continue, // Ignora linhas que não podem ser lidas
        }
    }

    match words.choose(&mut thread_rng()) {
        Some(word) => word.to_string(),
        None => "Nenhuma palavra encontrada".to_string(),
    }
}
