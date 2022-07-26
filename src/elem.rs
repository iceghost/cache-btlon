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

	pub fn desync(&mut self) {
		self.sync = false;
	}
}
