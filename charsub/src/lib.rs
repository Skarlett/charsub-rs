#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
mod patterns;
mod unit;
mod cursor;
mod api;
pub mod scheduler;

pub use scheduler::Scheduler;
pub use unit::Permutation;
pub use api::*;
pub use patterns::*;

use smallvec::SmallVec;

pub type Cell = SmallVec<[u8; 32]>;
pub type RuleCell = SmallVec<[u8; 8]>;
