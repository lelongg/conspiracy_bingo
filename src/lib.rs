#![recursion_limit = "256"]

use anyhow::Result;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs::File;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Conspiracy {
    name: String,
    checked: bool,
}

impl Conspiracy {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            checked: false,
        }
    }
}

struct Model {
    link: ComponentLink<Self>,
    conspiracies: Vec<Conspiracy>,
}

enum Msg {
    ToggleConspiracy(usize),
}

fn get_conspiracies() -> Result<Vec<Conspiracy>> {
    let mut rng = &mut rand::thread_rng();
    let mut all_conspiracies: Vec<String> =
        serde_json::from_str(include_str!("../assets/conspiracies.json"))?;
    all_conspiracies.shuffle(&mut rng);
    let conspiracies_name: Vec<_> = all_conspiracies
        .choose_multiple(&mut rng, 9)
        .cloned()
        .collect();
    let conspiracies = conspiracies_name
        .iter()
        .map(|name| Conspiracy::new(name))
        .collect();
    Ok(conspiracies)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            conspiracies: get_conspiracies().unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            ToggleConspiracy(conspiracy_index) => {
                self.conspiracies[conspiracy_index].checked =
                    !self.conspiracies[conspiracy_index].checked;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="title">{"Conspiracy Bingo"}</div>
                <div class="grid">
                {
                    self.conspiracies.iter().enumerate().map(
                        |(index, conspiracy)| html! {
                            <div class={if conspiracy.checked { &["item", "checked"] as &[_] } else { &["item"] }}
                                onclick=self.link.callback(move |_| Msg::ToggleConspiracy(index))>
                            {&conspiracy.name}
                            </div>}
                        ).collect::<Html>()
                }
                </div>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
