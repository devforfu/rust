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
    pub path: String,
}

pub fn read_lines(filename: &str) -> Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename).with_context(|| {
        format!("could not read file `{}`", filename)
    })?;
    Ok(io::BufReader::new(file).lines())
}

pub fn find_word(lines: &Vec<&str>, word: &str, ignore_case: bool) -> Vec<usize> {
    let mut matched: Vec<usize> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        match if ignore_case {
            line.to_lowercase().find(&word.to_lowercase())
        } else {
            line.find(word)
        } {
            Some(_) => { matched.push(i); }
            None => {}
        }
    }
    matched
}

#[cfg(test)]
mod utils {
    use std::fs::File;
    use std::io::Write;
    use std::str;
    use rand::{Rng, rngs::OsRng};
    use rand::prelude::SliceRandom;

    const WORDS: &'static [&'static str] = &include!("../res/words.json");
    const CHARS: &[u8] = b"abcdef0123456789";

    pub fn create_fixed_test_file(content: &str) -> String {
        let mut factory = RandomFactory::new();
        factory.tmp_file(content)
    }

    pub fn create_random_test_file(n_lines: u32, words_per_line: (u32, u32)) -> String {
        let mut factory = RandomFactory::new();
        factory.random_tmp_file(n_lines, words_per_line)
    }

    /// Provides a set of methods to generate random data for testing.
    pub struct RandomFactory {
        rng: OsRng
    }

    impl RandomFactory {
        pub fn new() -> RandomFactory {
            RandomFactory { rng: OsRng }
        }

        /// Generates a random hex string of requested `size`.
        pub fn hex_string(&mut self, size: usize) -> String {
            let n = CHARS.len();
            let chars: Vec<u8> = (0..size).map(|_| CHARS[self.rng.gen_range(0..n)]).collect();
            String::from(str::from_utf8(&chars).unwrap())
        }

        /// Creates a temporary file with a given `content` and random name and returns its path.
        pub fn tmp_file(&mut self, content: &str) -> String {
            let path = self.create_random_path();
            let mut file = File::create(&path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            String::from(path)
        }

        /// Creates a temporary file with random content.
        ///
        /// The generated file has `n_lines` with a random number of words within `words_per_line` range.
        pub fn random_tmp_file(&mut self, n_lines: u32, words_per_line: (u32, u32)) -> String {
            let (min, max) = words_per_line;

            let path = self.create_random_path();

            let mut file = File::create(&path).unwrap();

            (0..n_lines).for_each(|_| {
                let n_words = self.rng.gen_range(min..max + 1);
                let words: Vec<String> =
                    (0..n_words)
                        .map(|_| String::from(*WORDS.choose(&mut self.rng).unwrap()))
                        .collect();
                let line: String = format!("{}\n", words.join(" "));
                file.write_all(line.as_bytes()).unwrap();
            });

            path
        }

        fn create_random_path(&mut self) -> String {
            format!("/tmp/{}.txt", self.hex_string(20))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() -> Result<()> {
        let test_path = utils::create_fixed_test_file("first\nsecond\nthird");

        let lines: Vec<String> = read_lines(&test_path)?.map(|line| line.unwrap()).collect();

        assert_eq!(lines, vec!["first", "second", "third"]);
        Ok(())
    }

    #[test]
    fn test_number_of_lines_is_correct() -> Result<()> {
        let test_path = utils::create_random_test_file(10, (8, 12));

        let lines: Vec<String> = read_lines(&test_path)?.map(|line| line.unwrap()).collect();

        assert_eq!(lines.len(), 10);
        Ok(())
    }

    #[test]
    fn test_find_matched_substring() {
        struct SearchTest {
            name: &'static str,
            ignore_case: bool,
            word: &'static str,
            expected: Vec<usize>,
        }

        let lines = vec!["The first line.", "And the second one.", "The very last line."];

        let test_cases = vec![
            SearchTest {
                name: "case sensitive",
                ignore_case: false,
                word: "the",
                expected: vec![1],
            },
            SearchTest {
                name: "ignore case",
                ignore_case: true,
                word: "the",
                expected: vec![0, 1, 2],
            },
            SearchTest {
                name: "first and last (ignore case)",
                ignore_case: true,
                word: "line",
                expected: vec![0, 2],
            },
            SearchTest {
                name: "first and last (case sensitive)",
                ignore_case: false,
                word: "line",
                expected: vec![0, 2],
            }
        ];

        for test in test_cases {
            let matched = find_word(&lines, test.word, test.ignore_case);

            assert_eq!(matched, test.expected, "test case failed: {}", test.name);
        }
    }
}