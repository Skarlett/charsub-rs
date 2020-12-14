mod patterns;
mod unit;
mod unit_set;
use smallvec::SmallVec;
pub use patterns::{Handler, RuleEntry, ConstPattern, ModulusPattern};
pub use unit_set::Generator;
pub use unit::UnitPair;

pub type Cell = SmallVec<[u8; 32]>;
pub type RuleCell = SmallVec<[u8; 8]>;



#[cfg(test)]
mod Tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn name() {
        
        //patterns::Handler::new(patterns::Pattern::Const, unimplemented!());

    }
}
