use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Solve,
    New,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub onsolve: Callback<()>,
    pub onnew: Callback<()>,
}

pub struct Controls {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Controls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Solve => {
                self.props.onsolve.emit(());
            }
            Msg::New => {
                self.props.onnew.emit(());
            }
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
        html! {
            <div id="controls">
                <button onclick=self.link.callback(|_| Msg::Solve)>{ "solve" }</button>
                <button onclick=self.link.callback(|_| Msg::New)>{ "new" }</button>
            </div>
        }
    }
}
