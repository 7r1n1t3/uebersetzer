use event_handler::handle_event;
use log::debug;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::{process, sync::mpsc};

mod env;
mod error;
mod event_handler;
mod settings;
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

    let env_vars: HashMap<String, String> = env::load_env(Some(settings.env_path.as_path()))
        .unwrap_or_else(|err| panic!("couldn't load environment variables: {err}"));
    uebersetzer::uebersetz(
        settings.config_path.as_path(),
        &env_vars,
        Some(settings.recursive),
        Some(settings.force_write),
    )?;

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
                debug!("got event: {:?}", event);
                if let Err(err) = handle_event(event, &settings) {
                    eprintln!("failed to update config: {:?}", err)
                }
            }
            Err(err) => eprintln!("watch error: {err}"),
        }
    }

    Ok(())
}
