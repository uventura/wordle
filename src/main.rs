use std::ops::DerefMut;

use yew::prelude::*;
use gloo::console::log;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod service;

use service::*;

fn init_word() -> [char; 5] {
    return [char::default(),char::default(),char::default(),char::default(),char::default()];   
}

fn init_answers() -> [i8; 5] {
    return [-1,-1,-1,-1,-1];
}

// global variable
lazy_static! {
    static ref SECRET: Mutex<String> = Mutex::new(service::palavra_aleatoria());
}

#[function_component]
fn App() -> Html {
    let secret = service::palavra_aleatoria();
    // log!(SECRET);
    log!(serde_json::to_string_pretty(&*SECRET).unwrap());

    let answers = use_state(|| [init_answers(),init_answers(),init_answers(),init_answers(),init_answers(),init_answers()]);

    // keyboard array
    let words = use_state(|| [init_word(), init_word(),init_word(),init_word() ,init_word(),init_word()]);

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
    let send_data = Callback::from(move |_| {
        if *clonned_col < 4 {
            return;
        }
        
        let join_word: String = clonned_words[*clonned_line].iter().collect();
        log!(&*join_word);
        let mut result = service::validate_string(join_word, secret.to_owned());

        let mut new_word = *clonned_answers;
        new_word[*clonned_line] = result;
        clonned_answers.set(new_word);

        clonned_col.set(0);

        if *clonned_line >= 5 {
            return;
        }
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
        <div class="container">
            <h1 class="title">{"Wordle"}</h1>
            <div class="line">
                <div class={"item item-".to_owned()+&answers[0][0].to_string()}>{words[0][0]}</div>
                <div class={"item item-".to_owned()+&answers[0][1].to_string()}>{words[0][1]}</div>
                <div class={"item item-".to_owned()+&answers[0][2].to_string()}>{words[0][2]}</div>
                <div class={"item item-".to_owned()+&answers[0][3].to_string()}>{words[0][3]}</div>
                <div class={"item item-".to_owned()+&answers[0][4].to_string()}>{words[0][4]}</div>
            </div>
            <div class="line">
                <div class={"item item-".to_owned()+&answers[1][0].to_string()}>{words[1][0]}</div>
                <div class={"item item-".to_owned()+&answers[1][1].to_string()}>{words[1][1]}</div>
                <div class={"item item-".to_owned()+&answers[1][2].to_string()}>{words[1][2]}</div>
                <div class={"item item-".to_owned()+&answers[1][3].to_string()}>{words[1][3]}</div>
                <div class={"item item-".to_owned()+&answers[1][4].to_string()}>{words[1][4]}</div>
            </div>
            <div class="line">
                <div class={"item item-".to_owned()+&answers[2][0].to_string()}>{words[2][0]}</div>
                <div class={"item item-".to_owned()+&answers[2][1].to_string()}>{words[2][1]}</div>
                <div class={"item item-".to_owned()+&answers[2][2].to_string()}>{words[2][2]}</div>
                <div class={"item item-".to_owned()+&answers[2][3].to_string()}>{words[2][3]}</div>
                <div class={"item item-".to_owned()+&answers[2][4].to_string()}>{words[2][4]}</div>
            </div>
            <div class="line">
                <div class={"item item-".to_owned()+&answers[3][0].to_string()}>{words[3][0]}</div>
                <div class={"item item-".to_owned()+&answers[3][1].to_string()}>{words[3][1]}</div>
                <div class={"item item-".to_owned()+&answers[3][2].to_string()}>{words[3][2]}</div>
                <div class={"item item-".to_owned()+&answers[3][3].to_string()}>{words[3][3]}</div>
                <div class={"item item-".to_owned()+&answers[3][4].to_string()}>{words[3][4]}</div>
            </div>
            <div class="line">
                <div class={"item item-".to_owned()+&answers[4][0].to_string()}>{words[4][0]}</div>
                <div class={"item item-".to_owned()+&answers[4][1].to_string()}>{words[4][1]}</div>
                <div class={"item item-".to_owned()+&answers[4][2].to_string()}>{words[4][2]}</div>
                <div class={"item item-".to_owned()+&answers[4][3].to_string()}>{words[4][3]}</div>
                <div class={"item item-".to_owned()+&answers[4][4].to_string()}>{words[4][4]}</div>
            </div>
            <div class="line">
                <div class={"item item-".to_owned()+&answers[5][0].to_string()}>{words[5][0]}</div>
                <div class={"item item-".to_owned()+&answers[5][1].to_string()}>{words[5][1]}</div>
                <div class={"item item-".to_owned()+&answers[5][2].to_string()}>{words[5][2]}</div>
                <div class={"item item-".to_owned()+&answers[5][3].to_string()}>{words[5][3]}</div>
                <div class={"item item-".to_owned()+&answers[5][4].to_string()}>{words[5][4]}</div>
            </div>
            <div class="keyboard">
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('Q')} class="key-base key">{"Q"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('W')} class="key-base key">{"W"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('E')} class="key-base key">{"E"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('R')} class="key-base key">{"R"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('T')} class="key-base key">{"T"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('Y')} class="key-base key">{"Y"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('U')} class="key-base key">{"U"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('I')} class="key-base key">{"I"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('O')} class="key-base key">{"O"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('P')} class="key-base key">{"P"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('A')} class="key-base key">{"A"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('S')} class="key-base key">{"S"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('D')} class="key-base key">{"D"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('F')} class="key-base key">{"F"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('G')} class="key-base key">{"G"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('H')} class="key-base key">{"H"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('J')} class="key-base key">{"J"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('K')} class="key-base key">{"K"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('L')} class="key-base key">{"L"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('Z')} class="key-base key">{"Z"}</button>
                <button onclick={send_data} class="key-base key-enter">{"Enter"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('C')} class="key-base key">{"C"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('V')} class="key-base key">{"V"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('B')} class="key-base key">{"B"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('N')} class="key-base key">{"N"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('M')} class="key-base key">{"M"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('X')} class="key-base key">{"X"}</button>
                <button onclick={erase} class="key-base key-enter">{"Rem"}</button>
            </div>
        </div>
    }
}

fn main() {
    // let SECRET = service::palavra_aleatoria();
    yew::Renderer::<App>::new().render();
}
