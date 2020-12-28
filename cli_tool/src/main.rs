mod multi_runtime;
mod input;
mod error;

use charsub::{
    Generator,
    ModulusPattern,
    ConstPattern,
    Handler,
    Permutation
};

#[derive(Debug, Default)]
struct Binary;
impl Handler for Binary {
    fn handle(permute: &Permutation) -> bool {
        ModulusPattern::handle(permute) || ConstPattern::handle(permute) 
    }
}

use structopt::StructOpt;
use crate::input::Pattern;

fn main() {
    let opt = input::Opt::from_args();
    let pattern = opt.pattern;
    
    println!("{:#?}", &opt);

    let mut gen = Generator::new(
        opt.rules.clone(),
        multi_runtime::Schedules::from_opt(&opt)
    );
    gen.seed(opt.input.as_bytes());

    println!("{:#?}", &gen);
    let mut generation = 0;

    loop {
        let last = gen.length();
        //println!("{:?}", &gen);
        match pattern {
            Pattern::Const => gen.new_generation::<ConstPattern>(),
            Pattern::Modulo => gen.new_generation::<ModulusPattern>(),
        };
        
        if gen.length()-last == 0 && gen.done() { break }    
        if opt.limit != 0 && gen.generation() >= opt.limit { break }

        generation += 1;
    }

    println!("Generations: {}", generation);
    
    for x in gen.into_set() {
        println!("{}", String::from_utf8_lossy(&x))
    }
}

