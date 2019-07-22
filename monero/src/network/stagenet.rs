use super::*;
use crate::address::Format;
use wagu_model::{AddressError, Network, NetworkError};

use serde::Serialize;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Stagenet;

impl Network for Stagenet {}

impl MoneroNetwork for Stagenet {
    /// Returns the address prefix of the given network.
    /// https://github.com/monero-project/monero/blob/3ad4ecd4ff52f011ee94e0e80754b965b82f072b/src/cryptonote_config.h#L182&L184
    fn to_address_prefix(format: &Format) -> u8 {
        match format {
            Format::Standard => 24,
            Format::Integrated => 25,
            Format::Subaddress => 36
        }
    }

    /// Returns the network of the given address prefix.
    /// https://github.com/monero-project/monero/blob/3ad4ecd4ff52f011ee94e0e80754b965b82f072b/src/cryptonote_config.h#L182&L184
    fn from_address_prefix(prefix: u8) -> Result<Self, AddressError> {
        match prefix {
            24 | 25 | 36 => Ok(Self),
            _ => Err(AddressError::InvalidPrefix(vec![prefix]))
        }
    }
}

impl FromStr for Stagenet {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stagenet" => Ok(Self),
            _ => Err(NetworkError::InvalidNetwork(s.into()))
        }
    }
}

impl fmt::Display for Stagenet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "stagenet")
    }
}