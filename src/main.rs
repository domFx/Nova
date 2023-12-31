#[path = "./constants.rs"]
mod constants;

#[path = "./platform.rs"]
mod platform;

#[path = "./utils.rs"]
mod utils;

mod nova_engine;

use nova_engine::NovaEngine;
use pretty_env_logger;

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    NovaEngine::new();
}