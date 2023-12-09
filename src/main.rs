use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{"Wordle"}</h1>
            <div class="line">
                <div class="item item-wrong">{"A"}</div>
                <div class="item item-wrong-position">{"B"}</div>
                <div class="item item-correct">{"C"}</div>
                <div class="item item-correct">{"D"}</div>
                <div class="item item-wrong">{"E"}</div>
            </div>
            <div class="line">
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
            </div>
            <div class="line">
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
            </div>
            <div class="line">
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
            </div>
            <div class="line">
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
            </div>
            <div class="line">
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
                <div class="item"></div>
            </div>
            <div class="keyboard">
                <button class="key-base key">{"Q"}</button>
                <button class="key-base key">{"W"}</button>
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
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
