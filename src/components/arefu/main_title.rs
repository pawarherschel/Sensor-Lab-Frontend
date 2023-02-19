use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: Option<String>,
    pub on_load: Callback<String>,
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let stylesheet = style! { r#"
    .Some {
        color: pink;
    }
    .None {
        color: blue;
    }
    "# }
    .unwrap();

    props.on_load.emit("Hello".to_string());

    html! {
        <div class={stylesheet}>
        if let Some(title) = &props.title {
            <h1 class="Some">{title}</h1>
        } else {
            <h1 class="None">{"Arefu"}</h1>
        }
        </div>
    }
}
