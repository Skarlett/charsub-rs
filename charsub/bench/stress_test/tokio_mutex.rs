
use crate::gen_bench_stress_test;
use charsub::{ConstPattern, ModulusPattern, scheduler::TokioMutex};

gen_bench_stress_test!(TokioMutex, ConstPattern, 1, 1, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 1, 2, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 1, 4, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 1, 8, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 1, 16, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 1, 32, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 1, 64, 1);


gen_bench_stress_test!(TokioMutex, ConstPattern, 2, 1, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 2, 2, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 2, 4, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 2, 8, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 2, 16, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 2, 32, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 2, 64, 1);


gen_bench_stress_test!(TokioMutex, ConstPattern, 4, 1, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 4, 2, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 4, 4, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 4, 8, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 4, 16, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 4, 32, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 4, 64, 1);


gen_bench_stress_test!(TokioMutex, ConstPattern, 8, 1, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 8, 2, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 8, 4, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 8, 8, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 8, 16, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 8, 32, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 8, 64, 1);


gen_bench_stress_test!(TokioMutex, ConstPattern, 16, 1, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 16, 2, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 16, 4, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 16, 8, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 16, 16, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 16, 32, 1);
gen_bench_stress_test!(TokioMutex, ConstPattern, 16, 64, 1);




gen_bench_stress_test!(TokioMutex, ModulusPattern, 1, 2, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 1, 4, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 1, 8, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 1, 16, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 1, 32, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 1, 64, 1);




gen_bench_stress_test!(TokioMutex, ModulusPattern, 2, 1, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 2, 2, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 2, 4, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 2, 8, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 2, 16, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 2, 32, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 2, 64, 1);




gen_bench_stress_test!(TokioMutex, ModulusPattern, 4, 1, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 4, 2, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 4, 4, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 4, 8, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 4, 16, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 4, 32, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 4, 64, 1);




gen_bench_stress_test!(TokioMutex, ModulusPattern, 8, 1, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 8, 2, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 8, 4, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 8, 8, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 8, 16, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 8, 32, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 8, 64, 1);




gen_bench_stress_test!(TokioMutex, ModulusPattern, 16, 1, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 16, 2, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 16, 4, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 16, 8, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 16, 16, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 16, 32, 1);
gen_bench_stress_test!(TokioMutex, ModulusPattern, 16, 64, 1);