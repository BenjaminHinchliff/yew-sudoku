use yew::prelude::*;

use crate::digit::Digit;

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Pick(Option<Digit>),
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub value: Option<Digit>,
    pub picked: bool,
    pub onpick: Callback<Option<Digit>>,
}

pub struct PickerCell {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for PickerCell {
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
        let value = self.props.value;
        let onclick = self.link.callback(move |_| Msg::Pick(value));
        let val = if let Some(digit) = value {
            digit.to_string()
        } else {
            "X".to_string()
        };
        let pick_classes = classes!(
            "pick",
            if self.props.picked {
                Some("pick-selected")
            } else {
                None
            }
        );
        html! {
            <button class=pick_classes onclick={onclick}>{ val }</button>
        }
    }
}
