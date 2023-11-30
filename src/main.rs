#![allow(non_snake_case)]

use anyhow::Result;
use cfg::cfg::CFG;
use cfg::pda::PDA;
use cfg::trace;
use clap::Parser;

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(short = 'b', long = "bound-type", help = "The bound type to use.")]
    bound_type: u8,
    #[clap(
        short = 'f',
        long = "cfg-file",
        help = "The file containing the CFG definition."
    )]
    cfg_file: String,
    #[clap(
        short = 's',
        long = "string-file",
        help = "The file containing the string to test."
    )]
    str_file: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Arguments::parse();
    let cfg_def = std::fs::read_to_string(args.cfg_file)?;
    let input_string = std::fs::read_to_string(args.str_file)?;

    // Replace all "!" with blanks.
    let input_string = input_string.replace("!", "");

    // TODO: Calculate the value of bound we have to parse within.

    let cfg = cfg_def.parse::<CFG>().unwrap();

    let pda = PDA::from(cfg);
    // println!("{}", pda);
    let mut tracer = trace::PDAConfiguration::with_pda(pda, &input_string, 100);
    let ans = tracer.trace();

    if ans {
        println!("yes");
    } else {
        println!("no");
    }
    Ok(())
}
