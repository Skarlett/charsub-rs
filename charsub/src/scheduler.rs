use crate::{
    unit::Permutation,
    patterns::Handler,
    Cell as CharBuf,
};

use std::{
    collections::HashSet,
};

pub trait Scheduler {
    type Buffer;
    fn schedule(&mut self, permute: Permutation, buf: &mut Self::Buffer);
    fn into_buf(&self) -> HashSet<CharBuf> { unimplemented!() }
    fn buf_len(&self) -> usize { unimplemented!()}
}

enum SendData {
    Arc(Arc<Mutex<HashSet<CharBuf>>>),
    Set(HashSet<CharBuf>)
}

#[derive(Debug)]
struct SingleThread;

impl Scheduler for SingleThread {
    type Buffer = HashSet<CharBuf>;

    fn schedule(&mut self, mut permute: Permutation, buf: &mut r) {
        while let Some(x) = permute.commit() {
            buf.insert(x.clone());
        }
    }

}

use std::sync::{Arc, Mutex};
#[derive(Debug)]
struct MultithreadMutex {
    pool: threadpool::ThreadPool,
    storage: Arc<Mutex<HashSet<CharBuf>>>
}

impl Scheduler for MultithreadMutex {
    type Buffer = HashSet<CharBuf>;

    fn schedule(&mut self, mut permute: Permutation, buf: &mut Self::Buffer) {
        let lock = self.storage.clone();
        self.pool.execute(move || {
            
            let mut lock = lock.lock().unwrap();
            while let Some(x) = permute.commit() {
                lock.insert(x.clone());
            }
        })
    }

}

#[derive(Clone, Debug, Default)]
struct EVHashSet(HashSet<CharBuf>);

#[derive(Clone, Debug)]
enum Operation {
    Push(CharBuf),
    Clear,
}

#[derive(Debug)]
struct TokioMutex {
    runtime: tokio::runtime::Runtime,
    buffer: Arc<Mutex<HashSet<CharBuf>>>
}

impl Scheduler for TokioMutex {
    type Buffer = HashSet<CharBuf>;

    fn schedule(&mut self, mut permute: Permutation, buf: &mut Self::Buffer) {
        let buf = self.buffer.clone();
        self.runtime.spawn(async move {
            let mut lock = buf.lock().unwrap();
            while let Some(x) = permute.commit() {            
                lock.insert(x.clone());
            }
        });
    }
}
