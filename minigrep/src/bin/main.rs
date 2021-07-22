use log::info;
use anyhow::Result;
use structopt::StructOpt;
use minigrep::{Cli, read_lines, Finder};

fn main() -> Result<()> {
    env_logger::init();

    let args: Cli = Cli::from_args();

    let f = Finder::new();

    info!("Searching for word `{}` in file `{}`", args.pattern, args.path);
    let lines = read_lines(&args.path)?.map(|x| x.unwrap());

    for matched in f.find(lines, &args.pattern) {
        println!("{}", matched);
    }

    Ok(())
}
