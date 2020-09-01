/*!
One-line description.

More detailed description, with

# Example

*/

use crate::config::current_configuration;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::name::Name;
use crate::package::install::Action;
use crate::{Installable, Options};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Package {
    #[serde(skip)]
    pub(crate) path: PathBuf,
    pub name: Name,
    pub layer: Name,
    pub packages: Vec<InnerPackage>,
    #[serde(default)]
    pub on_install: Option<ScriptSet>,
    #[serde(default)]
    pub on_update: Option<ScriptSet>,
    #[serde(default)]
    pub on_delete: Option<ScriptSet>,
    #[serde(default)]
    pub requires: Vec<String>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[repr(u8)]
pub enum InnerPackagePriority {
    Lowest,
    Lower,
    Normal,
    Higher,
    Highest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InnerPackage {
    pub name: Name,
    pub installer: Name,
    #[serde(default)]
    pub priority: InnerPackagePriority,
    #[serde(default)]
    pub is_app: bool,
    #[serde(default)]
    pub additional_arguments: Vec<String>,
    #[serde(default)]
    pub platforms: Vec<Name>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScriptSet {
    #[serde(default)]
    pub before: Option<PathBuf>,
    #[serde(default)]
    pub after: Option<PathBuf>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub const DFPM_HOME: &str = "DFPM_HOME";

pub fn get_package_dir() -> Result<PathBuf> {
    let path = std::env::var(DFPM_HOME)?;
    let path = PathBuf::from_str(&path)?;
    if path.exists() && path.is_dir() {
        Ok(path)
    } else {
        println!("{:?} {} {}", path, path.exists(), path.is_dir());
        Err(crate::error::ErrorKind::MissingPackageDir.into())
    }
}

pub fn get_packages() -> Result<HashMap<Name, Package>> {
    let parent_path = get_package_dir()?;
    let mut packages: HashMap<Name, Package> = Default::default();
    for entry in std::fs::read_dir(parent_path)? {
        let entry = entry?;
        let package_path = entry.path();
        if package_path.is_dir() {
            let package_file = package_path.join("package.toml");
            if package_file.exists() && package_file.is_file() {
                let package = Package::load_from(&package_file)?;
                packages.insert(package.name.clone(), package);
            }
        }
    }
    Ok(packages)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Installable for Package {
    fn install(&self, options: &Options) -> Result<()> {
        info!("Package::install({:?})", options);
        install::install_action(Action::Install, self, options)
    }

    fn update(&self, options: &Options) -> Result<()> {
        info!("Package::update({:?})", options);
        install::install_action(Action::Update, self, options)
    }

    fn delete(&self, options: &Options) -> Result<()> {
        info!("Package::delete({:?})", options);
        install::install_action(Action::Delete, self, options)
    }
}

impl Package {
    pub fn load(name: &Name) -> Result<Self> {
        info!("Package::load({})", name);
        let mut path = get_package_dir()?;
        path.push(&name.to_string());
        path.push("package.toml");
        if path.exists() && path.is_file() {
            debug!(".. loading from file {:?}", path);
            Self::load_from(&path)
        } else {
            info!(".. package file {:?} does not exist", path);
            Err(crate::error::ErrorKind::NoSuchPackage(name.clone()).into())
        }
    }

    fn load_from(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path.clone())?;
        let mut package: Package = toml::from_str(&content)?;

        package.path = path.clone();

        let config = current_configuration().as_ref().unwrap();
        let layer = config
            .layers()
            .filter_map(|layer| layer.find(&package.layer))
            .next();
        match layer {
            Some(layer) => {
                if layer.is_inner() {
                    Ok(package)
                } else {
                    error!("layer {} is an outer layer", package.layer);
                    Err(ErrorKind::InvalidLayerInPackage.into())
                }
            }
            None => {
                error!("layer {} does not exist", package.layer);
                Err(ErrorKind::InvalidLayerInPackage.into())
            }
        }
    }

    pub fn inspect(&self, w: &mut impl Write, colored: bool) -> Result<()> {
        info!("Package::inspect(..., {})", colored);
        inspect::inspect_package(self, w, colored)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for InnerPackagePriority {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for InnerPackagePriority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InnerPackagePriority::Highest => "highest",
                InnerPackagePriority::Higher => "higher",
                InnerPackagePriority::Normal => "normal",
                InnerPackagePriority::Lower => "lower",
                InnerPackagePriority::Lowest => "lowest",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod inspect;

mod install;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    fn set_test_package_root() -> String {
        let pkgs_dir = format!("{}/test-pkgs", env!("CARGO_MANIFEST_DIR"));
        std::env::set_var(DFPM_HOME, pkgs_dir.clone());
        pkgs_dir
    }

    #[test]
    fn test_get_package_dir() {
        let pkgs_dir = set_test_package_root();

        let package_dir = get_package_dir();
        println!("{:#?}", package_dir);
        assert!(package_dir.is_ok());
        assert_eq!(package_dir.unwrap(), PathBuf::from_str(&pkgs_dir).unwrap())
    }

    #[test]
    fn test_get_packages() {
        let _ = set_test_package_root();

        let pkgs = get_packages();
        println!("{:#?}", pkgs);
        assert!(pkgs.is_ok());
        let pkgs = pkgs.unwrap();
        assert_eq!(pkgs.len(), 1);
        let package = pkgs.get(&("Rust".parse().unwrap())).unwrap();
        assert_eq!(package.name, Name::from_str("Rust").unwrap());
    }

    #[test]
    fn test_load_package() {
        let _ = set_test_package_root();

        let result = Package::load(&"Rust".parse().unwrap());
        assert!(result.is_ok());
        let package = result.unwrap();
        println!("**********");
        let result = package.inspect(&mut std::io::stdout(), true);
        println!("**********");
        assert!(result.is_ok());
    }

    #[test]
    fn test_priority_order() {
        assert!(InnerPackagePriority::Highest > InnerPackagePriority::Higher);
        assert!(InnerPackagePriority::Higher > InnerPackagePriority::Normal);
        assert!(InnerPackagePriority::Normal > InnerPackagePriority::Lower);
        assert!(InnerPackagePriority::Lower > InnerPackagePriority::Lowest);
    }
}
