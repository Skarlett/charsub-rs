use charsub::{
    Cell,
    RuleCell,
    Generator,
    RuleEntry,
    ConstPattern,
    ModulusPattern,
    Handler,
    UnitPair
};

use structopt::StructOpt;
use std::collections::HashMap;
use smallvec::SmallVec;

#[derive(Debug)]
enum Error {
    BadHandler
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, x: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(x, "{:?}", &self);
        Ok(())
    }
}

#[derive(Debug)]
struct State;

fn parse_rule(buf: &str) -> Result<(u8, RuleEntry), Box<dyn std::error::Error>> {
    const DELIMIER: char = ':';
    let mut iter = buf.split(DELIMIER);

    let root: u8 = iter.next().unwrap().as_bytes()[0];
    
    let mut rule_entry = RuleCell::new();
    rule_entry.extend(iter.next().unwrap().bytes());
    
    Ok((root, RuleEntry::Multi(rule_entry)))
}

#[derive(Debug)]
enum HandlerInput {
    Const,
    Modolo,
    All
}

enum Pattern {
    Const(ConstPattern),
    Modulo(ModulusPattern),
    Multi(SmallVec<[Box<Handler>; 12]>)
}

impl Handler for Pattern {
    fn handle(&mut self, permute: &UnitPair<'_>) -> bool {
        match self {
            Pattern::Const(hdlr) => hdlr.handle(permute),
            Pattern::Modulo(hdlr) => hdlr.handle(permute),
            Pattern::Multi(vec) => vec.iter_mut().any(|x| x.handle(permute))
        }
    }
}

impl From<HandlerInput> for Pattern {
    fn from(x: HandlerInput) -> Pattern {
        match x {
            HandlerInput::Modolo => Pattern::Modulo(Default::default()),
            HandlerInput::Const => Pattern::Const(Default::default()),
            All => unimplemented!()
        }
    }
}

impl std::str::FromStr for HandlerInput {
    type Err = Error;

    fn from_str(x: &str) -> Result<HandlerInput, Self::Err> {
        Ok(match x.to_ascii_lowercase().as_str() {
            "const" => HandlerInput::Const,
            "modolo" => HandlerInput::Modolo,
            "all" => HandlerInput::All,
            _ => return Err(Error::BadHandler)
        })
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Permutates input based on rules", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long, default_value="modolo")]
    handler: HandlerInput,

    /// Input file
    #[structopt()]
    input: String,

    #[structopt(short, long, default_value="0")]
    generation: usize,

    /// Output file, stdout if not present
    #[structopt(parse(try_from_str=parse_rule))]
    rules: Vec<(u8, RuleEntry)>,
}

struct ConstructedOpts {
    handler: Pattern,
    root: Cell,
    rules: HashMap<u8, RuleEntry>,
    until: usize
}

impl From<Opt> for ConstructedOpts {
    fn from(x: Opt) -> ConstructedOpts {
        ConstructedOpts {
            handler: Pattern::from(x.handler),
            until: x.generation,
            root: {
                let mut cell = Cell::new();
                cell.extend(x.input.bytes());
                cell
            },

            rules: {
                let mut map = HashMap::new();
                for (k, r) in x.rules {
                    map.insert(k, r);
                }
                map
            }
        }
    }
}

fn main() {
    let mut opt = ConstructedOpts::from(Opt::from_args());

    let mut gen = Generator::new(&opt.root, &opt.rules);
    let mut generation = 0;
    
    
    loop {
        if gen.new_generation(&mut opt.handler) == 0 { break }    
        if opt.until != 0 && generation >= opt.until { break }

        generation += 1;
        //println!("last [{}:{}]: {:?}...", gen.current_generation(), gen.last_idx(), gen.last_generation().iter().take(10).collect::<SmallVec<[&Cell; 16]>>())
    }
    println!("Generations: {}", generation);
    
    let mut output = Vec::with_capacity(gen.buf().len()+1);
    output.extend(gen.into_buf());
    output.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for x in &output {
        println!("{}", String::from_utf8_lossy(x))
    }
}