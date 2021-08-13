use yew::prelude::*;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub value: Option<u8>,
}

pub struct Cell(Props);
impl Component for Cell {
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
        let val = if let Some(v) = self.0.value {
            v.to_string()
        } else {
            " ".to_string()
        };
        html! {
            <button class="cell">{ val }</button>
        }
    }
}
