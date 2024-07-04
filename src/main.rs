mod nova_engine;
use nova_engine::NovaEngine;

fn main() {
    NovaEngine::new()
        .set_log_level("TRACE")
        .use_pretty_env_logger()
        .run();
}