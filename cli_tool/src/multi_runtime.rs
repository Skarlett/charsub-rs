/// primarily used to translate generic structures
/// into finite structures we can use
/// selectively during runtime
/// ---
use charsub::{
    Scheduler,
    scheduler::{MultithreadMutex, SingleThread, TokioMutex},
    Permutation,
    Cell, Handler,
    Length, Rulebook
};

use crate::input::{
    SchedulerInput,
    Opt
};

use hashbrown::HashSet;

use std::{
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub enum Schedules {
    Async(TokioMutex),
    Pool(MultithreadMutex),
    SingleThread(SingleThread),
}

impl Schedules {
    pub fn from_opt(opts: &Opt) -> Self {
        match opts.scheduler {
            SchedulerInput::AsyncRuntime => Schedules::Async(
                TokioMutex::new()
            ),
            SchedulerInput::ThreadPool => Schedules::Pool(MultithreadMutex::new(opts.workers)),
            SchedulerInput::SingleThread => Schedules::SingleThread(SingleThread::new())
        }
    }
}

impl Scheduler for Schedules { 
    
    fn push(&mut self, item: Cell) {
        match self {
            Schedules::Async(rt) => rt.push(item),
            Schedules::Pool(pool) => pool.push(item),
            Schedules::SingleThread(func) => func.push(item)
        }
    }

    fn new_generation<H>(&mut self, rules: &Rulebook) where H: Handler {
        match self {
            Schedules::Async(rt) => rt.new_generation::<H>(rules),
            Schedules::Pool(pool) => pool.new_generation::<H>(rules),
            Schedules::SingleThread(func) => func.new_generation::<H>(rules)
        }
    }

    fn clean_state(&self) -> bool {
        match self {
            Schedules::Async(rt) => rt.clean_state(),
            Schedules::Pool(pool) => pool.clean_state(),
            Schedules::SingleThread(func) => true,
        }
    }

    fn schedule(&mut self, permute: Permutation)  {
        match self {
            Schedules::Async(rt) => rt.schedule(permute),
            Schedules::Pool(pool) => pool.schedule(permute),
            Schedules::SingleThread(func) => func.schedule(permute),
        }
    }
}

impl Length for Schedules {
    fn length(&self) -> usize {
        match self {
            Schedules::Async(rt) => rt.length(),
            Schedules::Pool(pool) => pool.length(),
            Schedules::SingleThread(func) => func.length(),
        }
    }
}

impl Into<HashSet<Cell>> for Schedules {
    fn into(self) -> HashSet<Cell> {
        match self {
            Schedules::Async(rt) => rt.into(),
            Schedules::Pool(pool) => pool.into(),
            Schedules::SingleThread(func) => func.into(),
        }
    }
}