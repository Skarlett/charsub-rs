use crate::{
    Cell, RuleCell,
    patterns::RuleEntry,
    cursor::{Cursor, Output},
    scheduler::Scheduler
};

use hashbrown::{HashSet, HashMap};

pub use crate::patterns::Handler;

#[derive(Debug,)]
pub struct Rulebook(pub HashMap<u8, RuleEntry>);

impl<'a, T> From<T> for Rulebook 
where T: IntoIterator<Item=(u8, RuleCell)>
{
    fn from(x: T) -> Rulebook {
        let mut map = HashMap::new();
        for (k, r) in x.into_iter() {
            let rule = match r.len() {
                1 => RuleEntry::Single(*r.get(0).unwrap()),
                i if i > 1 => RuleEntry::Multi(RuleCell::from(r)),
                _ => panic!("lolwhut")
            };
            map.insert(k, rule);
        }
        Rulebook(map)
    }
}

#[derive(Debug)]
pub struct Generator<T> {
    rules: Rulebook,
    gen_ctr: usize,
    scheduler: T,
}

impl<T> Generator<T>
{
    pub fn new<R>(rules: R, scheduler: T) -> Self
    where
        R: Into<Rulebook>,
        T: Scheduler
    {
        Self {
            rules: rules.into(),
            gen_ctr: 0,
            scheduler,
        //  buf
        }
    }
    /// This function returns the map
    /// used to replace characters with
    /// where the key (`u8`), is the character
    /// wishing to be replaced with its value `RuleEntry`
    pub fn map(&self) -> &Rulebook {
        &self.rules
    }
    
    pub fn new_generation<H>(&mut self)
    where
        H: Handler,
        T: Scheduler
    {
        self.scheduler.new_generation::<H>(&self.rules)
    }

    pub fn generation(&self) -> usize {
        self.gen_ctr
    }

    pub fn length(&self) -> usize 
    where T: Scheduler + Length
    {
        self.scheduler.length()        
    }

    pub fn done(&self) -> bool
    where T: Scheduler
    {
        self.scheduler.clean_state()
    }

    pub fn into_set(self) -> HashSet<Cell> 
    where T: Scheduler + Into<HashSet<Cell>>
    {
        self.scheduler.into()
    }

    pub fn seed<I>(&mut self, item: I)
    where 
        T: Scheduler,
        I: Into<Cell>
    {
        self.scheduler.push(item.into());
    }
}

// pub trait PreEmptiveAlloc {
//     fn init_capacity(size: usize) -> Self;
// }

pub trait Length {
    fn length(&self) -> usize;
}

impl Length for HashSet<Cell> {
    fn length(&self) -> usize {
        self.len()
    }
}

use std::sync::{Arc, Mutex};
impl Length for Arc<Mutex<HashSet<Cell>>> {
    fn length(&self) -> usize {
        self.lock().unwrap().len()
    }
}
