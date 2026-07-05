/// environment loader
use std::{collections::HashMap, fs, io, path::Path};

pub fn load_env(env_path: &Path) -> io::Result<HashMap<String, String>> {
    let mut env_vars = HashMap::new();

    if env_path.is_file() {
        load_env_file(env_path, &mut env_vars)?;
    } else if env_path.is_dir() {
        let mut paths = fs::read_dir(env_path)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<io::Result<Vec<_>>>()?;

        paths.sort();

        for path in paths {
            if path.is_file() {
                load_env_file(&path, &mut env_vars)?;
            }
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("env path does not exist: {}", env_path.display()),
        ));
    }

    Ok(env_vars)
}

fn load_env_file(path: &Path, env_vars: &mut HashMap<String, String>) -> io::Result<()> {
    let iter = dotenvy::from_path_iter(path).map_err(_dotenvy_to_io)?;

    for entry in iter {
        let (key, value) = entry.map_err(_dotenvy_to_io)?;
        env_vars.insert(key, value);
    }

    Ok(())
}

fn _dotenvy_to_io(err: dotenvy::Error) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, err.to_string())
}
