use log::*;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
    Test,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Test => {
                info!("Test triggered.")
            }
            Msg::Click => {
                info!("Click triggered.");
                self.link.send_self(Msg::Test);
            }
        }
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        info!("rendered!");
        html! {
            <div>
                <button onclick=|_| { Msg::Click }>{ "Click" }</button>
            </div>
        }
    }
}