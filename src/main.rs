use std::{collections::VecDeque, str::FromStr};

use addr::Addr;
use anyhow::Result;
use bst::BinarySearchTree;
use data::Data;
use elem::Elem;

use crate::instruction::Instruction;

mod addr;
mod bst;
mod data;
mod elem;
mod instruction;

fn main() -> Result<()> {
	let s = include_str!("../initial/test1.txt");
	for line in s.lines() {
		let inst = Instruction::from_str(line)?;
		println!("{inst:?}")
	}
	println!("Hello, world!");
	Ok(())
}

struct Cache {
	deque: VecDeque<(Addr, Elem)>,
	bst: BinarySearchTree<Addr, usize>,
}

impl Cache {
	fn read(&self, addr: Addr) -> Option<&Data> {
		todo!()
	}
	fn put(&mut self, addr: Addr, data: Data) -> Option<elem::Elem> {
		todo!()
	}
	fn write(&mut self, addr: Addr, data: Data) -> Option<elem::Elem> {
		todo!()
	}

	// print elements of the cache, from youngest to eldest
	fn print(&self) -> &str {
		todo!()
	}
	fn inorder(&self) -> &str {
		todo!()
	}
	fn preorder(&self) -> &str {
		todo!()
	}
}
