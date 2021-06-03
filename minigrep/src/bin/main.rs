use structopt::StructOpt;
use minigrep::Cli;

fn main() {
    println!("{:?}", Cli::from_args());
}
