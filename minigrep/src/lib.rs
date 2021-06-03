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
