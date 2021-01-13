use std::iter;


#[macro_use]
extern crate criterion;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::black_box;
use charsub::Scheduler;
use hashbrown::HashMap;
use rand::prelude::*;

const CYCLES: &'static [usize] = &[1, 2, 4, 8, 16];
const MAX: usize = 16;

macro_rules! test_me {
    ($scheduler:ty, $pattern:ty) => 
    (   
        
        paste::paste!{
        
        
        fn [<finished_ $scheduler _ $pattern>](c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($scheduler));

            for input_size in CYCLES.iter()
            {
                for rule_size in CYCLES.iter() {
                    //group.throughput(Throughput::Bytes((input_size*rule_size) as u64));

                    group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}({}) input: {} rules: {}", stringify!($scheduler), stringify!($pattern), input_size, rule_size)), &(*input_size, *rule_size), |b, (in_size, rsize)| {
                            let mut buf: [u8; MAX] = [0; MAX];                            
                            rand::thread_rng().fill_bytes(&mut buf[..*in_size]);

                            let mut executor: $scheduler = Default::default();
                            executor.push(black_box(charsub::Cell::from(&buf[..*in_size])));
                            
                            let mut map = HashMap::new();
                            for _ in 0..*rule_size {
                                let k: u8 = rand::random();
                                let mut buf: [u8; MAX] = [0; MAX];
                                rand::thread_rng().fill_bytes(&mut buf[..*rsize]);
                                map.insert(k, charsub::RuleCell::from(black_box(&buf[..*rsize])));
                            }
                           
                            let rules = map.into();

                            b.iter(|| {
                                for _ in 0..*in_size {
                                    executor.new_generation::<$pattern>(black_box(&rules));
                                }
                                while !executor.clean_state() {}
                            });
                    });
                }
            }
            group.finish();
        }
        }
    )
}

use charsub::{
    scheduler::{MultithreadMutex, TokioMutex, SingleThread},
    ConstPattern, ModulusPattern

};

test_me!(SingleThread, ConstPattern);
test_me!(TokioMutex, ConstPattern);

criterion_group!(benches, finished_SingleThread_ConstPattern, finished_TokioMutex_ConstPattern);


criterion_main!(benches);
//fn main() {}