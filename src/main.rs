use anyhow::Result;
use cfg::CFG;
use clap::Parser;

mod parser;
mod cfg;
mod pda;
mod cnf;

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(short = 'b', long = "bound-type", help = "The bound type to use.")]
    bound_type: u8,
    #[clap(short = 'f', long = "cfg-file", help = "The file containing the CFG definition.")]
    cfg_file: String,
    #[clap(short = 's', long = "string-file", help = "The file containing the string to test.")]
    str_file: String
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    let cfg_def = std::fs::read_to_string(args.cfg_file).unwrap();
    let _def = cfg_def.parse::<CFG>().unwrap();   
    Ok(())
}
