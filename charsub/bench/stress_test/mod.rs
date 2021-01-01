
extern crate test;

mod mutex_thread_bench;
mod single_thread;
mod tokio_mutex;

use rand::prelude::*;

use std::collections::{HashMap,HashSet};


use charsub::{
    ConstPattern, Rulebook, scheduler::SingleThread, RuleCell, Scheduler, Cell
};

/// Used for generating the same test, 
/// multiple times,
/// with varying work loads, 
/// and different parameters
#[macro_export]
macro_rules! gen_bench_stress_test {
    ($scheduler:ty, $pattern:ty, $input_size:expr, $rule_size:expr, $input_amount:expr) => 
    {
        paste::paste! {
            #[bench]
            fn [<stress_test_ $scheduler _ $pattern _ $rule_size _ $input_size _ $input_amount>]
                (b: &mut test::Bencher) {
                use rand::prelude::*;
                use charsub::Scheduler;
                lazy_static::lazy_static! {
                    //use stress_test::*;
                    static ref INPUT: Vec<[u8; $input_size]> = {
                        let mut buf = Vec::new();
                        for x in 0..$input_amount {
                            let mut v: [u8; $input_size] = [0; $input_size];
                            rand::thread_rng().fill_bytes(&mut v[..]);
                            buf.push(v)
                        }
                        buf
                    };

                    static ref RULES: charsub::Rulebook = {
                        let mut m = std::collections::HashMap::new();
                        for _ in 0..$input_amount {
                            let k: u8 = rand::random();
                            let mut v: [u8; $rule_size] = [0; $rule_size];
                            rand::thread_rng().fill_bytes(&mut v[..]);
                            m.insert(k, charsub::RuleCell::from(&v[..]));
                        }
                        m.into()
                    };
                };
                
                let mut executor: $scheduler = Default::default();
                for x in INPUT.iter() {
                    executor.push(charsub::Cell::from(&x[..]))
                }

                b.iter(|| executor.new_generation::<$pattern>(&RULES))
            }
        }
    }
}
