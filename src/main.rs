use yew::prelude::*;

mod game;
mod wordbox;
mod keyboard;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{"Wordle"}</h1>
            <game::GameScreen/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
