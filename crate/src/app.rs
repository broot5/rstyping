mod text;
mod util;

use instant::Instant;
use wasm_bindgen::prelude::*;
use yew::events::IKeyboardEvent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[wasm_bindgen(module = "/module.mjs")]
extern "C" {
    type Chart;

    #[wasm_bindgen(constructor)]
    fn new() -> Chart;

    #[wasm_bindgen(method)]
    fn init(this: &Chart, arg: String);

    #[wasm_bindgen(method)]
    fn update(this: &Chart, accuracy: usize, typing_speed: usize);
}

pub struct Model {
    text: String,
    input: String,
    text_list: Vec<String>,
    text_list_index: usize,
    timer: Instant,
    elapsed_time: f64,
    result: String,
    chart: Chart,
}

pub enum Msg {
    GetInput(String),
    Next,
    Nope,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let content = text::texts();

        Model {
            text: "Press Enter to Start".into(),
            input: "".into(),
            text_list: util::manufacture_file(&content),
            text_list_index: 0,
            timer: Instant::now(),
            elapsed_time: 0_f64,
            result: "".into(),
            chart: Chart::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetInput(new_value) => {
                self.input = new_value;

                true
            }
            Msg::Next => {
                //Get elapsed time and start new timer
                self.elapsed_time = self.timer.elapsed().as_secs_f64();
                self.timer = Instant::now();

                //Check
                let accuracy = util::get_accuracy(
                    &self.text_list.get(self.text_list_index).unwrap(),
                    &self.input,
                );
                let typing_speed = util::get_typing_speed(&self.input, self.elapsed_time);

                self.result = format!("{}% {}", accuracy, typing_speed);

                //If first time, init the chart
                if self.text_list_index == 0 {
                    self.chart.init("#chart".into());
                }

                //Change list_index
                self.text_list_index += 1;
                if self.text_list_index >= self.text_list.len() - 1 {
                    self.text_list_index = 0;
                }

                //init
                self.input = "".into();
                self.text = self.text_list.get(self.text_list_index).unwrap().into();

                //Update chart
                self.chart.update(accuracy, typing_speed);

                true
            }
            Msg::Nope => false,
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="container">
                <div>
                    <label for="mainInput">{&self.text}</label>
                </div>
                <div>
                    <input
                        type="text"
                        id="mainInput"
                        value=&self.input
                        oninput=|e| Msg::GetInput(e.value)
                        onkeypress=|e| {
                            if e.key() == "Enter" {Msg::Next} else {Msg::Nope}}/>
                </div>
                <div>
                    <label id="result">{&self.result}</label>
                </div>
                <div>
                    <canvas id="chart"></canvas>
                </div>
            </div>
        }
    }
}
