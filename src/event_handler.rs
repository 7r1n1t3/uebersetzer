use log::info;
use notify::{Event, EventKind};
use std::io;

use crate::env;
use crate::error::UebersetzError;
use crate::settings;
use crate::uebersetzer;

pub fn handle_event(event: Event, settings: &settings::Settings) -> Result<(), UebersetzError> {
    let changed = matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_))
        && event.paths.iter().any(|p| p == &settings.env_path);

    if !changed {
        return Ok(());
    }

    info!(
        "detected file change: {}",
        settings.config_path.as_path().display()
    );

    let env_vars = env::load_env(Some(settings.env_path.as_path())).map_err(|err| match err {
        dotenvy::Error::Io(err) => err,
        err => io::Error::new(io::ErrorKind::InvalidData, err),
    })?;
    uebersetzer::uebersetz(
        settings.config_path.as_path(),
        &env_vars,
        Some(settings.recursive),
        Some(settings.force_write),
    )?;

    info!(
        "done uebersetzing: {}",
        settings.config_path.as_path().display()
    );

    Ok(())
}
