use std::{ops::DerefMut, result};

use yew::prelude::*;
use gloo::console::log;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod service;
mod win;
mod keyboard;
mod line;

use service::*;
use win::*;
use keyboard::Keyboard;
use line::Line;

fn init_word() -> [char; 5] {
    return [char::default(),char::default(),char::default(),char::default(),char::default()];   
}

fn init_answers() -> [i8; 5] {
    return [-1,-1,-1,-1,-1];
}

/// Cria a palavra secreta como uma variável global
lazy_static! {
    static ref SECRET: String = service::palavra_aleatoria().to_ascii_uppercase();
}

/// Inicializa os `useStates`, define os `callbacks` e renderiza o app
#[function_component]
fn App() -> Html {
    log!(serde_json::to_string_pretty(&*SECRET).unwrap());

    let answers = use_state(|| [init_answers(),init_answers(),init_answers(),init_answers(),init_answers(),init_answers()]);
    let words = use_state(|| [init_word(), init_word(),init_word(),init_word() ,init_word(),init_word()]);

    let win_status = use_state(|| false);

    let game_is_running = use_state(|| true);

    // Matrix indexes
    let line = use_state(|| 0);
    let col = use_state(|| 0);

    let clonned_line = line.clone();
    let clonned_col = col.clone();
    let clonned_words = words.clone();
    let onclick = Callback::from(move |letter: char| {
        if *clonned_col > 4 {
            return;
        }
        let mut new_word = *clonned_words;
        new_word[*clonned_line][*clonned_col] = letter;

        // log!(*line);
        let next_col = clonned_col.clone();
        clonned_col.set(*next_col+1);

        // log!(serde_json::to_string_pretty(&new_word).unwrap());
        clonned_words.set(new_word);
    });
    
    let clonned_line: UseStateHandle<usize> = line.clone();
    let clonned_col = col.clone();
    let clonned_words = words.clone();
    let clonned_answers = answers.clone();
    let clonned_win = win_status.clone();
    let clonned_game_is_running = game_is_running.clone();
    let send_data = Callback::from(move |_| {
        if *clonned_col < 4 {
            return;
        }
        
        let join_word: String = clonned_words[*clonned_line].iter().collect();
        log!(&*join_word);
        let mut result = service::validate_string(join_word, SECRET.to_owned());

        if result[0] == -1 {
            return;
        }

        let mut new_word = *clonned_answers;
        new_word[*clonned_line] = result;
        clonned_answers.set(new_word);

        let sum: i8 = result.iter().sum();
        if sum == 10 {
            clonned_win.set(true);
            clonned_game_is_running.set(false);
            return;
        }

        if *clonned_line >= 5 {
            clonned_game_is_running.set(false);
            clonned_win.set(false);
            return;
        }
        clonned_col.set(0);

        let new_line =  clonned_line.clone();
        clonned_line.set(*new_line+1);
    });


    let clonned_line = line.clone();
    let clonned_col = col.clone();
    let clonned_words = words.clone();
    let erase = Callback::from(move |_| {
        if *clonned_col == 0 {
            return;
        }
        let mut new_word = *clonned_words;
        new_word[*clonned_line][*clonned_col-1] = '\0';
        clonned_words.set(new_word);
        
        let new_col = clonned_col.clone();
        clonned_col.set(*new_col-1);
    });

    html! {
    if *game_is_running {
        <div class="container">
            <h1 class="title">{"Wordle"}</h1>

            <Line answers={answers[0]} words={words[0]} />
            <Line answers={answers[1]} words={words[1]} />
            <Line answers={answers[2]} words={words[2]} />
            <Line answers={answers[3]} words={words[3]} />
            <Line answers={answers[4]} words={words[4]} />
            <Line answers={answers[5]} words={words[5]} />
            
            <Keyboard onclick={onclick} send_data={send_data} erase={erase} />
        </div>
    } else {
        <win::WinScreen win_status={*win_status} />
    }
    }
}

fn main() {
    // let SECRET = service::palavra_aleatoria();
    yew::Renderer::<App>::new().render();
}
