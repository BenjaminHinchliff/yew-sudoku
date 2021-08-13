use yew::prelude::*;

use crate::{cell::Cell, digit::Digit};

pub type BoardData = [[Option<Digit>; 9]; 9];

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub data: BoardData,
}

pub struct Board(Props);
impl Component for Board {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.0 != props {
            self.0 = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="board">
                { self.0.data.iter().flatten().map(|c| { html! {
                    <Cell value={c.clone()} />
                } }).collect::<Html>() }
            </div>
        }
    }
}
