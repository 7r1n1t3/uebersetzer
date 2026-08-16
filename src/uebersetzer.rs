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
use tera::{Context, Tera};

use crate::env::load_env_to_tera_context;

use walkdir::WalkDir;
const UEBER_FILE_EXTENSION: &str = ".ueber";
const TMP_FILE_EXTENSION: &str = "ueber_tmp";

#[derive(thiserror::Error, Debug)]
pub enum UebersetzError {
    #[error("filesystem type error: {0}")]
    IO(String),
    #[error("tera error: {0}")]
    Tera(String),
}

impl From<UebersetzError> for io::Error {
    fn from(err: UebersetzError) -> Self {
        std::io::Error::other(err)
    }
}

impl From<io::Error> for UebersetzError {
    fn from(err: io::Error) -> Self {
        Self::IO(err.to_string())
    }
}

impl From<tera::Error> for UebersetzError {
    fn from(err: tera::Error) -> Self {
        Self::Tera(err.to_string())
    }
}

pub fn uebersetz(
    conf_path: &Path,
    env_vars: &HashMap<String, String>,
    recursive: Option<bool>,
    force_write: Option<bool>,
) -> Result<(), io::Error> {
    let max_depth = if recursive.unwrap_or(false) {
        usize::MAX
    } else {
        1
    };

    for entry in WalkDir::new(conf_path)
        .min_depth(1)
        .max_depth(max_depth)
        .follow_links(false)
    {
        let entry = entry.map_err(io::Error::other)?;

        if entry.file_type().is_file()
            && entry
                .file_name()
                .to_string_lossy()
                .contains(UEBER_FILE_EXTENSION)
        {
            debug!("uebersetzing file: {:?}", entry);
            uebersetz_file(entry.path(), env_vars, force_write)?;
        }
    }

    Ok(())
}

pub fn uebersetz_file(
    src_conf: &Path,
    env_vars: &HashMap<String, String>,
    force_write: Option<bool>,
) -> Result<(), UebersetzError> {
    let force_write = force_write.unwrap_or(false);

    let mut tera = Tera::default();
    let mut context = Context::new();
    load_env_to_tera_context(env_vars, &mut context);

    let dst_conf_filename: String = src_conf
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .replace(UEBER_FILE_EXTENSION, "");
    let dst_conf: PathBuf = src_conf.with_file_name(dst_conf_filename);

    if dst_conf.is_file() && !force_write {
        return Ok(());
    }

    tera.add_template_file(src_conf, Some("conf"))?;
    let rendered = tera.render("conf", &context)?;

    let tmp_conf: PathBuf = dst_conf.with_added_extension(TMP_FILE_EXTENSION);
    fs::write(&tmp_conf, rendered)?;
    debug!("writing to: {:?} ", dst_conf);

    fs::rename(tmp_conf, dst_conf)?;

    Ok(())
}
