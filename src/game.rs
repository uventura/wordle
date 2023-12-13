use yew::prelude::*;
use crate::keyboard;
use crate::wordbox;
use gloo::console::log;

#[function_component(UseState)]
fn state() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };


    html! {
        <div>
            <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}

#[function_component]
pub fn GameScreen() -> Html {
    let word: [char;5] = [char::default(),char::default(),char::default(),char::default(),char::default()];
    let word = use_state(|| word);
    let clonned_word = word.clone();
    let onclick = Callback::from(move |_| {
            // let mut word2 = [char::default(),char::default(),char::default(),char::default(),char::default()];
            let mut new_word = *clonned_word;
            new_word[0] = 'a';
            clonned_word.set(new_word);
        });

        
    // let word = use_state(|| "Teste");
    // let onclick: Callback<MouseEvent> = {
    //     let word = word.clone();
    //     let temp_word = ['a','b','c'];
    //     log!(serde_json::to_string_pretty(&temp_word).unwrap());
    //     Callback::from(move |_| word.set(&"Teste2"))
    // };

    // let onother = Callback::from(move |text: String| log!(text));
    // let temp_word = ['a','b','c'];
    
    html! {
        <>  
            // <wordbox::WordBox/>
            <div>
            // <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { &word[0] }
            </p>
            </div>
            // <div>{word}</div>
            <wordbox::WordBox {word}/>
            <keyboard::Keyboard {onclick} />
        </>
    }
}