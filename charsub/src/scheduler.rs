use crate::{
    unit::Permutation,
    Cell as CharBuf,
};

use std::{
    collections::HashSet,
};

pub trait Length {
    fn length(&self) -> usize;
}

pub trait Scheduler {
    type Buffer: Default + Length;
    fn schedule(&mut self, permute: Permutation, buf: &mut Self::Buffer);
}

#[derive(Debug)]
struct SingleThread;


impl Length for HashSet<CharBuf> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Scheduler for SingleThread {
    type Buffer = HashSet<CharBuf>;

    fn schedule(&mut self, mut permute: Permutation, buf: &mut Self::Buffer) {
        while let Some(x) = permute.commit() {
            buf.insert(x.clone());
        }
    }

}

use std::sync::{Arc, Mutex};
#[derive(Debug)]

struct MultithreadMutex {
    pool: threadpool::ThreadPool,
}

impl Length for Arc<Mutex<HashSet<CharBuf>>> {
    fn length(&self) -> usize {
        self.lock().unwrap().len()
    }
}

impl Scheduler for MultithreadMutex {
    type Buffer = Arc<Mutex<HashSet<CharBuf>>>;

    fn schedule(&mut self, mut permute: Permutation, buf: &mut Self::Buffer) {
        let buf_ref = buf.clone();

        self.pool.execute(move || {    
            let mut lock = buf_ref.lock().unwrap();
            while let Some(x) = permute.commit() {
                lock.insert(x.clone());
            }
        });
    }

}

#[derive(Debug)]
struct TokioMutex {
    runtime: tokio::runtime::Runtime,
}

impl Scheduler for TokioMutex {
    type Buffer = Arc<Mutex<HashSet<CharBuf>>>;

    fn schedule(&mut self, mut permute: Permutation, buf: &mut Self::Buffer) {
        let buf_ref = buf.clone();

        self.runtime.spawn(async move {
            let mut lock = buf_ref.lock().unwrap();
            while let Some(x) = permute.commit() {            
                lock.insert(x.clone());
            }
        });
    }
}
