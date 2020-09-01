use crate::error::ErrorKind;
use crate::name::Name;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Installer {
    pub name: Name,
    #[serde(default)]
    pub platforms: Vec<Name>,
    #[serde(default)]
    pub bootstrap: Option<String>,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub install_arguments: Vec<String>,
    #[serde(default)]
    pub update_arguments: Vec<String>,
    #[serde(default)]
    pub delete_arguments: Vec<String>,
    #[serde(default)]
    pub update_self_arguments: Vec<String>,
    #[serde(default)]
    pub requires: Vec<String>,
}

pub type Installers = HashMap<Name, Installer>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn load_config() -> Result<Installers, ErrorKind> {
    info!("load_config()");
    let mut installers = load_default_config()?;
    installers.extend(load_user_config()?);
    debug!(".. loaded {} installer configs", installers.len());
    Ok(installers)
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const DEFAULT_INSTALLER_CONFIG: &str = include_str!("installers.toml");

fn load_default_config() -> Result<Installers, ErrorKind> {
    info!("> load_default_config()");
    super::config_from_string(DEFAULT_INSTALLER_CONFIG)
}

fn load_user_config() -> Result<Installers, ErrorKind> {
    info!("> load_user_config()");
    if let Some(content) = super::config_from_user_file("installers")? {
        super::config_from_string(&content)
    } else {
        Ok(Default::default())
    }
}
