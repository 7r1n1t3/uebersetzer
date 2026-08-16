// uebersetzer: dotfiles templating engine
//  Copyright (C) <2026> <Rayen Daadaa>
//
//  This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
//
use log::debug;
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};
use tera::Context;

pub fn load_env(env_path: Option<&Path>) -> Result<HashMap<String, String>, dotenvy::Error> {
    let mut env_vars = HashMap::new();

    debug!("loading environment variables");
    _load_env_vars(&mut env_vars)?;

    if let Some(env_path) = env_path {
        debug!("loading env file(s): {0}", env_path.display());
        if env_path.is_file() {
            _load_env_file(env_path, &mut env_vars)?;
        } else if env_path.is_dir() {
            _load_env_dir(env_path, &mut env_vars)?;
        }
    }

    Ok(env_vars)
}

fn _load_env_file(
    filepath: &Path,
    env_vars: &mut HashMap<String, String>,
) -> Result<(), dotenvy::Error> {
    let iter = dotenvy::from_path_iter(filepath)?;

    for entry in iter {
        let (key, value) = entry?;
        env_vars.insert(key, value);
    }

    Ok(())
}

fn _load_env_dir(dir: &Path, env_vars: &mut HashMap<String, String>) -> Result<(), dotenvy::Error> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(dotenvy::Error::Io)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<io::Result<Vec<PathBuf>>>()
        .map_err(dotenvy::Error::Io)?;

    paths.sort();

    for path in paths {
        if path.is_file() {
            _load_env_file(&path, env_vars)?;
        }
    }

    Ok(())
}

fn _load_env_vars(env_vars: &mut HashMap<String, String>) -> Result<(), dotenvy::Error> {
    for (key, value) in std::env::vars() {
        env_vars.insert(key, value);
    }

    Ok(())
}

pub fn load_env_to_tera_context(env: &HashMap<String, String>, context: &mut Context) {
    for (env_var, value) in env {
        context.insert(env_var.clone(), value);
    }
}
