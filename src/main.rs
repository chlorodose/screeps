use tracing::debug;

mod common;

fn main() {
    tracing_subscriber::FmtSubscriber::builder().pretty().with_env_filter(tracing_subscriber::EnvFilter::from_env("LOG_LEVEL")).init();
    debug!("Initializing v8({})...", v8::VERSION_STRING);
    v8::V8::initialize_platform(v8::new_default_platform(0, false).make_shared());
    v8::V8::initialize();
    debug!("V8 success initialized!");
    debug!("Staring app...");
    bevy::app::App::new().run();
}