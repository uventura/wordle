use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub win_status: bool
}

#[function_component(WinScreen)]
pub fn win_screen(props: &Props) -> Html {

    html! {
    if props.win_status {
        <div class="win-container">
            <h1>{"Parabéns!"}</h1>
            <p>{"Você venceu o Wordle!"}</p>
            <p><a href="index.html" class="button">{"Jogar novamente"}</a></p>
        </div>
    } else {
        <div class="win-container">
            <h1>{"Infelizmente nao foi desta vez!!"}</h1>
            <p>{"Falhou miseravelmente"}</p>
            <div class="tentativas">
                <div class="tentativa perdeu"></div>
                <div class="tentativa perdeu"></div>
                <div class="tentativa perdeu"></div>
                <div class="tentativa perdeu"></div>
                <div class="tentativa perdeu"></div>
                <div class="tentativa perdeu"></div>
            </div>
            <p><a href="index.html" class="button">{"Jogar novamente"}</a></p>
        </div>
    }
    }
}