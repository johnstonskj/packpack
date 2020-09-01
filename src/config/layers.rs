use crate::error::ErrorKind;
use crate::name::Name;
use crate::package::get_packages;
use crate::{Installable, Options};
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Layer {
    pub name: Name,
    #[serde(default)]
    pub sub_layers: Vec<Name>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Layers {
    pub layers: Vec<Layer>,
}

#[derive(Clone, Debug)]
pub enum LayerKind {
    Outer {
        name: Name,
        sub_layers: Vec<Box<LayerKind>>,
    },
    Inner {
        name: Name,
    },
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn load_config() -> Result<Vec<Box<LayerKind>>, ErrorKind> {
    info!("load_config()");
    let mut layers = load_default_config()?;
    layers.layers.extend(load_user_config()?.layers);
    debug!(".. loaded {} layer configs", layers.layers.len());
    Ok(layers
        .layers
        .drain(..)
        .map(|mut layer| {
            Box::new(LayerKind::Outer {
                name: layer.name,
                sub_layers: layer
                    .sub_layers
                    .drain(..)
                    .map(|name| Box::new(LayerKind::Inner { name }))
                    .collect(),
            })
        })
        .collect())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Installable for LayerKind {
    fn install(&self, options: &Options) -> crate::error::Result<()> {
        info!("Layer::install({:?}) for {:?}", options, self.name());
        let packages = get_packages().unwrap();
        let names: Vec<&Name> = match self {
            Self::Outer {
                name: _,
                sub_layers,
            } => sub_layers
                .iter()
                .filter_map(|l| {
                    if let Self::Inner { name } = l.as_ref() {
                        Some(name)
                    } else {
                        None
                    }
                })
                .collect(),
            Self::Inner { name } => vec![name],
        };
        for name in names {
            for package in packages.iter().map(|(_, p)| p).filter(|p| &p.layer == name) {
                package.install(options)?;
            }
        }
        Ok(())
    }

    fn update(&self, options: &Options) -> crate::error::Result<()> {
        info!("Layer::update({:?} for {:?})", options, self.name());
        let packages = get_packages().unwrap();
        for (_, package) in packages {
            if &package.layer == self.name() {
                package.update(options)?;
            }
        }
        Ok(())
    }

    fn delete(&self, options: &Options) -> crate::error::Result<()> {
        info!("Layer::delete({:?} for {:?})", options, self.name());
        let packages = get_packages().unwrap();
        for (_, package) in packages {
            if &package.layer == self.name() {
                package.delete(options)?;
            }
        }
        Ok(())
    }
}

impl LayerKind {
    pub fn name(&self) -> &Name {
        match self {
            Self::Outer {
                name,
                sub_layers: _,
            } => &name,
            Self::Inner { name } => &name,
        }
    }

    pub fn find(&self, a_name: &Name) -> Option<Box<&Self>> {
        match self {
            Self::Outer {
                name: _,
                sub_layers,
            } => sub_layers
                .iter()
                .filter_map(|layer| layer.find(a_name))
                .next(),
            Self::Inner { name } => {
                if a_name == name {
                    Some(Box::new(self))
                } else {
                    None
                }
            }
        }
    }

    pub fn is_outer(&self) -> bool {
        match self {
            Self::Outer {
                name: _,
                sub_layers: _,
            } => true,
            _ => false,
        }
    }
    pub fn is_inner(&self) -> bool {
        match self {
            Self::Inner { name: _ } => true,
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const DEFAULT_LAYER_CONFIG: &str = include_str!("layers.toml");

fn load_default_config() -> Result<Layers, ErrorKind> {
    info!("> load_default_layer_config()");
    super::config_from_string(DEFAULT_LAYER_CONFIG)
}

fn load_user_config() -> Result<Layers, ErrorKind> {
    info!("> load_user_layer_config()");
    if let Some(content) = super::config_from_user_file("layers")? {
        super::config_from_string(&content)
    } else {
        Ok(Layers { layers: vec![] })
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
    fn serialize_layers() {
        let layers = vec![
            Layer {
                name: "bootstrap".parse().unwrap(),
                sub_layers: vec![],
            },
            Layer {
                name: "system".parse().unwrap(),
                sub_layers: vec![
                    "package-managers".parse().unwrap(),
                    "shell".parse().unwrap(),
                    "fonts".parse().unwrap(),
                    "secure-tools".parse().unwrap(),
                    "other-tools".parse().unwrap(),
                ],
            },
        ];
        println!("{}", toml::to_string(&Layers { layers }).unwrap());
    }
}
