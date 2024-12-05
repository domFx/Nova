mod nova_engine;
use nova_engine::NovaEngine;

fn main() {
    let mut engine: NovaEngine = NovaEngine::new()
        .set_log_level("trace")
        .use_pretty_env_logger();

    
    engine.run();

    // engine.destroy();
}