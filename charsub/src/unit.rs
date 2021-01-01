use crate::RuleCell;
use crate::patterns::RuleEntry;

#[derive(Debug, PartialEq)]
pub struct Permutation {
    rule: RuleEntry,
    cell: crate::Cell,
    pub rule_idx: usize,
    pub cell_idx: usize
}

impl Permutation {
    pub fn new(cell: crate::Cell, rule: RuleEntry, cell_idx: usize) -> Self {
        Self {
            cell,
            rule,
            cell_idx,
            rule_idx: 0,
        }
    }

    pub fn changes(&self) -> usize {
        self.rule.len()
    }

    pub fn index(&self) -> usize {
        self.cell_idx
    }

    pub fn rules(&self) -> &RuleEntry {
        &self.rule
    }

    pub fn len(&self) -> usize {
        self.cell.len()
    }

    /// checks if we should step again
    pub fn peek_next(&self) -> bool {
        match &self.rule {
            RuleEntry::Single(_byte) => !self.rule_idx > 0,
            RuleEntry::Multi(buf) => buf.len() >= self.rule_idx+1
        }
    }

    pub fn commit<'s>(&'s mut self) -> Option<&'s crate::Cell> {
        match &self.rule {
            RuleEntry::Single(_byte) => {
                //println!("rules: {}", *_byte as char);

                if self.rule_idx > 0 {
                    return None
                }
                self.rule_idx = 1;
                return Some(&self.cell)
            }

            RuleEntry::Multi(rules) => {
                //println!("rules: {}", String::from_utf8_lossy(&rules));
                if self.rule.len() >= self.rule_idx+1 {
                    self.cell[self.cell_idx] = *rules.get(self.rule_idx).unwrap();
                    self.rule_idx += 1;
                    return Some(&self.cell)
                }
                return None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Cell;
    use smallvec::SmallVec;
    
    #[test]
    fn behavior_commit() {
        let mut cell = Cell::new();
        cell.extend(b"...".iter().map(|x| *x));
        let rules = {
            let mut rules = RuleCell::new();
            rules.extend(b"AB".iter().map(|x| *x));
            RuleEntry::Multi(rules)
        };

        let mut permutate = Permutation::new(cell, rules, 0);
        let mut buf = SmallVec::new();
        
        for x in vec![b"A..", b"B.."] {
            buf.extend(x.iter().map(|x| *x));
            assert_eq!(Some(&buf), permutate.commit());
            buf.clear()
        }
        
        assert_eq!(None, permutate.commit());
        assert_eq!(None, permutate.commit());
    }
}