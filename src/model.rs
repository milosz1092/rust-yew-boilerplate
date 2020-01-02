use log::*;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {}
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        info!("rendered!");
        html! {
            <div>
                <button onclick=|_| { Msg::Click }>{ "Click" }</button>
            </div>
        }
    }
}