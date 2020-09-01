use crate::error::ErrorKind;
use crate::name::Name;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Platform {
    pub name: Name,
    #[serde(default)]
    pub system_installer: Option<Name>,
    #[serde(default)]
    pub app_installer: Option<Name>,
    #[serde(default)]
    pub determine_installer: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct CurrentPlatform {
    pub target_os: Name,
    pub name: Name,
    pub system_installer: Option<Name>,
    pub app_installer: Option<Name>,
    pub determine_installer: Option<PathBuf>,
}

pub type Platforms = HashMap<Name, Platform>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------
//
// #[cfg(target_os = "linux")]
// const PLATFORM_NAME: &str = "linux";
//
// #[cfg(target_os = "macos")]
// const PLATFORM_NAME: &str = "macos";
//
// #[cfg(target_os = "windows")]
// const PLATFORM_NAME: &str = "windows";
//
// #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
// const PLATFORM_NAME: &str = "";

#[inline]
pub fn current_name() -> Option<Name> {
    let platform_name = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        ""
    };
    info!("current_name() -> {:?}", platform_name);

    if platform_name.is_empty() {
        None
    } else {
        Some(platform_name.parse().unwrap())
    }
}

pub fn get_current_platform_config() -> Result<CurrentPlatform, ErrorKind> {
    info!("get_current_platform_config()");
    let mut platforms = load_config()?;
    if let Some(name) = current_name() {
        match platforms.remove(&name) {
            Some(platform) => Ok(CurrentPlatform {
                target_os: name,
                name: platform.name,
                system_installer: platform.system_installer,
                app_installer: platform.app_installer,
                determine_installer: platform.determine_installer,
            }),
            None => {
                warn!("No platform config for the target_os = {:?}", name);
                Err(ErrorKind::UnsupportedPlatform)
            }
        }
    } else {
        warn!("No platform config for the running O/S");
        Err(ErrorKind::UnsupportedPlatform)
    }
}

pub fn load_config() -> Result<Platforms, ErrorKind> {
    info!("load_config()");
    let mut platforms = load_default_config()?;
    platforms.extend(load_user_config()?);

    if platforms
        .iter()
        .all(|(_, p)| p.system_installer.is_some() || p.determine_installer.is_some())
    {
        debug!(".. loaded {} platform configs", platforms.len());
        Ok(platforms)
    } else {
        error!("Platforms must have either `system_installer` or `determine_installer` specified");
        Err(ErrorKind::InvalidConfigFormat)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const DEFAULT_PLATFORM_CONFIG: &str = include_str!("platforms.toml");

fn load_default_config() -> Result<Platforms, ErrorKind> {
    info!("> load_default_platform_config()");
    super::config_from_string(DEFAULT_PLATFORM_CONFIG)
}

fn load_user_config() -> Result<Platforms, ErrorKind> {
    info!("> load_user_platform_config()");
    if let Some(content) = super::config_from_user_file("platforms")? {
        super::config_from_string(&content)
    } else {
        Ok(Default::default())
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn serialize_platforms() {
        println!(
            "{}",
            toml::to_string(&Platform {
                name: "macOS".parse().unwrap(),
                system_installer: Some("homebrew".parse().unwrap()),
                app_installer: Some("homebrew-apps".parse().unwrap()),
                determine_installer: None,
            })
            .unwrap()
        );
        println!(
            "{}",
            toml::to_string(&Platform {
                name: "Linux".parse().unwrap(),
                system_installer: None,
                app_installer: None,
                determine_installer: Some("which-linux".parse().unwrap())
            })
            .unwrap()
        );
    }
}
