use std::fmt::Display;

use crate::addr::Addr;
use crate::data::Data;

#[derive(Debug)]
pub struct Elem {
	addr: Addr,
	data: Data,
	sync: bool,
}

impl Elem {
	pub fn new(addr: Addr, data: Data) -> Self {
		Self {
			addr,
			data,
			// newly constructed element must be synced with main memory
			sync: true,
		}
	}

	#[inline]
	pub fn desync(&mut self) {
		self.sync = false;
	}

	#[inline]
	pub fn data(&self) -> &Data {
		&self.data
	}

	#[inline]
	pub fn addr(&self) -> Addr {
		self.addr
	}
}

impl Display for Elem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} {}", self.addr, self.data, self.sync)
	}
}
