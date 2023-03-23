use log::info;
use log_panics::{BacktraceMode, Config};
use std::fs;

pub fn init() {
    let path = appdirs::user_log_dir(Some("genocide"), Some("Genocide"))
        .expect("Failed to get log directory");
    fs::create_dir_all(path.clone()).unwrap();
    let path = path.join("debug.log");
    simple_logging::log_to_file(path.clone(), log::LevelFilter::Debug).unwrap();

    // log panics
    Config::new()
        .backtrace_mode(BacktraceMode::Resolved)
        .install_panic_hook();

    info!("Log file: {}", path.to_str().unwrap());
}
