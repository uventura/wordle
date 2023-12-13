use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct Props {
    word: [char;5],
    values: [i8; 5],
}

fn get_class_by_value(value: i8) -> String {
    let mut status_tag = String::new();

    match value {
        -1 => status_tag = "".to_owned(),
        0 => status_tag = "wrong".to_owned(),
        1 => status_tag = "wrong-position".to_owned(),
        2 => status_tag = "correct".to_owned(),
        _ => status_tag = "correct".to_owned(),
    }
    return format!("item item-{status_tag}");
}

#[function_component(WordLine)]
fn word_line(props: &Props) -> Html {
    let Props { word, values } = props;
    html! {
        <div class="line">
            <div class={get_class_by_value(values[0])}>{word[0]}</div>
            <div class={get_class_by_value(values[1])}>{word[1]}</div>
            <div class={get_class_by_value(values[2])}>{word[2]}</div>
            <div class={get_class_by_value(values[3])}>{word[3]}</div>
            <div class={get_class_by_value(values[4])}>{word[4]}</div>
        </div>
    }
}

fn init_props() -> Props {
    let props = Props {
        word: [char::default(),char::default(),char::default(),char::default(),char::default()],
        values: [-1,-1,-1,-1,-1]
    };
    return props;
}


#[function_component]
fn Button() -> Html {
    let counter: UseStateHandle<i32> = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let letter: UseStateHandle<char> = use_state(|| char::default());
    let onchange = {
        let letter = letter.clone();
        move |_| {
            let value = 'b';
            letter.set(value);
        }
    };

    html! {
        <div class="line">
            <div {onclick} class="item item-">{*counter}</div>
            <div class="item item-">{*counter}</div>
            <div class="item item-">{*counter}</div>
            <div {onchange} class="item item-">{*letter}</div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct BoxProps {
    pub word: UseStateHandle<[char;5]>
}

#[function_component(WordBox)]
pub fn word_box(props: &BoxProps) -> Html {

    let mut words: [Props; 6] = [
        init_props(),
        init_props(),
        init_props(),
        init_props(),
        init_props(),
        init_props()
    ];

    words[1].word[0] = 'b';

    html! {
        <>
        <WordLine ..words[0].clone() />
        <WordLine ..words[1].clone() />
        <WordLine ..words[2].clone() />
        <WordLine ..words[3].clone() />
        <WordLine ..words[4].clone() />
        <WordLine ..words[5].clone() />
        <Button/>
        </>
    }
}
