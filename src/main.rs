#![allow(non_snake_case)]

use anyhow::Result;
use cfg::cfg::CFG;
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
    let input_string = input_string.replace("!", "");

    let cfg = cfg_def.parse::<CFG>().unwrap();
    let bound = match args.bound_type {
        1 => 100,
        2 => input_string.len().max(1) as isize,
        3 => ((2 * input_string.len()).max(1) - 1).max(1) as isize,
        _ => {
            panic!("Illegal bound passed.")
        }
    };

    // dbg!(&input_string, &bound);

    let ans = cfg.trace_string(input_string.as_str(), bound);

    if ans {
        println!("yes");
    } else {
        println!("no");
    }

    Ok(())
}
