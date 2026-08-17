use log::info;
use notify::{Event, EventKind};
use std::path::Path;

use crate::env;
use crate::settings;
use crate::uebersetzer;

#[derive(thiserror::Error, Debug)]
pub enum EventHandlerError {
    #[error("env loader error: {0}")]
    EnvLoaderError(#[from] env::EnvLoaderError),
    #[error("uebersetzer error: {0}")]
    UebersetzError(#[from] uebersetzer::UebersetzError),
}

/// call uebersetz on config_path whenever a Modify/Create signal is received for env_path
///
/// # Arguments
///
/// * event: catched event
/// * settings: user settings
///
/// # Errors
///
/// returns error if it failed to load env or to uebersetz file
pub fn handle_event(event: Event, settings: &settings::Settings) -> Result<(), EventHandlerError> {
    let env_changed = matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_))
        && event
            .paths
            .iter()
            .any(|p| is_env_path(p, &settings.env_path));

    if !env_changed {
        return Ok(());
    }

    info!(
        "detected file change: {}",
        settings.config_path.as_path().display()
    );

    let env_vars = env::load_env(Some(settings.env_path.as_path()))?;
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

/// Returns whether `env_path` is a file or a directory.
///
/// # Arguments
///
/// * path: path to check
///
/// # Returns
///
/// true if `path` refers to (or is contained within) `env_path`,
fn is_env_path(path: &Path, env_path: &Path) -> bool {
    if env_path.is_file() {
        path == env_path
    } else if env_path.is_dir() {
        path.starts_with(env_path)
    } else {
        false
    }
}
