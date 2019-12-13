#![recursion_limit = "128"]

use rstyping::*;

use yew::events::IKeyboardEvent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    value: String,
    label: String,
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
        let content = "그래도 지구는 돈다
나는 생각한다 고로 나는 존재한다
나치가 그들을 덮쳤을 때
달리는 기차 위에 중립은 없다
당신들을 묻어버리겠다
만국의 노동자여 단결하라
모든 국가는 그에 걸맞은 정부를 가진다
박수칠 때 떠나라
빛이 있으라
빵이 없으면 케이크를 먹으면 되지
신은 죽었다
싸움이 급하니 나의 죽음을 알리지 말라
역사를 잊은 민족에게 미래는 없다
왔노라, 보았노라, 이겼노라
이것 또한 지나가리라
일찍 일어나는 새가 벌레를 잡는다
적의 적은 나의 친구
종교는 인민의 아편이다
죄는 미워하되 사람은 미워하지 말라
주사위는 던져졌다"
            .into();

        Model {
            value: "".into(),
            label: "Press Enter to Start".into(),
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

                //Change value, label, list_index
                if self.list_index >= self.list.len() - 1 {
                    self.list_index = 0;
                }

                self.value = "".into();
                self.list_index += 1;
                self.label = self.list.get(self.list_index).unwrap().into();
            }
            Msg::Nope => {}
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <label>{&self.label}</label>
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
