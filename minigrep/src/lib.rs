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
mod utils {
    use std::fs::File;
    use rand::{rngs::OsRng, seq::SliceRandom, Rng};
    use std::io::Write;

    const WORDS: &'static [&'static str] = &include!("../res/words.json");
    const CHARS: &[u8] = b"abcdef0123456789";

    pub fn get_file_name(size: u32) -> &'static str {
        let mut rng = OsRng;
        let n = CHARS.len();
        let chars = (0..size).map(|_| {
            let index = rng.gen_range(0..n);
            &CHARS[index]
        });
        let filename = chars.join("");
        filename
    }

    pub fn create_fixed_test_file(path: &str, content: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    pub fn create_random_test_file(path: &str, n_lines: u32, words_per_line: (u32, u32)) {
        let (min, max) = words_per_line;

        let mut rng = OsRng;

        let mut file = File::create(path).unwrap();

        (0..n_lines).for_each(|_| {
            let n_words = rng.gen_range(min..max + 1);
            let words: Vec<String> =
                (0..n_words)
                    .map(|_| String::from(*WORDS.choose(&mut rng).unwrap()))
                    .collect();
            let line: String = words.join(" ");
            file.write_all(line.as_bytes()).unwrap();
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() -> Result<()> {
        utils::create_fixed_test_file(TEST_FILE, "first\nsecond\nthird");

        let lines: Vec<String> = read_lines(TEST_FILE)?.map(|line| line.unwrap()).collect();

        assert_eq!(lines, vec!["first", "second", "third"]);
        Ok(())
    }

    #[test]
    fn test_find_matched_lines() {
        utils::create_random_test_file(TEST_FILE, 10, (8, 12));
    }
}