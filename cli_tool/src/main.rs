use charsub::{
    RuleCell,
    Generator,
    Pattern,
    Rulebook
};

use structopt::StructOpt;

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

fn parse_rule(buf: &str) -> Result<(u8, RuleCell), Box<dyn std::error::Error>> {
    const DELIMIER: char = ':';
    let mut iter = buf.split(DELIMIER);

    let root: u8 = iter.next().unwrap().as_bytes()[0];
    
    let mut rule_entry = RuleCell::new();
    rule_entry.extend(iter.next().unwrap().bytes());
    
    Ok((root, rule_entry))
}

#[derive(Debug)]
struct Handler(Pattern);

impl std::str::FromStr for Handler {
    type Err = Error;

    fn from_str(x: &str) -> Result<Handler, Self::Err> {
        Ok(Handler(match x.to_ascii_lowercase().as_str() {
            "const" => Pattern::Modulo(Default::default()),
            "modulo" => Pattern::Const(Default::default()),
            "all" => unimplemented!(),
            _ => return Err(Error::BadHandler)
        }))
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Permutates input based on rules", about = "An example of StructOpt usage.")]
struct Opt {
    /// Describe which handler you'd like to use
    #[structopt(short, long, default_value="modulo")]
    handler: Handler,

    /// The root of value of characters are being manipulated.
    #[structopt()]
    input: String,

    /// limit the amount of times it regenerates 
    #[structopt(short, long, default_value="0")]
    limit: usize,

    /// Substitute characters based on the notation of `from:to` 
    /// where `to` can be any length, but `from` is restricted to any characer
    /// example: 1:ilLI a:b b:dp
    #[structopt(parse(try_from_str=parse_rule))]
    rules: Vec<(u8, RuleCell)>,
}

struct ConstructedOpts {
    handler: Pattern,
    root: String,
    rules: Rulebook,
    until: usize
}
impl From<Opt> for ConstructedOpts {
    fn from(x: Opt) -> ConstructedOpts {
        ConstructedOpts {
            handler: Pattern::from(x.handler.0),
            until: x.limit,
            root: x.input,
            rules: x.rules.into()
        }
    }
}

fn main() {
    let mut opt: ConstructedOpts = Opt::from_args().into();

    let mut gen = Generator::new(opt.root.as_bytes(), opt.rules);
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