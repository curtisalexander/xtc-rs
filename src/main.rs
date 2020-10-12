use structopt::StructOpt;

use xtc::Args;
use std::process;

fn main() {
    let args: Args = Args::from_args();
    if let Err(e) = xtc::run(args) {
        println!("Stopping with error: {}", e);
        process::exit(1);
    }
    process::exit(0);
}