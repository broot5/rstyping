#![recursion_limit = "128"]

mod texts;

use rstyping::*;

use hangul::HangulExt;
use instant::Instant;
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
    typed: usize,
    timer: Instant,
    elapsed_time: f64,
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
            typed: 0,
            timer: Instant::now(),
            elapsed_time: 0_f64,
            result: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetInput(new_value) => {
                self.value = new_value;

                true
            }
            Msg::Next => {
                //Get elapsed time and start new timer
                self.elapsed_time = self.timer.elapsed().as_secs_f64();
                self.timer = Instant::now();

                //Get typed
                for i in self.value.chars() {
                    match i.is_syllable() {
                        true => match i.is_open().unwrap() {
                            true => self.typed += 2,
                            false => self.typed += 3,
                        },
                        false => self.typed += 1,
                    }
                }

                //Check
                let accuracy = get_accuracy(&self.list.get(self.list_index).unwrap(), &self.value);
                let typing_speed = get_typing_speed(self.typed, self.elapsed_time);

                self.result = format!("{}% {}", accuracy, typing_speed);

                //Change list_index
                self.list_index += 1;
                if self.list_index >= self.list.len() - 1 {
                    self.list_index = 0;
                }

                //init
                self.value = "".into();
                self.text = self.list.get(self.list_index).unwrap().into();
                self.typed = 0;

                true
            }
            Msg::Nope => false,
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="container">
                <div>
                    <label for="input">{&self.text}</label>
                </div>
                <div>
                    <input
                        type="text"
                        id="input"
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
