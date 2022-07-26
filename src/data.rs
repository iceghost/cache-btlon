use std::str::FromStr;

use crate::addr::Addr;

#[derive(Debug)]
pub enum Data {
    Float(f32),
    Int(i32),
    Bool(bool),
    Addr(Addr),
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "false" || s == "true" {
            Ok(Self::Bool(s == "true"))
        } else if let Ok(int) = s.parse::<i32>() {
            Ok(Self::Int(int))
        } else if let Ok(float) = s.parse::<f32>() {
            Ok(Self::Float(float))
        } else {
            let s = &s[..s.len() - 1];
            let addr = Addr::from(s.parse::<usize>()?);
            Ok(Self::Addr(addr))
        }
    }
}
