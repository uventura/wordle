use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct KeyProps {
    letter: char,
}

#[function_component(KeyboardButton)]
fn keyboard_button(props: &KeyProps) -> Html {
    let KeyProps { letter } = props;

    let letter: UseStateHandle<char> = use_state(|| char::default());
    let onclick = {
        let letter = letter.clone();
        move |_| {
            let value = 'b';
            letter.set(value);
        }
    };

    html! {
        <button {onclick} class="key-base key">
            {*letter}
        </button>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct KeyboardProps {
    // pub onkeypress: Callback<[char;5]>
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Keyboard)]
pub fn keyboard(props: &KeyboardProps) -> Html {
    let KeyboardProps { onclick } = props;
    
    html! {
        <div class="keyboard">
            <KeyboardButton letter='Q' />
            <button {onclick} class="key-base key">{"W"}</button>
            <button class="key-base key">{"E"}</button>
            <button class="key-base key">{"R"}</button>
            <button class="key-base key">{"T"}</button>
            <button class="key-base key">{"Y"}</button>
            <button class="key-base key">{"U"}</button>
            <button class="key-base key">{"I"}</button>
            <button class="key-base key">{"O"}</button>
            <button class="key-base key">{"P"}</button>
            <button class="key-base key">{"A"}</button>
            <button class="key-base key">{"S"}</button>
            <button class="key-base key">{"D"}</button>
            <button class="key-base key">{"F"}</button>
            <button class="key-base key">{"G"}</button>
            <button class="key-base key">{"H"}</button>
            <button class="key-base key">{"J"}</button>
            <button class="key-base key">{"K"}</button>
            <button class="key-base key">{"L"}</button>
            <button class="key-base key">{"Z"}</button>
            <button class="key-base key-enter">{"Enter"}</button>
            <button class="key-base key">{"C"}</button>
            <button class="key-base key">{"V"}</button>
            <button class="key-base key">{"B"}</button>
            <button class="key-base key">{"N"}</button>
            <button class="key-base key">{"M"}</button>
            <button class="key-base key">{"X"}</button>
            <button class="key-base key-enter">{"Rem"}</button>
        </div>
    }
}