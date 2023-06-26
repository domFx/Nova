#[path = "./constants.rs"]
mod constants;

#[path = "./platform.rs"]
mod platform;

mod nova_engine;
use nova_engine::NovaEngine;

fn main() {
    NovaEngine::new();
}