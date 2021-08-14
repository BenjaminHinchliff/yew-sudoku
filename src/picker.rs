use enum_iterator::IntoEnumIterator;
use yew::prelude::*;

use crate::digit::Digit;

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Pick(Option<Digit>),
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub picked: Option<Digit>,
    pub onpick: Callback<Option<Digit>>,
}

pub struct Picker {
    link: ComponentLink<Self>,
    props: Props,
}

impl Picker {
    fn render_pick(&self, digit: Option<Digit>) -> Html {
        let onclick = self.link.callback(move |_| Msg::Pick(digit));
        let val = if let Some(digit) = digit {
            digit.to_string()
        } else {
            "X".to_string()
        };
        html! {
            <button class="pick" onclick={onclick}>{ val }</button>
        }
    }
}

impl Component for Picker {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Pick(digit) => {
                self.props.onpick.emit(digit);
                true
            }
        }
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
        let values = Digit::into_enum_iter()
            .map(|d| Some(d))
            .chain(std::iter::once(None));
        html! {
            <div id="picker">
                { values.map(|d| self.render_pick(d)).collect::<Html>() }
            </div>
        }
    }
}
