use structopt::StructOpt;
use crate::error::Error;
use charsub::RuleCell;

#[derive(Debug, Clone, Copy)]
pub enum Pattern {
    Modulo,
    Const,
}

impl std::str::FromStr for Pattern {
    type Err = Error;

    fn from_str(x: &str) -> Result<Pattern, Self::Err> {
        Ok(match x.to_ascii_lowercase().as_str() {
            "const" => Pattern::Const,
            "modulo" => Pattern::Modulo,
            _ => return Err(Error::BadInput(format!("Expected pattern, got '{}'", x)))
        })
    }
}

#[derive(Debug)]
pub enum SchedulerInput {
    ThreadPool,
    AsyncRuntime,
    SingleThread,
}

impl std::str::FromStr for SchedulerInput {
    type Err = Error;

    fn from_str(x: &str) -> Result<SchedulerInput, Self::Err> {
        Ok(match x.to_ascii_lowercase().as_str() {
            "multithread" | "multi" => SchedulerInput::ThreadPool,
            "single" => SchedulerInput::SingleThread,
            "async" => SchedulerInput::AsyncRuntime,

            _ => return Err(Error::BadInput(format!("Expected SchedulerInput, got '{}'", x)))
        })
    }
}


#[derive(Debug, StructOpt)]
#[structopt(name = "Permutates input based on rules", about = "An example of StructOpt usage.")]
pub struct Opt {
    /// Describe which handler you'd like to use
    #[structopt(short, long, default_value="modulo")]
    pub pattern: Pattern,

    /// limit the amount of times it regenerates 
    #[structopt(short, long, default_value="0")]
    pub limit: usize,
    
    #[structopt(short, default_value="single")]
    pub scheduler: SchedulerInput,

    #[structopt(long, short = "-c", default_value="2", env="C_WORKERS")]
    pub workers: usize,

    /// The root of value of characters are being manipulated.
    #[structopt()]
    pub input: String,

    /// Substitute characters based on the notation of `from:to` 
    /// where `to` can be any length, but `from` is restricted to any characer
    /// example: 1:ilLI a:b b:dp
    #[structopt(parse(try_from_str=parse_rule))]
    pub rules: Vec<(u8, RuleCell)>,
}

fn parse_rule(buf: &str) -> Result<(u8, RuleCell), Box<dyn std::error::Error>> {
    const DELIMIER: char = ':';
    let mut iter = buf.split(DELIMIER);

    let root: u8 = iter.next().unwrap().as_bytes()[0];
    
    let mut rule_entry = RuleCell::new();
    rule_entry.extend(iter.next().unwrap().bytes());
    
    Ok((root, rule_entry))
}