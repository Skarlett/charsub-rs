#![feature(generic_associated_types)]

mod patterns;
mod unit;
mod cursor;
mod api;
mod scheduler;


//pub use api::*;

use smallvec::SmallVec;

type Cell = SmallVec<[u8; 32]>;
pub type RuleCell = SmallVec<[u8; 8]>;
