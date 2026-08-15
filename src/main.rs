// uebersetzer: dotfiles templating engine
//  Copyright (C) <2026> <Rayen Daadaa>
//
//  This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
//
use event_handler::handle_event;
use log::debug;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
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
                if let Err(err) = handle_event(event, &settings) {
                    eprintln!("failed to update config: {err}")
                }
            }
            Err(err) => eprintln!("watch error: {err}"),
        }
    }

    Ok(())
}
