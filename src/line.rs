use yew::prelude::*;

/// Propriedades para o componente `Line` (`answers` e `words`)

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// Resultado das respostas da linha - Inicializado com -1.
    pub answers: [i8; 5],
    /// Array de letras da linha.
    pub words: [char; 5],
}

/// Renderiza a linha exibindo as letras de cada bloco e modificando a classe de cada div de acordo com o valor das respostas.
#[function_component(Line)]
pub fn line(props: &Props) -> Html {
    let Props { answers, words } = props;

    html! {
        <div class="line">
            <div class={"item item-".to_owned()+&answers[0].to_string()}>{words[0]}</div>
            <div class={"item item-".to_owned()+&answers[1].to_string()}>{words[1]}</div>
            <div class={"item item-".to_owned()+&answers[2].to_string()}>{words[2]}</div>
            <div class={"item item-".to_owned()+&answers[3].to_string()}>{words[3]}</div>
            <div class={"item item-".to_owned()+&answers[4].to_string()}>{words[4]}</div>
        </div>
    }
}