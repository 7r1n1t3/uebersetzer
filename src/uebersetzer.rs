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
    #[error("filesystem error: {0}")]
    Io(#[from] io::Error),
    #[error("tera error: {0}")]
    Tera(#[from] tera::Error),
}

pub enum UebersetzState {
    Written,
    UnchangedContent,
    ExistsNoForceWrite,
}

pub fn uebersetz(
    conf_path: &Path,
    env_vars: &HashMap<String, String>,
    recursive: Option<bool>,
    force_write: Option<bool>,
) -> Result<(), UebersetzError> {
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
) -> Result<UebersetzState, UebersetzError> {
    let force_write = force_write.unwrap_or(false);

    let dst_conf_filename: String = src_conf
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .replace(UEBER_FILE_EXTENSION, "");
    let dst_conf: PathBuf = src_conf.with_file_name(dst_conf_filename);

    if dst_conf.is_file() && !force_write {
        return Ok(UebersetzState::ExistsNoForceWrite);
    }

    let mut tera = Tera::default();
    let mut context = Context::new();
    load_env_to_tera_context(env_vars, &mut context);

    tera.add_template_file(src_conf, Some("conf"))?;
    let rendered = tera.render("conf", &context)?;

    if rendered == get_file_content(dst_conf.as_path())? {
        // skip if content of file didn't change
        debug!(
            "rendered content is same as file content: {:?}, skipping",
            dst_conf
        );
        return Ok(UebersetzState::UnchangedContent);
    }

    let tmp_conf: PathBuf = dst_conf.with_added_extension(TMP_FILE_EXTENSION);
    fs::write(&tmp_conf, rendered)?;
    debug!("writing to: {:?} ", dst_conf);

    fs::rename(tmp_conf, dst_conf)?;

    Ok(UebersetzState::Written)
}

fn get_file_content(file: &Path) -> Result<String, io::Error> {
    fs::read_to_string(file)
}
