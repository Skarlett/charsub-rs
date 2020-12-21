use std::collections::{HashMap, HashSet};
use crate::{
    Cell, RuleCell,
    unit::Permutation,
    patterns::{RuleEntry, ConstPattern, ModulusPattern},
    cursor::{Cursor, Output},

};

pub use crate::patterns::Handler;

pub struct Rulebook(pub HashMap<u8, RuleEntry>);

impl<T> From<T> for Rulebook 
where T: IntoIterator<Item=(u8, RuleCell)>
{
    fn from(x: T) -> Rulebook {
        let mut map = HashMap::new();
        for (k, r) in x.into_iter() {
            let rule = match r.len() {
                1 => RuleEntry::Single(*r.get(0).unwrap()),
                i if i > 1 => RuleEntry::Multi(r),
                _ => panic!("lolwhut")
            };
            map.insert(k, rule);
        }
        Rulebook(map)
    }
}

pub struct Generator {
    total: HashSet<Cell>,
    rules: Rulebook,
    i: usize,
}

impl Generator {
    pub fn new<T: Into<Rulebook>>(root: &[u8], rules: T) -> Self {
        let mut buf = HashSet::new();
        let mut cell = Cell::new();
        cell.extend(root.iter().map(|b| *b));
        buf.insert(cell);

        Self {
            rules: rules.into(),
            total: buf,
            i: 0,
        }
    }

    /// This function returns the map
    /// used to replace characters with
    /// where the key (`u8`), is the character
    /// wishing to be replaced with its value `RuleEntry`
    pub fn map(&self) -> &Rulebook {
        &self.rules
    }
    
    /// Returns a reference to the inner buffer
    pub fn buf(&self) -> &HashSet<Cell> {
        &self.total
    }

    /// Returns the value to the inner buffer, destroying its self.
    pub fn into_buf(self) -> HashSet<Cell> {
        self.total
    }

    pub fn new_generation<'s, T: Handler>(&'s mut self, handler:&mut T) -> usize {
        let original = self.total.len();
        
        let mut generation = Vec::new();
        for item in &self.total {
            let mut cursor = Cursor::new(&item, &self.rules.0);
            permutate_cell(&mut cursor, handler, &mut generation);
        }

        self.total.extend(generation);
        self.i += 1;
        self.total.len()-original
    }

    pub fn generation(&self) -> usize {
        self.i
    }
}

fn permutate_cell<T: Handler>(cursor: &mut Cursor, handler:&mut T, buffer: &mut Vec<Cell>) -> usize {
    let original = buffer.len();
    
    loop {
        match cursor.step() {
            Output::Permute(mut permute) => {
                if !T::handle(&permute) {
                    continue
                }
                
                while let Some(mutation) = permute.commit() {
                    buffer.push(mutation.clone());
                }
            },

            Output::NoPermute(_idx) => {
                //println!("[{}]{:?}", cursor.cell_idx, cursor.buffer());
                continue
            },
            Output::EndOfLine => break
        }
    }

    buffer.len()-original
}
