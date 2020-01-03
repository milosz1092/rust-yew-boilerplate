use log::info;
use serde_derive::{Deserialize, Serialize};
// use std::time::Duration;
use yew::worker::*;
// use yew::services::fetch::FetchService;
// use yew::services::interval::IntervalService;
// use yew::services::Task;

pub mod utils;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetPrimeCount(i32, i32),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    PrimeCount(i32),
}

pub enum Msg {
    // Updating,
}

pub struct Worker {
    link: AgentLink<Worker>,
    // interval: IntervalService,
    // task: Box<dyn Task>,
    // fetch: FetchService,
}

impl Agent for Worker {
    type Reach = Public;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        // let mut interval = IntervalService::new();
        // let duration = Duration::from_secs(3);
        // let callback = link.send_back(|_| Msg::Updating);
        // let task = interval.spawn(duration, callback);
        Worker {
            link,
            // interval,
            // task: Box::new(task),
            // fetch: FetchService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            // Msg::Updating => {
            //     info!("Tick...");
            // }
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            Request::GetPrimeCount(first, last) => {
                let result = utils::count_prime_numbers_in_range(first, last);
                self.link.response(who, Response::PrimeCount(result));
            }
        }
    }

    fn name_of_resource() -> &'static str {
        "worker_prime.js"
    }
}
