#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use sudoku::Sudoku;
use yew::prelude::*;

mod board;
mod board_view;
mod controls;
mod digit;
mod generate;
mod picker;
mod picker_cell;
use board::Board;
use board_view::BoardView;
use controls::Controls;
use digit::Digit;
use generate::generate_board;
use picker::Picker;

enum Msg {
    GenerateBoard,
    Select((usize, usize)),
    Pick(Option<Digit>),
    Solve,
    IsSolved,
}

struct Model {
    link: ComponentLink<Self>,
    board: Board,
    picked: Option<Digit>,
    solved: bool,
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
            solved: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GenerateBoard => {
                self.board = generate_board();
                self.solved = false;
            }
            Msg::Pick(picked) => {
                self.picked = picked;
            }
            Msg::Select((x, y)) => {
                self.board.data[y][x].value = self.picked;
                self.link.send_message(Msg::IsSolved);
            }
            Msg::Solve => {
                self.board = Sudoku::from_bytes(self.board.into())
                    .unwrap()
                    .some_solution()
                    .unwrap()
                    .to_bytes()
                    .into();
                self.link.send_message(Msg::IsSolved);
            }
            Msg::IsSolved => {
                if Sudoku::from_bytes(self.board.into()).unwrap().is_solved() {
                    self.solved = true;
                }
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
                <BoardView board=self.board onselect=self.link.callback(Msg::Select) />
                <Picker picked=self.picked onpick=self.link.callback(Msg::Pick) />
                <Controls onsolve=self.link.callback(|_| Msg::Solve) onnew=self.link.callback(|_| Msg::GenerateBoard) />
                <p id="solved" class=if self.solved { "solved-visible" } else { "" }>{ "Congrats! You Solved It" }</p>
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
