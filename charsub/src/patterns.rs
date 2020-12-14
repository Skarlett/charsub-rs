use std::{
    collections::{HashMap},
};
use crate::{
    unit_set::Cursor,
    RuleCell,
    unit::UnitPair
};

pub struct Rule<'r> {
    pub idx: usize,
    pub inner: &'r RuleEntry
}

impl<'r> From<(usize, &'r RuleEntry)> for Rule<'r> {
    fn from(data: (usize, &'r RuleEntry)) -> Self {
        Self {
            idx: data.0,
            inner: data.1
        }
    }
}

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