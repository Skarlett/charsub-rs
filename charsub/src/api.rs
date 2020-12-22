use std::collections::{HashMap, HashSet};
use crate::{
    Cell, RuleCell,
    unit::Permutation,
    patterns::{RuleEntry, ConstPattern, ModulusPattern},
    cursor::{Cursor, Output},
    scheduler::Scheduler

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

pub struct Generator<T> {
    rules: Rulebook,
    gen_ctr: usize,
    scheduler: T
}

impl<T> Generator<T> {
    pub fn new<R>(rules: R, scheduler: T) -> Self
    where
        R: Into<Rulebook>,
        T: Scheduler
    {
        Self {
            rules: rules.into(),
            gen_ctr: 0,
            scheduler
        }
    }

    /// This function returns the map
    /// used to replace characters with
    /// where the key (`u8`), is the character
    /// wishing to be replaced with its value `RuleEntry`
    pub fn map(&self) -> &Rulebook {
        &self.rules
    }
    
    pub fn new_generation<H>(&mut self, old_generation: HashSet<Cell>) -> HashSet<Cell> 
    where
        H: Handler,
        T: Scheduler
    {
        let original = self.scheduler.buf_len();

        let mut generation = HashSet::new();
        
        for item in &old_generation {
            let mut cursor = Cursor::new(&item, &self.rules.0);
            permutate_cell::<T, H>(&mut cursor, &mut generation, &mut self.scheduler);
        }

        generation.union(&old_generation);
        self.gen_ctr += 1;
        // self.total.len()-original;

        unimplemented!()
    }

    pub fn generation(&self) -> usize {
        self.gen_ctr
    }
}

fn permutate_cell<S, H>(cursor: &mut Cursor, buffer: &mut HashSet<Cell>, scheduler: &mut S) -> usize
where 
    S: Scheduler,
    H: Handler
{
    let original = buffer.len();
    
    loop {
        match cursor.step() {
            Output::Permute(permute) => {
                if !H::handle(&permute) {
                    continue
                }
                
                scheduler.schedule(permute, buffer);
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
