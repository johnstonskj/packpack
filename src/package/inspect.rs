use crate::error::Result;
use crate::package::{InnerPackagePriority, Package, ScriptSet};
use std::borrow::Cow;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn inspect_package(package: &Package, w: &mut impl Write, colored: bool) -> Result<()> {
    debug!(
        "> inspect_package({:?}, .., {})",
        package.name.to_string(),
        colored
    );
    writeln!(w, "{}", bold_string("File", colored))?;
    writeln!(w, "\t{}", package.path.to_string_lossy())?;

    writeln!(w, "{}", bold_string("Name", colored))?;
    writeln!(
        w,
        "\t{}, in layer {}",
        package.name.to_string(),
        package.layer.to_string()
    )?;

    if !package.requires.is_empty() {
        writeln!(w, "{}", bold_string("Requires commands", colored))?;
        for command in &package.requires {
            writeln!(w, "* {}", command)?;
        }
    }

    if let Some(script_set) = &package.on_install {
        writeln!(w, "{}", bold_string("On Install", colored))?;
        write_scripts(w, script_set)?;
    }

    if let Some(script_set) = &package.on_update {
        writeln!(w, "{}", bold_string("On Update", colored))?;
        write_scripts(w, script_set)?;
    }

    if let Some(script_set) = &package.on_delete {
        writeln!(w, "{}", bold_string("On Delete", colored))?;
        write_scripts(w, script_set)?;
    }

    writeln!(w, "{}", bold_string("Packages", colored))?;
    let mut packages = package.packages.clone();
    packages.sort_by(|a, b| b.priority.cmp(&a.priority));
    for package in packages {
        writeln!(w, "* {} ({})", package.name, package.installer)?;
        if !package.additional_arguments.is_empty() {
            writeln!(
                w,
                "  * additional_arguments: {:?}",
                package.additional_arguments
            )?;
        }
        if package.priority != InnerPackagePriority::Normal {
            writeln!(w, "  * priority: {:?}", package.priority)?;
        }
        if !package.platforms.is_empty() {
            writeln!(w, "  * only for platforms: {:?}", package.platforms)?;
        }
    }

    writeln!(w)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn bold_string(s: &str, colored: bool) -> Cow<str> {
    use ansi_term::Style;
    if colored {
        Style::new().bold().paint(s).to_string().into()
    } else {
        s.into()
    }
}

fn write_scripts(w: &mut impl Write, script_set: &ScriptSet) -> Result<()> {
    if let Some(script) = &script_set.before {
        writeln!(
            w,
            "\tbefore packages, run script: {}",
            script.to_string_lossy()
        )?;
    }
    if let Some(script) = &script_set.after {
        writeln!(
            w,
            "\tafter packages, run script: {}",
            script.to_string_lossy()
        )?;
    }
    Ok(())
}
