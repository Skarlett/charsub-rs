use crate::{
    Cell,
    RuleCell,
    patterns::RuleEntry,
    unit::Permutation,
    Rulebook
};

use hashbrown::HashMap;

#[derive(Debug, PartialEq)]
pub enum Output {
    Permute(Permutation),
    NoPermute(usize),
    EndOfLine
}

#[derive(Debug)]
pub struct Cursor<'buf, 'rules> {
    buf: &'buf Cell,
    rule_lookup: &'rules Rulebook,
    cell_idx: usize,
    reset_flag: bool
}

impl<'b, 'r> Cursor<'b, 'r> {
    pub fn new(buf: &'b Cell, rule_lookup: &'r Rulebook) -> Self {
        Self {
            buf,
            cell_idx: 0,
            rule_lookup,
            reset_flag: false
        }
    }

    pub fn buffer(&self) -> &'b Cell {
        self.buf
    }

    pub fn step(&mut self) -> Output {                
        let byte = match self.buf.get(self.cell_idx) {
            Some(byte) => *byte,
            None => {
                self.cell_idx = 0;
                self.reset_flag = true;
                return Output::EndOfLine
            }
        };

        let output = match self.rule_lookup.0.get(&byte) {
            Some(entry) => Output::Permute(Permutation::new(self.buf.clone(), entry.clone(), self.cell_idx)),
            None => Output::NoPermute(self.cell_idx)
        };

        self.cell_idx += 1;
        output
    }
}


// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::Cell;
//     use smallvec::SmallVec;
    
//     #[test]
//     fn behavior_step() {
//         let mut cell = Cell::new();
//         cell.extend(b"...".iter().map(|x| *x));

//         let rules: Rulebook = {
//             let mut rulebook = HashMap::new();
//             let rules = b"ab";
//             rulebook.insert(b'.', &rules[..]);
//             rulebook.into()
//         };
       
//         let mut cursor = Cursor::new(&cell, &rules);

//         for cell_idx in 0..3 {
//             let permute = cursor.step();
//             match permute {
//                 Output::Permute(permute) => {
//                     assert_eq!(permute.index(), cell_idx)
//                 },
//                 Output::NoPermute(idx) => assert_eq!(cell_idx, idx),
//                 Output::EndOfLine => assert_eq!(1, 2)
//             }
//         }
//         assert!(!cursor.reset_flag);
//         assert_eq!(cursor.step(), Output::EndOfLine);
//         assert!(cursor.reset_flag);
//     }
// }