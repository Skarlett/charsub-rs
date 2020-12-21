use crate::{
    unit::Permutation,
    patterns::Handler,
    Cell,
};

use std::collections::HashSet;

pub trait Scheduler: std::fmt::Debug {
    type Buffer: Default;

    fn schedule<T: Handler>(&mut self, permute: Permutation);
    fn flush(&mut self) -> Self::Buffer;
}

#[derive(Debug)]
struct SingleThread {
    storage: HashSet<Cell>
}

impl Scheduler for SingleThread {
    type Buffer = HashSet<Cell>;

    fn schedule<T: Handler>(&mut self, mut permute: Permutation) {
        if T::handle(&permute) {
            while let Some(x) = permute.commit() {
                self.storage.insert(x.clone());
            }
        }
    }

    fn flush(&mut self) -> Self::Buffer {
        std::mem::replace(&mut self.storage, Self::Buffer::default())
    }
}


use std::sync::{Arc, Mutex};
#[derive(Debug)]
struct MultithreadMutex {
    pool: threadpool::ThreadPool,
    collection: Arc<Mutex<HashSet<Cell>>>
}

impl Scheduler for MultithreadMutex {
    type Buffer = HashSet<Cell>;

    fn schedule<T: Handler>(&mut self, mut permute: Permutation) {
        if T::handle(&permute) {
            let lock = self.collection.clone();
            self.pool.execute(move || {
                let mut lock = lock.lock().unwrap();

                while let Some(x) = permute.commit() {
                    lock.insert(x.clone());
                }
            })
        }
    }

    fn flush(&mut self) -> Self::Buffer {
        let data = self.collection.lock().unwrap().clone();
        self.collection = Arc::new(Mutex::new(Self::Buffer::default()));
        data
    }
}

#[derive(Debug)]
struct MultithreadEventual {
    pool: threadpool::ThreadPool,
    collection: Arc<Mutex<HashSet<Cell>>>
}

impl Scheduler for MultithreadEventual {
    type Buffer = HashSet<Cell>;

    fn schedule<T: Handler>(&mut self, mut permute: Permutation) {
        let lock = self.collection.clone();

        self.pool.execute(move || {
            if T::handle(&permute) {
                let mut lock = lock.lock().unwrap();
                while let Some(x) = permute.commit() {            
                    lock.insert(x.clone());
                }
            }
        })
    }

    fn flush(&mut self) -> Self::Buffer {
        let data = self.collection.lock().unwrap().clone();
        self.collection = Arc::new(Mutex::new(Self::Buffer::default()));
        data
    }
}

#[derive(Clone, Debug, Default)]
struct HashSetWrapper(HashSet<Cell>);

#[derive(Clone, Debug)]
enum Operation {
    Push(Cell),
    Clear,
}

impl evc::OperationCache for HashSetWrapper {
    type Operation = Operation;

    fn apply_operation(&mut self, operation: Self::Operation) {
        match operation {
            Operation::Push(value) => { self.0.insert(value); },
            Operation::Clear => self.0.clear(),
        }
    }
}


#[derive(Debug)]
struct TokioEventual {
    exchange: HashSetWrapper,
    buffer: HashSet<Cell>
}

impl Scheduler for TokioEventual {
    type Buffer = HashSet<Cell>;

    fn schedule<T: Handler>(&mut self, mut permute: Permutation) {
        use tokio::task;
        tokio::runtime
        task::spawn(async move {
            if T::handle(&permute) {
                while let Some(x) = permute.commit() {            
                    lock.insert(x.clone());
                }
            }
        })
    }

    fn flush(&mut self) -> Self::Buffer {
        let data = self.buffer.lock().unwrap().clone();
        self.collection = Arc::new(Mutex::new(Self::Buffer::default()));
        data
    }
}