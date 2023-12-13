use gloo::console::log;
use std::fs::File;
use std::io::{self, BufRead};
use rand::seq::{SliceRandom, IteratorRandom};
use rand::thread_rng;

pub fn validate_string(word: String, secret: String) -> [i8;5] {
    let mut validated_word: [i8;5] = [-1,-1,-1,-1,-1];

    if !verificar_palavra_existe(word.to_ascii_lowercase().clone()) {
        return  validated_word;
    }

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

fn verificar_palavra_existe(palavra: String) -> bool {
    let palavras = ler_palavras_do_arquivo("./novaspalavras.txt");
            // Faça algo com as palavras, por exemplo, imprimir
        if palavra_existe(&palavra, &palavras) {
            return true;
        }
        return false;
}

pub fn palavra_aleatoria() -> String {
    let file_path = "novaspalavras.txt";
    
    // let file = match File::open(file_path) {
    //     Ok(file) => file,
    //     Err(e) => return format!("Erro ao abrir o arquivo: {}", e).to_string(),
    // };
    let file = include_str!("novaspalavras.txt");

    let words = file.split('\n');

    // log!(file.clone());
    // let reader = io::BufReader::new(file);
    // let mut words: Vec<String> = Vec::new();

    // for line in reader.lines() {
    //     match line {
    //         Ok(line) => words.extend(line.split_whitespace().map(String::from)),
    //         Err(_) => continue, // Ignora linhas que não podem ser lidas
    //     }
    // }

    match words.choose(&mut thread_rng()) {
        Some(word) => word.to_string(),
        None => "Nenhuma palavra encontrada".to_string(),
    }
}

fn ler_palavras_do_arquivo(file_path: &str) -> io::Result<Vec<String>> {
    // let file = File::open(file_path)?;
    // let reader = io::BufReader::new(file);
    let file_path = "novaspalavras.txt";
    let file = include_str!("novaspalavras.txt");

    let words = file.split_whitespace().map(str::to_string).collect();
    // let mut words = Vec::new();
    // for line in reader.lines() {
    //     let line = line?;
    //     words.push(line);
    // }

    Ok(words)
    // words
}


fn palavra_existe(palavra: &str, palavras: &Result<Vec<String>, io::Error>) -> bool {
    palavras.as_ref().unwrap().contains(&palavra.to_string())
}