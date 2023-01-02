use std::{io::ErrorKind, str::FromStr};

use serde_derive::Deserialize;
#[derive(PartialEq, Deserialize)]
pub enum OSType {
    Linux,
    MacOS,
}

impl FromStr for OSType {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if *s == *"linux" {
            return Ok(OSType::Linux);
        }
        if *s == *"macos" {
            return Ok(OSType::MacOS);
        }
        return Err(ErrorKind::InvalidInput);
    }
}
