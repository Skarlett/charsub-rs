use crate::{
    Cell, RuleCell,
    patterns::RuleEntry,
    unit::UnitPair,
    patterns::Handler
};
use std::collections::{HashMap, HashSet};

pub struct Generator {
    total: HashSet<Cell>,
    rules: HashMap<u8, RuleEntry>,
    generation: usize,
}

impl Generator {
    pub fn new(root: &Cell, rules: &HashMap<u8, RuleEntry>) -> Self {
        let mut buf = HashSet::new();
        buf.insert(root.clone());

        Self {
            rules: rules.clone(),
            total: buf,
            generation: 0,
        }
    }
    
    pub fn current_generation(&self) -> usize {
        self.generation
    }

    pub fn buf(&self) -> &HashSet<Cell> {
        &self.total
    }

    pub fn into_buf(self) -> HashSet<Cell> {
        self.total
    }

    pub fn new_generation<'s, T: Handler>(&'s mut self, handler:&mut T) -> usize {
        let original = self.total.len();
        
        let mut generation = Vec::new();
        for item in &self.total {
            let mut cursor = Cursor::new(&item, &self.rules);
            permutate_cell(&mut cursor, handler, &mut generation);
        }

        self.total.extend(generation);
        self.generation += 1;
        self.total.len()-original
    }

}

fn permutate_cell<T: Handler>(cursor: &mut Cursor, handler:&mut T, buffer: &mut Vec<Cell>) -> usize {
    let original = buffer.len();
    
    loop {
        match cursor.step() {
            CursorOutput::Permute(mut permute) => {
                if !handler.handle(&permute) {
                    continue
                }
                
                while let Some(mutation) = permute.commit() {
                    buffer.push(mutation.clone());
                }
            },

            CursorOutput::NoPermute(_idx) => {
                //println!("[{}]{:?}", cursor.cell_idx, cursor.buffer());
                continue
            },
            CursorOutput::EndOfLine => break
        }
    }

    buffer.len()-original
}

#[derive(Debug, PartialEq)]
pub enum CursorOutput<'a> {
    Permute(UnitPair<'a>),
    NoPermute(usize),
    EndOfLine
}

pub struct Cursor<'buf, 'rules> {
    buf: &'buf Cell,
    rule_lookup: &'rules HashMap<u8, RuleEntry>,
    cell_idx: usize,
    reset_flag: bool
}

impl<'b, 'r> Cursor<'b, 'r> {
    pub fn new(buf: &'b Cell, rule_lookup: &'r HashMap<u8, RuleEntry>) -> Self {
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

    pub fn step<'s>(&'s mut self) -> CursorOutput<'s> {                
        let byte = match self.buf.get(self.cell_idx) {
            Some(byte) => *byte,
            None => {
                self.cell_idx = 0;
                self.reset_flag = true;
                return CursorOutput::EndOfLine
            }
        };

            
        let output = match self.rule_lookup.get(&byte) {
            Some(entry) => CursorOutput::Permute(UnitPair::new(self.buf.clone(), entry, self.cell_idx)),
            None => CursorOutput::NoPermute(self.cell_idx)
        };

        self.cell_idx += 1;
        output
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::Cell;
    use smallvec::SmallVec;
    
    #[test]
    fn behavior_step() {
        let mut cell = Cell::new();
        cell.extend(b"...".iter().map(|x| *x));

        let rules = {
            let mut rules = RuleCell::new();
            rules.extend(b"AB".iter().map(|x| *x));
            let mut rulebook = HashMap::new();
            rulebook.insert(b'.', RuleEntry::Multi(rules));
            rulebook
        };
       
        let mut cursor = Cursor::new(&cell, &rules);

        for cell_idx in 0..3 {
            let permute = cursor.step();
            match permute {
                CursorOutput::Permute(permute) => {
                    assert_eq!(permute.index(), cell_idx)
                },
                CursorOutput::NoPermute(idx) => assert_eq!(cell_idx, idx),
                CursorOutput::EndOfLine => assert_eq!(1, 2)
            }
        }
        assert!(!cursor.reset_flag);
        assert_eq!(cursor.step(), CursorOutput::EndOfLine);
        assert!(cursor.reset_flag);
    }
}