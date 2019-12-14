#![recursion_limit = "128"]

mod texts;

use rstyping::*;

use yew::events::IKeyboardEvent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    value: String,
    text: String,
    list: Vec<String>,
    list_index: usize,
    result: String,
}

enum Msg {
    GetInput(String),
    Next,
    Nope,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let content = texts::texts();

        Model {
            value: "".into(),
            text: "Press Enter to Start".into(),
            list: manufacture_file(&content),
            list_index: 0,
            result: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetInput(new_value) => {
                self.value = new_value.trim().parse().unwrap();
            }
            Msg::Next => {
                //Check
                self.result = check(self.list.get(self.list_index).unwrap(), &self.value);

                //Change value, text, list_index
                if self.list_index >= self.list.len() - 1 {
                    self.list_index = 0;
                }

                self.value = "".into();
                self.list_index += 1;
                self.text = self.list.get(self.list_index).unwrap().into();
            }
            Msg::Nope => {}
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <label>{&self.text}</label>
                </div>
                <div>
                    <input
                        type="text"
                        value=&self.value
                        oninput=|e| Msg::GetInput(e.value)
                        onkeypress=|e| {
                            if e.key() == "Enter" {Msg::Next} else {Msg::Nope}}/>
                </div>
                <div>
                    <label>{&self.result}</label>
                </div>
            </div>
        }
    }
}
