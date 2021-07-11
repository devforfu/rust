use anyhow::Result;
use structopt::StructOpt;
use minigrep::{Cli, read_lines};

fn main() -> Result<()> {
    let args: Cli = Cli::from_args();
    let lines = read_lines(&args.path)?;
    for (i, line) in lines.enumerate() {
        println!("{}: {}", i, line?);
    }
    Ok(())
}
