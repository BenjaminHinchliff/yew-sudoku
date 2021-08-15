#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use yew::prelude::*;

mod board;
mod board_view;
mod digit;
mod generate;
mod picker;
mod picker_cell;
use board::Board;
use board_view::BoardView;
use digit::Digit;
use generate::generate_board;
use picker::Picker;

enum Msg {
    GenerateBoard,
    Select((usize, usize)),
    Pick(Option<Digit>),
}

struct Model {
    link: ComponentLink<Self>,
    board: Board,
    picked: Option<Digit>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GenerateBoard);
        Self {
            link,
            board: Board::default(),
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
                self.board[y][x].value = self.picked;
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
                <BoardView data={self.board} onselect=self.link.callback(Msg::Select) />
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
