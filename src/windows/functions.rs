use clap::Parser;

use crate::common::{Browser, FilePath, Publisher};
use crate::error::{ExtError, Result, convert_result};
use std::path::PathBuf;

pub(crate) fn get_chromium_root(publisher: Publisher, browser: Browser) -> Result<FilePath> {
    match crate::interface::Interface::parse().override_localappdata_path {
        Some(path) => Ok(PathBuf::from(format!(
            "{}\\{}\\{}\\User Data\\Default\\extensions",
            path, publisher, browser
        ))
        .into_boxed_path()),
        None => Ok(PathBuf::from(format!(
            "{}\\{}\\{}\\User Data\\Default\\extensions",
            convert_result(std::env::var("LOCALAPPDATA"), ExtError::EnvironmentVarError)?,
            publisher,
            browser
        ))
        .into_boxed_path()),
    }
}

pub(crate) fn get_firefox_root() -> Result<FilePath> {
    match crate::interface::Interface::parse().override_appdata_path {
        Some(path) => {
            Ok(PathBuf::from(format!("{}\\Mozilla\\Firefox\\Profiles", path)).into_boxed_path())
        }
        None => Ok(PathBuf::from(format!(
            "{}\\Mozilla\\Firefox\\Profiles",
            convert_result(std::env::var("APPDATA"), ExtError::EnvironmentVarError)?
        ))
        .into_boxed_path()),
    }
}
