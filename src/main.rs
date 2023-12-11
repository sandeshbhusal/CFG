#![allow(non_snake_case)]

use anyhow::Result;
use cfg::cfg::CFG;
use clap::Parser;

#[derive(Debug, Parser)]
struct Arguments {
    bound_type: u8,
    cfg_file: String,
    str_file: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Arguments::parse();
    let cfg_def = std::fs::read_to_string(args.cfg_file)?;
    let input_string = std::fs::read_to_string(args.str_file)?.trim().to_string();
    let input_string = input_string.replace("!", "");

    let cfg = cfg_def.parse::<CFG>().unwrap();
    let bound = match args.bound_type {
        1 => 100,
        2 => {
            let b_h = input_string.len().max(1) as f64;
            ((b_h - 1.0f64) / (cfg.longest_derivation_length() as f64 - 1.0f64)).ceil() as isize
        }
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
