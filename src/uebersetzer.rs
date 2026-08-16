// uebersetzer: dotfiles templating engine
//  Copyright (C) <2026> <Rayen Daadaa>
//
//  This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
//
/// uebersetzer logic implementation
use regex::{Captures, Regex};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

use crate::ueber;

const UEBER_FILE_EXTENSION: &str = ".ueber";
const TMP_FILE_EXTENSION: &str = "ueber_tmp";

pub fn uebersetz(
    conf_path: &Path,
    env_vars: &HashMap<String, String>,
    recursive: Option<bool>,
    force_write: Option<bool>,
) -> Result<(), io::Error> {
    for entry in fs::read_dir(conf_path)? {
        let path: PathBuf = entry?.path();

        if path.is_file()
            && path
                .to_owned()
                .into_os_string()
                .into_string()
                .unwrap_or_default()
                .contains(UEBER_FILE_EXTENSION)
        {
            uebersetz_file(path.as_path(), env_vars, force_write)?;
        } else if path.is_dir() && recursive.unwrap_or(false) {
            uebersetz(&path, env_vars, Some(true), force_write)?;
        }
    }

    Ok(())
}

pub fn uebersetz_file(
    src_conf: &Path,
    env_vars: &HashMap<String, String>,
    force_write: Option<bool>,
) -> Result<(), io::Error> {
    let dst_conf_filename: String = src_conf
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .replace(UEBER_FILE_EXTENSION, "");
    let dst_conf: PathBuf = src_conf.with_file_name(dst_conf_filename);
    let bk_conf: PathBuf = dst_conf.with_added_extension("bk");

    if dst_conf.is_file() {
        if force_write.unwrap_or(false) {
            fs::copy(dst_conf.as_path(), bk_conf.as_path())?;
            fs::remove_file(dst_conf.as_path())?;
        } else {
            return Ok(());
        }
    }

    let tmp_conf: PathBuf = dst_conf.with_added_extension(TMP_FILE_EXTENSION);
    let reader = io::BufReader::new(File::open(src_conf)?);
    let mut writer = io::BufWriter::new(File::create(&tmp_conf)?);
    let var_placeholder = Regex::new(ueber::VAR_PLACEHOLDER).unwrap();

    for line in reader.lines() {
        let line = line?;
        let parsed = var_placeholder.replace_all(&line, |caps: &Captures<'_>| {
            env_vars
                .get(&caps[1])
                .cloned()
                .or_else(|| env::var(&caps[1]).ok())
                .unwrap_or_else(|| caps[0].to_string())
        });

        writeln!(writer, "{parsed}")?;
    }

    writer.flush()?;
    fs::rename(tmp_conf, dst_conf)?;
    fs::remove_file(bk_conf)?;

    Ok(())
}
