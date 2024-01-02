#[path = "vulkan/platform.rs"]
mod platform;

#[path = "./utils.rs"]
mod utils;

#[path = "./vulkan_app.rs"]
mod vulkan_app;

mod nova_engine;

mod vulkan;

use nova_engine::NovaEngine;
use pretty_env_logger;

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    // std::env::set_var("RUST_BACKTRACE", "full");
    pretty_env_logger::init();

    NovaEngine::new();
}