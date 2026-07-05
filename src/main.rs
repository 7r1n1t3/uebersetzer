use event_handler::handle_event;
use log::debug;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{process, sync::mpsc};

mod env;
mod event_handler;
mod settings;
mod ueber;
mod uebersetzer;

fn main() -> Result<(), notify::Error> {
    env_logger::init();

    let settings = match settings::load_settings() {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("failed to load settings: {err}");
            process::exit(1);
        }
    };
    debug!("loaded settings: {:?}", settings);

    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |result| {
            let _ = tx.send(result);
        },
        notify::Config::default(),
    )?;

    watcher.watch(
        &settings.env_path,
        if settings.recursive && settings.env_path.is_dir() {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        },
    )?;

    for result in rx {
        match result {
            Ok(event) => {
                if let Err(err) = handle_event(event, &settings) {
                    eprintln!("failed to update config: {err}")
                }
            }
            Err(err) => eprintln!("watch error: {err}"),
        }
    }

    Ok(())
}
