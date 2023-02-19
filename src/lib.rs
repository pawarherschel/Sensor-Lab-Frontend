use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize)]
pub struct Table<T: PartialEq>
where
    T: ToString + PartialEq + std::fmt::Display,
{
    table: Vec<Vec<T>>,
}

impl<T: PartialEq + std::fmt::Display> IntoIterator for Table<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.table.into_iter()
    }
}

impl<T> From<Vec<Vec<T>>> for Table<T>
where
    T: ToString + PartialEq + std::fmt::Display,
{
    fn from(table: Vec<Vec<T>>) -> Self {
        Self { table }
    }
}

pub fn create_table<T>(table: Table<T>) -> Html
where
    T: ToString + PartialEq + std::fmt::Display,
{
    html! {
        <table>
            { for table.into_iter().map(|row| html! {
                <tr>{
                    for row.into_iter().map(|cell| html! { <td>{cell.to_string()}</td> })
                }</tr>
            })
        }</table>
    }
}

#[styled_component(HelloWorld)]
pub fn hello_world() -> Html {
    let stylesheet = style! { r#"color: pink;"# }.unwrap();

    let stylesheet_2 = style! {
        r#"
        h3 {
            color: blue;
        }
        p {
            color: red;
        }
        "# 
    }
    .unwrap();

    let table = vec![
        vec!["a", "b", "c"],
        vec!["d", "e", "f"],
        vec!["g", "h", "i"],
    ];

    if let Ok(string) = serde_json::to_string_pretty(&table) {
        log!(string);
    }

    html! {
        <>
            <h1 class={stylesheet}>{ "RustConf Explorer" }</h1>
            <div class={stylesheet_2}>
                <h3>{"Videos to watch"}</h3>
                <p>{ "John Doe: Building and breaking things" }</p>
                <p>{ "Jane Smith: The development process" }</p>
                <p>{ "Matt Miller: The Web 7.0" }</p>
                <p>{ "Tom Jerry: Mouse less development" }</p>
            </div>
        <div>
            {create_table(table.into())}
        </div>
        </>
    }
}
