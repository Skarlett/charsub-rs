use crate::{
    unit::Permutation,
    Cell as CharBuf,
    RuleCell,
    patterns::Handler,
    cursor::{Cursor, Output},
    Rulebook,
    Length
};

use std::sync::{Arc, Mutex};
use hashbrown::HashSet;

pub trait Scheduler {

    fn schedule(&mut self, permute: Permutation);
    fn clean_state(&self) -> bool;

    fn new_generation<H>(&mut self, rules: &Rulebook) where H: Handler;

    fn push(&mut self, item: CharBuf);

    fn permutate_cell<H>(&mut self, cursor: &mut Cursor)
    where
        H: Handler+ std::fmt::Debug
    {
        loop {
            match cursor.step() {
                Output::Permute(permute) => {
                    if H::handle(&permute) {
                        continue
                    }
                    self.schedule(dbg!(permute));
                },

                Output::NoPermute(_idx) => {
                    continue
                },
                Output::EndOfLine => break
            }
        }

    }
}


#[derive(Debug, Default)]
pub struct SingleThread {
    buf: HashSet<CharBuf>   
}

impl SingleThread {
    pub fn new() -> Self {
        Self {
            buf: HashSet::new()
        }
    }
}

impl Into<HashSet<CharBuf>> for SingleThread {
    fn into(self) -> HashSet<CharBuf> {
        self.buf
    }
}

impl Length for SingleThread {
    fn length(&self) -> usize {
        self.buf.len()
    }
}

impl Scheduler for SingleThread {
    fn schedule(&mut self, mut permute: Permutation) {
        while let Some(x) = permute.commit() {
            self.buf.insert(x.clone());
        }
    }

    fn push(&mut self, item: CharBuf) {
        self.buf.insert(item);
    }

    fn clean_state(&self) -> bool {
        true
    }

    fn new_generation<H>(&mut self, rules: &Rulebook) where H: Handler {
        let mut buf = std::mem::replace(&mut self.buf, HashSet::new());
        //println!("{:?}", buf.clone().iter().map(|x| String::from_utf8_lossy(&x)).collect::<Vec<_>>());
        for item in &buf {
            let mut cursor = Cursor::new(&item, rules);
            self.permutate_cell::<H>(&mut cursor);
        }

        self.buf.extend(buf.drain());
    }
}

fn shared_buf() -> Arc<Mutex<HashSet<CharBuf>>> {
    Arc::new(Mutex::new(HashSet::new()))
}


#[derive(Debug, Default)]
pub struct MultithreadMutex {
    pool: threadpool::ThreadPool,
    buf: Arc<Mutex<HashSet<CharBuf>>>
}

impl MultithreadMutex {
    pub fn new(workers: usize) -> Self {
        Self {
            pool: threadpool::ThreadPool::new(workers),
            buf: shared_buf()
        }
    }
}

impl Into<HashSet<CharBuf>> for MultithreadMutex {
    fn into(self) -> HashSet<CharBuf> {
        self.buf.lock().unwrap().clone()
    }
}

impl Length for MultithreadMutex {
    fn length(&self) -> usize {
        self.buf.lock().unwrap().len()
    }
}

impl Scheduler for MultithreadMutex {

    fn new_generation<H>(&mut self, rules: &Rulebook) where H: Handler { 
        let mut lock: Vec<_> = self.buf.lock().unwrap().drain().collect();
        
        for item in &lock {
            let mut cursor = Cursor::new(&item, rules);
            self.permutate_cell::<H>(&mut cursor);
        }

        self.buf.lock().unwrap().extend(lock.drain(..));

    }

    fn schedule(&mut self, mut permute: Permutation) {
        let buf_ref = self.buf.clone();

        self.pool.execute(move || {    
            let mut lock = buf_ref.lock().unwrap();
            while let Some(x) = permute.commit() {
                lock.insert(x.clone());
            }
        });
    }

    fn clean_state(&self) -> bool {
        true
    }

    fn push(&mut self, item: CharBuf) {
        self.buf.lock().unwrap().insert(item);
    }
}

lazy_static! {
    static ref RT: Arc<tokio::runtime::Runtime> = Arc::new(tokio::runtime::Runtime::new().unwrap());
}

#[derive(Debug)]
pub struct TokioMutex {
    runtime: Arc<tokio::runtime::Runtime>,
    buf: Arc<Mutex<HashSet<CharBuf>>>
}

impl Default for TokioMutex {
    fn default() -> Self {
        Self::new()
    }
}

impl TokioMutex {

    #[inline]
    pub fn new() -> Self {
        Self { runtime: RT.clone(), buf: Arc::new(Mutex::new(Default::default())) }
    }
}

impl Into<HashSet<CharBuf>> for TokioMutex {
    fn into(self) -> HashSet<CharBuf> {
        self.buf.lock().unwrap().clone()
    }
}

impl Length for TokioMutex {
    fn length(&self) -> usize {
        self.buf.lock().unwrap().len()
    }
}

impl Scheduler for TokioMutex {
    fn new_generation<H>(&mut self, rules: &Rulebook) where H: Handler { 
        let lock: Vec<_> = self.buf.lock().unwrap().drain().collect();
        
        for item in lock {
            let mut cursor = Cursor::new(&item, rules);
            self.permutate_cell::<H>(&mut cursor);
        }
    }

    fn schedule(&mut self, mut permute: Permutation) {
        let buf_ref = self.buf.clone();

        self.runtime.spawn(async move {
            let mut lock = buf_ref.lock().unwrap();
            while let Some(x) = permute.commit() {
                lock.insert(x.clone());
            }
        });
    }

    fn clean_state(&self) -> bool {
        true
    }

    fn push(&mut self, item: CharBuf) {
        self.buf.lock().unwrap().insert(item);
    }
}
