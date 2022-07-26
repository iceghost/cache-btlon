use anyhow::Result;

use std::fmt::Display;
use std::num::ParseIntError;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Addr(usize);

impl Addr {
	#[inline]
	pub fn is_odd(&self) -> bool {
		self.0 % 2 == 0
	}

	#[inline]
	pub fn is_even(&self) -> bool {
		!self.is_odd()
	}
}

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

impl Display for Addr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}
