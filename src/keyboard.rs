use yew::prelude::*;

/// Propriedades para o componente `Keyboard` (`onclick`, `send_data` e `erase`)
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// emite um callback com o valor da tecla pressionada
    pub onclick: Callback<char>,

    /// emite um callback que envia a linha para o verificador, para saber se ela está correta
    pub send_data: Callback<MouseEvent>,

    /// apaga a última letra escrita
    pub erase: Callback<MouseEvent>,
}

/// Exibe o teclado e lida com o callback de cada tecla
#[function_component(Keyboard)]
pub fn keyboard(props: &Props) -> Html {
    let Props { onclick, send_data, erase } = props;
    
    html! {
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
    }
}