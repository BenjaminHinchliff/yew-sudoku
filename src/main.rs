#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use digit::Digit;
use generate::generate_board;
use yew::prelude::*;

mod board;
mod digit;
mod generate;
mod picker;
mod picker_cell;
use board::{Board, BoardData};
use picker::Picker;

enum Msg {
    GenerateBoard,
    Select((usize, usize)),
    Pick(Option<Digit>),
}

struct Model {
    link: ComponentLink<Self>,
    board: BoardData,
    picked: Option<Digit>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GenerateBoard);
        Self {
            link,
            board: BoardData::default(),
            picked: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GenerateBoard => {
                self.board = generate_board();
            }
            Msg::Pick(picked) => {
                self.picked = picked;
            }
            Msg::Select((x, y)) => {
                self.board[y][x] = self.picked;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <Board data={self.board} onselect=self.link.callback(Msg::Select) />
                <Picker picked={self.picked} onpick=self.link.callback(Msg::Pick) />
            </main>
        }
    }
}

#[cfg(feature = "wasm-logger")]
fn init_logging() {
    wasm_logger::init(Default::default());
}

#[cfg(not(feature = "wasm-logger"))]
fn init_logging() {}

fn main() {
    init_logging();
    yew::start_app::<Model>();
}
