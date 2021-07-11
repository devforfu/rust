use anyhow::{Result, Context};
use std::fs::File;
use std::io::{self, BufRead};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    // #[structopt(parse(from_os_str))]
    pub path: String,
}

pub fn read_lines(filename: &str) -> Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename).with_context(|| {
        format!("could not read file `{}`", filename)
    })?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    use std::io::Write;

    static INIT: Once = Once::new();
    static TEST_FILE: &str = "/tmp/test.txt";

    fn init() {
        INIT.call_once(|| {
            create_test_file("first\nsecond\nthird");
        });
    }

    fn create_test_file(content: &str) {
        let mut file = File::create(TEST_FILE).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_read_lines() -> Result<()> {
        init();

        let lines: Vec<String> = read_lines(TEST_FILE)?.map(|line| line.unwrap()).collect();

        assert_eq!(lines, vec!["first", "second", "third"]);
        Ok(())
    }

}