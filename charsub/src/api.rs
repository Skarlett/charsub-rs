use std::collections::{HashMap, HashSet};
pub use crate::patterns::Handler;

use crate::{
    Cell, RuleCell,
    patterns::{RuleEntry, ConstPattern, ModulusPattern},
    unit::UnitPair,
    cursor::{Cursor, CursorOutput},

};
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
    generation: usize,
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
            generation: 0,
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


use smallvec::SmallVec;
#[derive(Debug)]
pub enum Pattern {
    Const(ConstPattern),
    Modulo(ModulusPattern),
    //Multi(Box<dyn Handler>)
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern::Const(Default::default())
    }
}

impl Handler for Pattern {
    fn handle(&mut self, permute: &UnitPair<'_>) -> bool {
        match self {
            Pattern::Const(hdlr) => hdlr.handle(permute),
            Pattern::Modulo(hdlr) => hdlr.handle(permute),
            //Pattern::Multi(vec) => vec.iter_mut().any(|x| x.handle(permute))
        }
    }
}