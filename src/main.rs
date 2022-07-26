use std::{collections::VecDeque, str::FromStr};

use addr::Addr;
use anyhow::Result;
use bst::BinarySearchTree;
use data::Data;
use elem::Elem;
use log::debug;

use crate::{
	bst::{IntoInorderIter, IntoPreorderIter},
	instruction::Instruction,
};

mod addr;
mod bst;
mod data;
mod elem;
mod instruction;

fn main() -> Result<()> {
	pretty_env_logger::init();
	let s = include_str!("../initial/test1.txt");
	let mut cache = Cache::with_capacity(15);
	for line in s.lines() {
		let inst = Instruction::from_str(line)?;
		debug!("Instruction: {inst:?}");
		match inst {
			Instruction::Read(addr, default) => match cache.read(&addr) {
				Some(data) => {
					println!("{data}");
				}
				None => {
					cache.put(addr, default);
				}
			},
			Instruction::Put(addr, data) => {
				cache.put(addr, data);
			}
			Instruction::Write(addr, data) => {
				cache.write(addr, data);
			}
			Instruction::Print => {
				println!("Print stack");
				for elem in cache.iter() {
					println!("{}", elem);
				}
			}
			Instruction::Inorder => {
				println!("Print BST in inorder");
				for elem in cache.inorder_iter() {
					println!("{}", elem);
				}
			}
			Instruction::Preorder => {
				println!("Print BST in preorder");
				for elem in cache.preorder_iter() {
					println!("{}", elem);
				}
			}
		}
	}
	Ok(())
}

struct Cache {
	deque: VecDeque<*const Elem>,
	bst: BinarySearchTree<Addr, *mut Elem>,
	capacity: usize,
}

impl Cache {
	fn with_capacity(capacity: usize) -> Self {
		Self {
			deque: VecDeque::with_capacity(capacity),
			bst: BinarySearchTree::default(),
			capacity,
		}
	}

	fn read(&self, addr: &Addr) -> Option<&Data> {
		unsafe { self.bst.get(addr).map(|&elem| (*elem).data()) }
	}

	fn elem_mut(&mut self, addr: &Addr) -> Option<&mut Elem> {
		unsafe { self.bst.get_mut(addr).map(|&mut ptr| &mut *ptr) }
	}

	fn put(&mut self, addr: Addr, data: Data) -> Option<Box<Elem>> {
		let elem = Box::new(Elem::new(addr, data));
		let ptr = Box::into_raw(elem);

		let ejected = match self.bst.get(&addr) {
			Some(&ejected) => {
				// painstakingly iter through the deque to find the replaced pointer
				let index = self
					.deque
					.iter()
					.enumerate()
					.find_map(|(i, &current_ptr)| (current_ptr == ejected).then_some(i))
					.unwrap();
				self.bst.set(addr, ptr);
				self.deque[index] = ptr;

				Some(ejected)
			}
			None => {
				let ejected = if self.deque.len() >= self.capacity {
					let mut ejected = if addr.is_even() {
						self.deque.pop_back()
					} else {
						self.deque.pop_front()
					};
					let ejected = ejected.take().unwrap();
					unsafe {
						debug!("{ejected:p} is deleted from BST");
						self.bst.delete(&(*ejected).addr())
					}
				} else {
					None
				};
				self.deque.push_back(ptr);
				self.bst.set(addr, ptr);

				ejected
			}
		};

		unsafe {
			ejected.map(|ptr| {
				debug!("{ptr:p}, {} got freed", (*ptr).addr());
				Box::from_raw(ptr)
			})
		}
	}

	fn write(&mut self, addr: Addr, data: Data) -> Option<Box<Elem>> {
		let ejected = self.put(addr, data);
		if let Some(elem) = self.elem_mut(&addr) {
			elem.desync();
		}
		ejected
	}

	fn iter(&self) -> impl Iterator<Item = &Elem> {
		unsafe { self.deque.iter().rev().map(|&ptr| &*ptr) }
	}

	fn inorder_iter(&self) -> impl Iterator<Item = &Elem> {
		unsafe { self.bst.inorder_iter().map(|(_, &ptr)| &*ptr) }
	}

	fn preorder_iter(&self) -> impl Iterator<Item = &Elem> {
		unsafe { self.bst.preorder_iter().map(|(_, &ptr)| &*ptr) }
	}
}

impl Drop for Cache {
	fn drop(&mut self) {
		for &ptr in &self.deque {
			let ptr = ptr as *mut Elem;
			unsafe { drop(Box::from_raw(ptr)) }
		}
	}
}
