use yew::prelude::*;
use gloo::console::log;

fn init_word() -> [char; 5] {
    return [char::default(),char::default(),char::default(),char::default(),char::default()];   
}

#[function_component]
fn App() -> Html {
    // let word: [char;5] = [char::default(),char::default(),char::default(),char::default(),char::default()];
    let word = use_state(|| init_word());
    let words = use_state(|| [init_word(), init_word(),init_word(),init_word() ,init_word(),init_word()]);

    let line = use_state(|| 0);
    let col = use_state(|| 0);

    let clonned_lie = line.clone();
    let clonned_words = words.clone();
    let onclick = Callback::from(move |letter: char| {
            // let mut word2 = [char::default(),char::default(),char::default(),char::default(),char::default()];
            if *col > 4 {
                return;
            }
            let mut new_word = *clonned_words;
            new_word[*clonned_lie][*col] = letter;

            // let new_line =  line.clone();
            // line.set(*new_line+1);
            // log!(*line);
            let next_col = col.clone();
            col.set(*next_col+1);

            // log!(serde_json::to_string_pretty(&new_word).unwrap());
            clonned_words.set(new_word);
        });

    html! {
        <div class="container">
            <h1 class="title">{"Wordle"}</h1>
            <div class="line">
                <div class="item item-wrong">{words[0][0]}</div>
                <div class="item item-wrong-position">{words[0][1]}</div>
                <div class="item item-correct">{words[0][2]}</div>
                <div class="item item-correct">{words[0][3]}</div>
                <div class="item item-wrong">{words[0][4]}</div>
            </div>
            <div class="line">
                <div class="item">{words[1][0]}</div>
                <div class="item">{words[1][1]}</div>
                <div class="item">{words[1][2]}</div>
                <div class="item">{words[1][3]}</div>
                <div class="item">{words[1][4]}</div>
            </div>
            <div class="line">
                <div class="item">{words[2][0]}</div>
                <div class="item">{words[2][1]}</div>
                <div class="item">{words[2][2]}</div>
                <div class="item">{words[2][3]}</div>
                <div class="item">{words[2][4]}</div>
            </div>
            <div class="line">
                <div class="item">{words[3][0]}</div>
                <div class="item">{words[3][1]}</div>
                <div class="item">{words[3][2]}</div>
                <div class="item">{words[3][3]}</div>
                <div class="item">{words[3][4]}</div>
            </div>
            <div class="line">
                <div class="item">{words[4][0]}</div>
                <div class="item">{words[4][1]}</div>
                <div class="item">{words[4][2]}</div>
                <div class="item">{words[4][3]}</div>
                <div class="item">{words[4][4]}</div>
            </div>
            <div class="line">
                <div class="item">{words[5][0]}</div>
                <div class="item">{words[5][1]}</div>
                <div class="item">{words[5][2]}</div>
                <div class="item">{words[5][3]}</div>
                <div class="item">{words[5][4]}</div>
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
                <button class="key-base key-enter">{"Enter"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('C')} class="key-base key">{"C"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('V')} class="key-base key">{"V"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('B')} class="key-base key">{"B"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('N')} class="key-base key">{"N"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('M')} class="key-base key">{"M"}</button>
                <button onclick={let onclick = onclick.clone();move |_| onclick.emit('X')} class="key-base key">{"X"}</button>
                <button class="key-base key-enter">{"Rem"}</button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
