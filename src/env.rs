// uebersetzer: dotfiles templating engine
//  Copyright (C) <2026> <Rayen Daadaa>
//
//  This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
//
/// environment loader
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

pub fn load_env(env_path: &Path) -> Result<HashMap<String, String>, dotenvy::Error> {
    // TODO: support for list of env files
    let mut env_vars = HashMap::new();

    if env_path.is_file() {
        load_env_file(env_path, &mut env_vars)?;
    } else if env_path.is_dir() {
        let mut paths: Vec<PathBuf> = fs::read_dir(env_path)
            .map_err(dotenvy::Error::Io)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<io::Result<Vec<PathBuf>>>()
            .map_err(dotenvy::Error::Io)?;

        paths.sort();

        for path in paths {
            if path.is_file() {
                load_env_file(&path, &mut env_vars)?;
            }
        }
    } else {
        return Err(dotenvy::Error::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "env path does not exist",
        )));
    }

    Ok(env_vars)
}

fn load_env_file(
    path: &Path,
    env_vars: &mut HashMap<String, String>,
) -> Result<(), dotenvy::Error> {
    let iter = dotenvy::from_path_iter(path)?;

    for entry in iter {
        let (key, value) = entry?;
        env_vars.insert(key, value);
    }

    Ok(())
}
