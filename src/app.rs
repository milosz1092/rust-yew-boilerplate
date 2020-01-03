use log::*;
use yew::worker::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use serde_derive::{Deserialize, Serialize};

pub mod worker_prime;

#[derive(Serialize, Deserialize)]
pub struct State {
    prime_range_first: String,
    prime_range_last: String,
}

pub struct App {
    link: ComponentLink<Self>,
    worker_prime: Box<dyn Bridge<worker_prime::Worker>>,
    state: State,
}

pub enum Msg {
    UpdatePrimeRange(String, String),
    DataReceived(worker_prime::Response),
    ComputePrime,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::DataReceived);
        let worker_prime = worker_prime::Worker::bridge(callback);

        let state = State {
            prime_range_first: "".into(),
            prime_range_last: "".into()
        };

        App { worker_prime, link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdatePrimeRange(val, part) => {
                if part == "first" {
                    self.state.prime_range_first = val;
                } else if part == "last" {
                    self.state.prime_range_last = val;
                }
            }
            Msg::ComputePrime => {
                info!("Click triggered.");
                
                self.worker_prime.send(worker_prime::Request::GetPrimeCount(
                    self.state.prime_range_first.parse::<i32>().unwrap(),
                    self.state.prime_range_last.parse::<i32>().unwrap()
                ));
            }
            Msg::DataReceived(worker_prime_response) => {
                match worker_prime_response {
                    worker_prime::Response::PrimeCount(value) => info!("Data from agent: {}", value),
                }
                
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        info!("rendered!");
        html! {
            <div>
                { self.view_prime_inputs() }
                <button onclick=|_| { Msg::ComputePrime }>{ "Compute" }</button>
            </div>
        }
    }
}

impl App {
    fn view_prime_inputs(&self) -> Html<App> {
        html! {
            <div>
                <label>
                    { "from: " }
                </label>
                <input class="prime-begin"
                    placeholder="first from range"
                    type="number"
                    value=&self.state.prime_range_first
                    oninput=|e| Msg::UpdatePrimeRange(e.value, "first".to_string())
                />

                <label>
                    { "to: " }
                </label>
                <input class="prime-end"
                    placeholder="last from range"
                    type="number"
                    value=&self.state.prime_range_last
                    oninput=|e| Msg::UpdatePrimeRange(e.value, "last".to_string())
                />
            </div>
        }
    }
}