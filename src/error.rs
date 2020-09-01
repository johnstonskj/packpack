/*!
One-line description.

More detailed description, with

# Example

*/

use crate::name::Name;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
        #[doc("Invalid Name, either empty or contains incorrect characters")]
        InvalidName(v: String) {
            description("Invalid Name, either empty or contains incorrect characters")
            display("Invalid Name '{}', either empty or contains incorrect characters", v)
        }
        #[doc("Could not parse configuration file")]
        InvalidConfigFormat {
            description("Could not parse configuration file")
            display("Could not parse configuration file")
        }
        #[doc("Package `layer` is either invalid, or is an outer layer")]
        InvalidLayerInPackage {
            description("Package `layer` is either invalid, or is an outer layer")
            display("Package `layer` is either invalid, or is an outer layer")
        }
         #[doc("Either DFML_HOME note set, or no package directory found there")]
        MissingPackageDir {
            description("Either DFML_HOME note set, or no package directory found there")
            display("Either DFML_HOME note set, or no package directory found there")
        }
        #[doc("No package found with the provided name")]
        NoSuchPackage(n: Name) {
            description("No package found with the provided name")
            display("No package found with the provided name '{}'", n)
        }
        #[doc("Current platform is unsupported (determined by target_os)")]
        UnsupportedPlatform {
            description("Current platform is unsupported (determined by target_os)")
            display("Current platform is unsupported (determined by target_os)")
        }
        #[doc("Installer may not use the name 'system'")]
        SystemInstallerName {
            description("Installer may not use the name 'system'")
            display("Installer may not use the name 'system'")
        }
        #[doc("No system installer present for current platform")]
        MissingSystemInstaller {
            description("No system installer present for current platform")
            display("No system installer present for current platform")
        }
        #[doc("A required command was not found")]
        MissingRequiredCommand {
            description("A required command was not found")
            display("A required command was not found")
        }
    }

    foreign_links {
        Deserialize(::toml::de::Error);
        Environment(::std::env::VarError);
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Serialize(::toml::ser::Error);
        Unexpected(::std::convert::Infallible);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
