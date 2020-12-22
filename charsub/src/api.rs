use std::collections::{HashMap, HashSet};
use crate::{
    Cell, RuleCell,
    patterns::RuleEntry,
    cursor::{Cursor, Output},
    scheduler::{Scheduler, Length}
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

pub struct Generator<T: Scheduler> {
    rules: Rulebook,
    gen_ctr: usize,
    scheduler: T,
    buf: T::Buffer,
}

impl<T> Generator<T> where T: Scheduler
{
    pub fn new<R>(rules: R, scheduler: T, buf: T::Buffer) -> Self
    where R: Into<Rulebook>
    {
        Self {
            rules: rules.into(),
            gen_ctr: 0,
            scheduler,
            buf
        }
    }

    /// This function returns the map
    /// used to replace characters with
    /// where the key (`u8`), is the character
    /// wishing to be replaced with its value `RuleEntry`
    pub fn map(&self) -> &Rulebook {
        &self.rules
    }
    
    pub fn new_generation<H: Handler>(&mut self, old_generation: HashSet<Cell>) -> usize 
    where
        H: Handler,
        T: Scheduler
    {
        let original = self.buf.length();
        let mut generation = HashSet::new();
        
        for item in &old_generation {
            let mut cursor = Cursor::new(&item, &self.rules.0);
            Self::permutate_cell::<H>(&mut self.scheduler, &mut cursor, &mut self.buf);
        }

        generation.union(&old_generation);
        self.gen_ctr += 1;
        self.buf.length()-original
    }

    pub fn generation(&self) -> usize {
        self.gen_ctr
    }

    fn permutate_cell<H: Handler>(executor: &mut T, cursor: &mut Cursor, buf: &mut T::Buffer) -> usize {
        let original = buf.length();
        
        loop {
            match cursor.step() {
                Output::Permute(permute) => {
                    if !H::handle(&permute) {
                        continue
                    }
                    executor.schedule(permute, buf);
                },

                Output::NoPermute(_idx) => {
                    continue
                },
                Output::EndOfLine => break
            }
        }

        buf.length()-original
    }
}

