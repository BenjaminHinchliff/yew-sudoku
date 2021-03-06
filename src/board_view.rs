use yew::prelude::*;

use crate::board::Board;

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Select((usize, usize)),
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub board: Board,
    pub onselect: Callback<(usize, usize)>,
}

pub struct BoardView {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for BoardView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(pos) => self.props.onselect.emit(pos),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let cells =
            self.props
                .board
                .data
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().map(move |(x, c)| {
                    let onclick = self.link.callback(move |_| Msg::Select((x, y)));
                    let val = if let Some(c) = c.value {
                        c.to_string()
                    } else {
                        " ".to_string()
                    };
                    let borders = vec![
                        if x % 3 == 0 { Some("cell-left") } else { None },
                        if x % 3 == 2 { Some("cell-right") } else { None },
                        if y % 3 == 0 { Some("cell-top") } else { None },
                        if y % 3 == 2 { Some("cell-bottom") } else { None },
                    ];
                    let classes = classes!("cell", borders);
                    html! {
                        <button class=classes disabled=c.disabled onclick={onclick}>{ val }</button>
                    }
                })
                })
                .collect::<Html>();
        html! {
            <div id="board">
                { cells }
            </div>
        }
    }
}
