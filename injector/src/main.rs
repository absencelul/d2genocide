use dll_syringe::{process::OwnedProcess, Syringe};
use log::{debug, error};
use std::path::Path;

#[cfg(feature = "logging")]
mod logging;

fn main() {
    #[cfg(feature = "logging")]
    logging::init();

    let process_name = "Game.exe";
    let dll_path = dll_path();
    debug!("DLL path: {}", dll_path);

    let target_process = OwnedProcess::find_first_by_name(process_name);
    if target_process.is_none() {
        eprintln!("Unable to find process: {}", process_name);
        error!("Unable to find process: {}", process_name);
        std::process::exit(1);
    }

    let syringe = Syringe::for_process(target_process.unwrap());
    let _payload = syringe.inject(dll_path).expect("Unable to inject DLL");

    std::process::exit(0);
}

fn dll_path<'a>() -> &'a str {
    let mut path = Path::new("genocide.dll");

    #[cfg(debug_assertions)]
    {
        if !path.exists() {
            path = Path::new("target/i686-pc-windows-msvc/debug/genocide.dll");
        }

        if !path.exists() {
            path = Path::new("target/i686-pc-windows-msvc/debug/deps/genocide.dll");
        }
    }

    #[cfg(not(debug_assertions))]
    {
        if !path.exists() {
            path = Path::new("target/i686-pc-windows-msvc/release/genocide.dll");
        }

        if !path.exists() {
            path = Path::new("target/i686-pc-windows-msvc/release/deps/genocide.dll");
        }
    }

    if !path.exists() {
        eprintln!("DLL not found");
        error!("DLL not found");
        std::process::exit(1);
    }

    path.to_str().expect("Unable to convert path to string")
}
