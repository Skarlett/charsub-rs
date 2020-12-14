mod patterns;
mod unit;
mod unit_set;


pub use patterns::{Handler, RuleEntry, ConstPattern, ModulusPattern};
pub use unit_set::Generator;
pub use unit::UnitPair;

use smallvec::SmallVec;
pub type Cell = SmallVec<[u8; 32]>;
pub type RuleCell = SmallVec<[u8; 8]>;
