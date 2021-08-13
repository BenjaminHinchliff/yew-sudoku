use yew::prelude::*;

mod cell;
mod digit;
mod board;
use board::{Board, BoardData};

enum Msg {
    AddOne,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    board: BoardData,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            board: [[None; 9]; 9],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <Board data={self.board} />
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(Default::default());
    yew::start_app::<Model>();
}
