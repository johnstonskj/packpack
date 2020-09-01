/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::ErrorKind;
use crate::name::Name;
use serde::Deserialize;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Configuration {
    platform: CurrentPlatform,
    layers: Vec<Box<LayerKind>>,
    installers: Installers,
}

type ConfigResult = Result<Configuration, ErrorKind>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref CURRENT_CONFIG: ConfigResult = load_config();
}

pub fn current_configuration() -> &'static ConfigResult {
    info!("current_configuration()");
    &CURRENT_CONFIG
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Configuration {
    pub fn platform(&self) -> &CurrentPlatform {
        &self.platform
    }

    pub fn layers(&self) -> impl Iterator<Item = &Box<LayerKind>> {
        self.layers.iter()
    }

    pub fn installers(&self) -> impl Iterator<Item = (&Name, &Installer)> {
        self.installers.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn load_config() -> ConfigResult {
    info!("load_config()");

    let layers = layers::load_config()?;

    let platform = platforms::get_current_platform_config()?;

    let mut installers = installers::load_config()?;
    trace!("> remove installers that are not supported on the current platform");
    installers.retain(|_, i| i.platforms.is_empty() || i.platforms.contains(&platform.target_os));

    trace!("> ensure the selected platform config points to a valid system installer");
    if let Some(name) = &platform.system_installer {
        if !installers.contains_key(name) {
            error!(
                "platform {} could not find system installer for {:?}",
                &platform.name,
                installers.keys()
            );
            return Err(ErrorKind::MissingSystemInstaller);
        }
    }

    trace!("> ensure the selected platform config points to a valid application installer");
    if let Some(name) = &platform.app_installer {
        if !installers.contains_key(name) {
            error!(
                "platform {} could not find application installer for {:?}",
                &platform.name,
                installers.keys()
            );
            return Err(ErrorKind::MissingSystemInstaller);
        }
    }

    Ok(Configuration {
        layers,
        platform,
        installers,
    })
}

fn config_from_string<'a, T>(content: &'a str) -> Result<T, ErrorKind>
where
    T: Deserialize<'a>,
{
    trace!("> > config_from_string(...)");
    match toml::from_str(content) {
        Ok(config) => Ok(config),
        Err(err) => {
            error!("Serde error: {:?}", err);
            Err(ErrorKind::InvalidConfigFormat)
        }
    }
}

fn config_from_user_file(file_name: &str) -> Result<Option<String>, ErrorKind> {
    trace!("> > config_from_user_file({:?})", file_name);

    if let Some(file_path) = make_user_file_path(file_name) {
        if file_path.exists() && file_path.is_file() {
            match std::fs::read_to_string(file_path) {
                Ok(content) => Ok(Some(content)),
                Err(err) => {
                    error!("File read error: {:?}", err);
                    Err(ErrorKind::InvalidConfigFormat)
                }
            }
        } else {
            info!("> > .. user config file {:?} does not exist", file_path);
            Ok(None)
        }
    } else {
        warn!("> > .. unable to make a file path for user config file");
        Ok(None)
    }
}

fn make_user_file_path(base_name: &str) -> Option<PathBuf> {
    let base_path = dirs::config_dir();
    if let Some(mut file_path) = base_path {
        file_path.push(&format!("{}.toml", base_name));
        Some(file_path)
    } else {
        None
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod installers;
use installers::{Installer, Installers};

pub mod layers;
pub use layers::LayerKind;

pub mod platforms;
pub use platforms::{CurrentPlatform, Platform, Platforms};

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_load_config() {
        let result = current_configuration();
        println!("{:#?}", result);
        assert!(result.is_ok());
        let config = result.as_ref().unwrap();
        println!("{:#?}", config);
    }
}
