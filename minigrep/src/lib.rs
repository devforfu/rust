use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        // read_lines()
    }

}