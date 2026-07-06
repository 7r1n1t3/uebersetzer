// uebersetzer: dotfiles templating engine
//  Copyright (C) <2026> <Rayen Daada>
//
//  This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
//
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
