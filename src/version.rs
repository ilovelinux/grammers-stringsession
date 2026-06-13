use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct VersionError;

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid version")
    }
}

impl error::Error for VersionError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Version {
    #[default]
    V1 = 0,
}

impl TryFrom<u8> for Version {
    type Error = VersionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::V1),
            _ => Err(VersionError),
        }
    }
}

impl From<Version> for u8 {
    fn from(version: Version) -> Self {
        version as Self
    }
}
