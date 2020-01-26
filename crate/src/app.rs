mod text;
mod util;

use instant::Instant;
use yew::events::IKeyboardEvent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/module.mjs")]
extern "C" {
    type Chart;

    #[wasm_bindgen(constructor)]
    fn new(arg: String) -> Chart;

    #[wasm_bindgen(method)]
    fn update(this: &Chart);
}

pub struct Model {
    value: String,
    text: String,
    list: Vec<String>,
    list_index: usize,
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
            value: "".into(),
            text: "Press Enter to Start".into(),
            list: util::manufacture_file(&content),
            list_index: 0,
            timer: Instant::now(),
            elapsed_time: 0_f64,
            result: "".into(),
            chart: Chart::new("#chart".into()),
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

                //Check
                let accuracy =
                    util::get_accuracy(&self.list.get(self.list_index).unwrap(), &self.value);
                let typing_speed = util::get_typing_speed(&self.value, self.elapsed_time);

                self.result = format!("{}% {}", accuracy, typing_speed);

                //update Chart
                self.chart = Chart::new("#chart".into());
                self.chart.update();

                //Change list_index
                self.list_index += 1;
                if self.list_index >= self.list.len() - 1 {
                    self.list_index = 0;
                }

                //init
                self.value = "".into();
                self.text = self.list.get(self.list_index).unwrap().into();

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
                        value=&self.value
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
