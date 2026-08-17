use log::debug;
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};
use tera::Context;

#[derive(thiserror::Error, Debug)]
pub enum EnvLoaderError {
    #[error("filesystem error: {0}")]
    Io(#[from] io::Error),
    #[error("dotenvy error: {0}")]
    Dotenvy(#[from] dotenvy::Error),
}

/// Loads context for Tera from environment variables and then from env_path
///
/// # Arguments
///
/// * `env_path` - path of environment file(s)
///
/// # Returns
///
/// HashMap of context
///
/// # Errors
///
/// Returns dotenvy::Error if any error during parsing of env_path happened
pub fn load_env(env_path: Option<&Path>) -> Result<HashMap<String, String>, EnvLoaderError> {
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

/// Loads env from file
///
/// # Arguments
///
/// * `filepath` - path of env file
/// * `env_vars` - map of env variables to insert to
///
/// # Errors
///
/// Returns dotenvy::Error if any error during parsing of filepath happened
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

/// Loads env from directory
/// Skips a file if it can't import it
///
/// # Arguments
///
/// * `dir` - path of env directory
/// * `env_vars` - map of env variables to insert to
///
/// # Errors
///
/// Returns io::Error if any error occurs during traversing of dir
fn _load_env_dir(dir: &Path, env_vars: &mut HashMap<String, String>) -> Result<(), io::Error> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<io::Result<Vec<PathBuf>>>()?;

    paths.sort();

    for path in paths {
        if path.is_file() && _load_env_file(&path, env_vars).is_err() {
            continue;
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
