use crate::error::Result;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::package::{InnerPackage, Package, ScriptSet};
use crate::Options;
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone, Debug)]
pub(super) enum Action {
    Install,
    Update,
    Delete,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn install_action(action: Action, package: &Package, options: &Options) -> Result<()> {
    debug!(
        "install_action({:?}, {:?}, {:?})",
        action,
        package.name.to_string(),
        options
    );

    check_requires(&package.requires, &options)?;

    do_action(
        &action,
        &package.path,
        &package.packages,
        match action {
            Action::Install => &package.on_install,
            Action::Update => &package.on_update,
            Action::Delete => &package.on_delete,
        },
        &options,
    )?;

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn check_requires(requires: &[String], _: &Options) -> Result<()> {
    debug!("> check_requires({:?}, ..)", requires);
    for required_cmd in requires {
        let cmd_path = match which::which(&required_cmd) {
            Ok(cmd_path) => cmd_path,
            Err(err) => {
                error!("error finding command '{}': {:?}", required_cmd, err);
                return Err(crate::error::ErrorKind::MissingRequiredCommand.into());
            }
        };
        trace!("> > found command {:?} at {:?})", required_cmd, cmd_path);
    }
    Ok(())
}

fn do_action(
    action: &Action,
    base_path: &PathBuf,
    packages: &[InnerPackage],
    script_set: &Option<ScriptSet>,
    options: &Options,
) -> Result<()> {
    debug!("> do_action({:?}, .., .., ..)", action);

    if let Some(script_set) = script_set {
        if let Some(path) = &script_set.before {
            run_script(base_path, path, options)?;
        }
    }

    for package in packages {
        install_inner_package(action, package, options)?;
    }

    if let Some(script_set) = script_set {
        if let Some(path) = &script_set.after {
            run_script(base_path, path, options)?;
        }
    }

    Ok(())
}

fn run_script(base_path: &PathBuf, script_path: &PathBuf, options: &Options) -> Result<()> {
    trace!("> > run_script({:?}, {:?}, ..)", base_path, script_path);
    let script_path = if script_path.is_absolute() {
        script_path.clone()
    } else {
        match base_path.parent() {
            None => script_path.clone(),
            Some(base_path) => base_path.join(script_path),
        }
    }
    .canonicalize()?;
    trace!("> > > full script path: {:?}", script_path);

    let shell = match which::which("sh") {
        Ok(cmd_path) => cmd_path,
        Err(err) => {
            error!("error finding command 'sh': {:?}", err);
            return Err(crate::error::ErrorKind::MissingRequiredCommand.into());
        }
    };
    trace!("> > > shell path: {:?}", shell);

    if options.dry_run {
        info!(
            "> > > SKIPPING `{} {:?}`",
            shell.to_string_lossy(),
            script_path
        );
    } else {
        Command::new(&shell.to_string_lossy().to_string())
            .arg(&script_path.to_string_lossy().to_string())
            .output()?;
    }
    Ok(())
}

fn install_inner_package(
    action: &Action,
    package: &InnerPackage,
    _options: &Options,
) -> Result<()> {
    trace!(
        "> > install_inner_package({:?}, {:?} ({}), ..)",
        action,
        package.name.to_string(),
        package.installer.to_string(),
    );

    // find installer

    Ok(())
}
