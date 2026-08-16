// uebersetzer: dotfiles templating engine
//  Copyright (C) <2026> <Rayen Daadaa>
//
//  This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
//
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
