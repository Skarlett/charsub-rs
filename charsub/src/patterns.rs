use std::{
    collections::{HashMap},
};
use crate::{
    unit_set::Cursor,
    RuleCell,
    unit::UnitPair
};

pub trait Handler {
    fn handle(&mut self, permute: &UnitPair<'_>) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleEntry {
    Single(u8),
    Multi(RuleCell)
}

impl RuleEntry {
    pub fn len(&self) -> usize {
        match self {
            Self::Single(b) => 1,
            Self::Multi(buf) => buf.len() 
        }
    }
}

use smallvec::SmallVec;
impl From<RuleEntry> for RuleCell {
    fn from(x: RuleEntry) -> RuleCell {
        match x {
            RuleEntry::Multi(buf) => buf,
            RuleEntry::Single(byte) => { 
                let mut x = SmallVec::new();
                x.push(byte);
                x
            }
        }
    }
}


#[derive(Default)]
pub struct ConstPattern {}
impl Handler for ConstPattern {
    fn handle(&mut self, permute: &UnitPair<'_>) -> bool {
        true
    }
}

#[derive(Default)]
pub struct ModulusPattern {}
impl Handler for ModulusPattern {
    fn handle(&mut self, permute: &UnitPair) -> bool {
        permute.index() % permute.len() == 0
    }
}