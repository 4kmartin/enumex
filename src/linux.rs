use std::path::PathBuf;

use crate::common::{Browser, FilePath, Publisher};
use crate::error::{ExtError, Result, convert_result};

pub(crate) fn get_chromium_root(
    publisher: Publisher,
    browser: Browser,
    path_override: &Option<String>,
) -> Result<FilePath> {
    let config = format!(
        "{}/Default/Extensions",
        get_chromium_config_path(path_override, publisher, browser)?
    );
    Ok(PathBuf::from(config).into_boxed_path())
}

fn get_chromium_config_path(
    path_override: &Option<String>,
    publisher: Publisher,
    browser: Browser,
) -> Result<String> {
    let prefix = match path_override {
        Some(path) => format!("{}/.config", path),
        None => {
            let home = convert_result(std::env::var("HOME"), ExtError::EnvironmentVarError)?;
            format!("{}/.config", home)
        }
    };
    match browser {
        Browser::Chromium => Ok(format!("{}/chromium", prefix)),
        _ => Ok(format!(
            "{}/{}-{}",
            prefix,
            publisher.to_string().to_lowercase(),
            browser.to_string().to_lowercase()
        )),
    }
}

pub(crate) fn get_firefox_root(path_override: &Option<String>) -> Result<FilePath> {
    match path_override {
        Some(path) => {
            let str_pth = format!("{}/.mozilla/firefox/Profiles", path);
            let pth = PathBuf::from(str_pth).into_boxed_path();
            Ok(pth)
        }
        None => {
            let home = convert_result(std::env::var("HOME"), ExtError::EnvironmentVarError)?;
            let path_str = format!("{}/.mozilla/firefox/Profiles", home);
            let path = PathBuf::from(path_str).into_boxed_path();
            Ok(path)
        }
    }
}
