use yew::agent::Threaded;

mod worker_prime;

fn main() {
    web_logger::init();
    yew::initialize();
    worker_prime::Worker::register();
    yew::run_loop();
}
