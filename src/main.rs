#![allow(unused)]

mod nova_engine;

use log::error;
use nova_engine::NovaEngine;

fn main() {
    let mut engine: NovaEngine = NovaEngine::new()
        .set_log_level("trace")
        .use_pretty_env_logger();

    {
        let engine_result = engine.run();
    
        if let Err(error) = engine_result {
            error!("Unexpected error encountered: {}", error);
        }
    }

    // engine.destroy();
}