use clap::Parser;
use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::path::PathBuf;

const SETTINGS_FILE: &str = "uebersetzer.toml";

#[derive(Debug, Deserialize, Parser)]
#[command(version, about, long_about = None)]
pub struct Settings {
    /// Directory where config files live
    #[arg(long)]
    pub config_path: PathBuf,

    #[arg(short, long)]
    /// Path where environment variables live
    pub env_path: PathBuf,

    /// Whether to read files recursively from path if it points to a directory
    #[arg(short, long)]
    #[serde(default)]
    pub recursive: bool,

    /// Force write to existing config values
    #[arg(short, long)]
    #[serde(default)]
    pub force_write: bool,
}

pub fn load_settings() -> Result<Settings, config::ConfigError> {
    let mut builder = Config::builder();
    let xdg = xdg::BaseDirectories::new();

    let mut config_dirs = xdg.get_config_dirs();
    config_dirs.reverse();

    for path in config_dirs {
        builder = builder.add_source(
            File::from(path.join(SETTINGS_FILE))
                .format(FileFormat::Toml)
                .required(false),
        );
    }

    if let Some(path) = xdg.get_config_file(SETTINGS_FILE) {
        builder = builder.add_source(File::from(path).format(FileFormat::Toml).required(false));
    }

    let mut settings: Settings = builder.build()?.try_deserialize()?;
    settings.config_path = expand_path(settings.config_path)?;
    settings.env_path = expand_path(settings.env_path)?;

    Ok(settings)
}

fn expand_path(path: PathBuf) -> Result<PathBuf, config::ConfigError> {
    let expanded = shellexpand::full(
        path.to_str()
            .ok_or_else(|| config::ConfigError::Message("path is not valid UTF-8".into()))?,
    )
    .map_err(|err| config::ConfigError::Message(format!("failed to expand path: {err}")))?;

    Ok(PathBuf::from(expanded.into_owned()))
}
