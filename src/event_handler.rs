/// event handler
use notify::{Event, EventKind};
use std::io;

use crate::env;
use crate::settings;
use crate::uebersetzer;

pub fn handle_event(event: Event, settings: &settings::Settings) -> Result<(), io::Error> {
    let changed = matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_))
        && event.paths.iter().any(|p| p == &settings.env_path);

    if !changed {
        return Ok(());
    }

    let env_vars = env::load_env(settings.env_path.as_path())?;
    uebersetzer::uebersetz(
        settings.config_path.as_path(),
        &env_vars,
        Some(settings.recursive),
        Some(settings.force_write),
    )?;

    Ok(())
}
