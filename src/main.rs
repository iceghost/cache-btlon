use std::str::FromStr;

use addr::Addr;
use anyhow::Result;
use data::Data;

use crate::instruction::Instruction;

mod addr;
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

trait Cache {
    fn read(&self, addr: Addr) -> Option<&Data>;
    fn put(&mut self, addr: Addr, data: Data) -> Option<elem::Elem>;
    fn write(&mut self, addr: Addr, data: Data) -> Option<elem::Elem>;

    // print elements of the cache, from youngest to eldest
    fn print(&self) -> &str;

    // print the BST of the cache
    fn inorder(&self) -> &str;
    fn preorder(&self) -> &str;
}
