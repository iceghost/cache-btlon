use anyhow::Result;

use std::num::ParseIntError;

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Addr(usize);

impl From<usize> for Addr {
    fn from(int: usize) -> Self {
        Self(int)
    }
}

impl FromStr for Addr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
