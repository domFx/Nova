use nova_runtime::application::NovaApplication;

fn main() {
    let _ = NovaApplication::new()
        .set_log_level("trace")
        .use_pretty_env_logger()
        .run();
}