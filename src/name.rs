/*!
One-line description.

More detailed description, with

# Example

*/

use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Name(String);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Name {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Name::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(crate::error::ErrorKind::InvalidName(s.to_string()).into())
        }
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        self.0.clone()
    }
}

impl Name {
    pub fn is_valid(s: &str) -> bool {
        !s.is_empty()
            && is_valid_first_char(s.chars().next().unwrap())
            && s[1..].chars().all(is_valid_rest_char)
    }
}

fn is_valid_first_char(c: char) -> bool {
    c.is_alphabetic()
}

fn is_valid_rest_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_' || c == '.'
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
