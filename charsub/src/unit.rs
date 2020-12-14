use crate::RuleCell;
use crate::patterns::RuleEntry;

#[derive(Debug, PartialEq)]
pub struct UnitPair<'r> {
    rule: &'r RuleEntry,
    cell: crate::Cell,
    pub rule_idx: usize,
    pub cell_idx: usize
}

impl<'r> UnitPair<'r> {
    pub fn new(cell: crate::Cell, rule: &'r RuleEntry, cell_idx: usize) -> Self {
        Self {
            cell,
            rule,
            cell_idx,
            rule_idx: 0,
        }
    }

    pub fn index(&self) -> usize {
        self.cell_idx
    }

    pub fn rules(&self) -> &RuleEntry {
        self.rule
    }

    pub fn len(&self) -> usize {
        self.cell.len()
    }

    /// checks if we should step again
    pub fn peek_next(&self) -> bool {
        match self.rule {
            RuleEntry::Single(byte) => !self.rule_idx > 0,
            RuleEntry::Multi(buf) => self.rule.len() >= self.rule_idx+1
        }
    }

    pub fn commit<'s>(&'s mut self) -> Option<&'s crate::Cell> {
        match self.rule {
            RuleEntry::Single(byte) => {
                if self.rule_idx > 0 {
                    return None
                }
                self.rule_idx = 1;
                return Some(&self.cell)
            }

            RuleEntry::Multi(rules) => {
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

        let mut permute = UnitPair::new(cell, &rules, 0);

        let mut buf = SmallVec::new();
        
        for x in vec![b"A..", b"B.."] {
            buf.extend(x.iter().map(|x| *x));
            assert_eq!(Some(&buf), permute.commit());
            buf.clear()
        }        
        assert_eq!(None, permute.commit());
        assert_eq!(None, permute.commit());
    }
}