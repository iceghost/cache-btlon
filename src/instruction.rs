use anyhow::Result;
use std::str::FromStr;

use crate::addr::Addr;
use crate::data::Data;

#[derive(Debug)]
pub enum Instruction {
    Read(Addr, Data),
    Put(Addr, Data),
    Write(Addr, Data),
    Print,
    Inorder,
    Preorder,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inst = match s.chars().next() {
            Some('P') => Self::Print,
            Some('I') => Self::Inorder,
            Some('E') => Self::Preorder,
            Some(c) => {
                let mut parts = s.split(' ').skip(1);
                let addr = Addr::from_str(parts.next().expect("instruction requires addr"))?;
                let data = Data::from_str(parts.next().expect("instruction requires data"))?;
                match c {
                    'R' => Self::Read(addr, data),
                    'U' => Self::Put(addr, data),
                    'W' => Self::Write(addr, data),
                    _ => panic!("unrecognized instruction '{c}'"),
                }
            }
            None => todo!(),
        };
        Ok(inst)
    }
}
