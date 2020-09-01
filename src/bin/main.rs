#[allow(unused_imports)]
#[macro_use]
extern crate log;

use dfpm::config::{current_configuration, LayerKind};
use dfpm::name::Name;
use dfpm::package::{get_packages, Package};
use dfpm::{Installable, Options};
use serde::export::Formatter;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use structopt::StructOpt;
use text_trees::StringTreeNode;

#[derive(Clone, Debug)]
enum ConfigKind {
    Platform,
    Layers,
    Installers,
}

#[derive(Clone, Debug)]
enum Scope {
    Package,
    Layer,
    All,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "metapack", about = "Meta-package manager")]
struct CommandLine {
    /// The level of logging to perform; from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    #[structopt(long, short)]
    dry_run: bool,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    /// Show the current configuration settings
    Config,
    /// Install a meta-package
    Install {
        /// Installer scope: 'package', 'layer', 'all'
        #[structopt(long, short)]
        scope: Scope,

        /// The package, or layer, to act upon
        #[structopt(name = "NAME")]
        names: Vec<Name>,
    },
    /// Update an installed meta-package
    Update {
        /// Installer scope: 'platform', 'layer', 'all'
        #[structopt(long, short)]
        scope: Scope,

        /// The package, or layer, to act upon
        #[structopt(name = "NAME")]
        names: Vec<Name>,
    },
    /// Delete an installed meta-package
    Delete {
        /// Installer scope: 'platform', 'layer', 'all'
        #[structopt(long, short)]
        scope: Scope,

        /// The package, or layer, to act upon
        #[structopt(name = "NAME")]
        names: Vec<Name>,
    },
    /// Inspect meta-package definition file
    Inspect {
        /// The package to act upon
        #[structopt(name = "NAME")]
        package: Option<Name>,
    },
}

impl Display for ConfigKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConfigKind::Platform => "platform",
                ConfigKind::Layers => "layers",
                ConfigKind::Installers => "installers",
            }
        )
    }
}

impl FromStr for ConfigKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "platform" => Ok(ConfigKind::Platform),
            "layers" => Ok(ConfigKind::Layers),
            "installers" => Ok(ConfigKind::Installers),
            _ => Err(format!("invalid value '{}' for config kind", s)),
        }
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Scope::Package => "package",
                Scope::Layer => "layer",
                Scope::All => "all",
            }
        )
    }
}

impl FromStr for Scope {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "package" => Ok(Scope::Package),
            "layer" => Ok(Scope::Layer),
            "all" => Ok(Scope::All),
            _ => Err(format!("invalid value '{}' for installer scope", s)),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLine::from_args();

    pretty_env_logger::formatted_builder()
        .filter_level(match args.verbose {
            0 => log::LevelFilter::Off,
            1 => log::LevelFilter::Error,
            2 => log::LevelFilter::Warn,
            3 => log::LevelFilter::Info,
            4 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();

    let loaded_config = match current_configuration() {
        Ok(config) => config,
        Err(kind) => {
            let err: dfpm::error::Error = kind.to_string().into();
            return Err(Box::new(err));
        }
    };

    let install_options = Options {
        dry_run: args.dry_run,
    };

    match args.cmd {
        SubCommand::Config => {
            let mut root: StringTreeNode = "<config>".into();

            let mut platform_root: StringTreeNode = "<platform>".into();
            platform_root.push(format!(
                "{} ({})",
                loaded_config.platform().name,
                dfpm::config::platforms::current_name().unwrap()
            ));
            root.push_node(platform_root);

            let mut layer_root: StringTreeNode = "<layers>".into();
            for layer in loaded_config.layers() {
                let node: StringTreeNode = match layer.as_ref() {
                    LayerKind::Outer { name, sub_layers } => {
                        let mut node: StringTreeNode = name.to_string().into();
                        for layer in sub_layers {
                            node.push(layer.name().to_string().into())
                        }
                        node
                    }
                    LayerKind::Inner { name } => name.to_string().into(),
                };
                layer_root.push_node(node);
            }
            root.push_node(layer_root);

            let platform_installer = &loaded_config.platform().system_installer.as_ref().unwrap();
            let mut installer_root: StringTreeNode = "<installers>".into();
            for (id, installer) in loaded_config.installers() {
                installer_root.push(format!(
                    "{} ({}{})",
                    installer.name,
                    if &id == platform_installer { "*" } else { "" },
                    id,
                ));
            }
            root.push_node(installer_root);

            root.write(&mut std::io::stdout())?;
        }
        SubCommand::Install { scope, names } => match scope {
            Scope::Package => {
                info!("SubCommand::Install >> Scope::Package {:?}", names);
                for name in names {
                    let package = Package::load(&name)?;
                    package.install(&install_options)?;
                }
            }
            Scope::Layer => {
                info!("SubCommand::Install >> Scope::Layer {:?}", names);
                for layer in loaded_config.layers() {
                    match layer.as_ref() {
                        LayerKind::Outer {
                            name: _,
                            sub_layers,
                        } => {
                            debug!(
                                "{:?} ∩ {:?}?",
                                names,
                                sub_layers
                                    .iter()
                                    .map(|layer| layer.name())
                                    .collect::<Vec<&Name>>(),
                            );
                            for layer in sub_layers {
                                if names.contains(&layer.name()) {
                                    layer.install(&install_options)?;
                                }
                            }
                        }
                        LayerKind::Inner { name } => {
                            debug!("{:?} ∋ {:?}?", name, names);
                            if names.contains(&name) {
                                layer.install(&install_options)?;
                            }
                        }
                    }
                }
            }
            Scope::All => {
                info!("SubCommand::Install >> Scope::All");
                for layer in loaded_config.layers() {
                    layer.install(&install_options)?;
                }
            }
        },
        SubCommand::Delete { scope, names } => match scope {
            Scope::Package => {
                for name in names {
                    let package = Package::load(&name)?;
                    package.delete(&install_options)?;
                }
            }
            Scope::Layer => {
                for layer in loaded_config.layers() {
                    if names.contains(&layer.name()) {
                        layer.delete(&install_options)?;
                    }
                }
            }
            Scope::All => {
                for layer in loaded_config.layers() {
                    layer.delete(&install_options)?;
                }
            }
        },
        SubCommand::Update { scope, names } => match scope {
            Scope::Package => {
                for name in names {
                    let package = Package::load(&name)?;
                    package.update(&install_options)?;
                }
            }
            Scope::Layer => {
                for layer in loaded_config.layers() {
                    if names.contains(&layer.name()) {
                        layer.update(&install_options)?;
                    }
                }
            }
            Scope::All => {
                for layer in loaded_config.layers() {
                    layer.update(&install_options)?;
                }
            }
        },
        SubCommand::Inspect { package } => match package {
            Some(package) => {
                let package = Package::load(&package)?;
                package.inspect(&mut std::io::stdout(), true)?;
            }
            None => {
                let packages = get_packages()?;
                for (_, package) in packages {
                    package.inspect(&mut std::io::stdout(), true)?;
                }
            }
        },
    }

    Ok(())
}
