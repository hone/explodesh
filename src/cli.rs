use clap::Clap;
use std::str::FromStr;

/// Tool for converting TOML files to a set key/value files/folders
#[derive(Clap)]
#[clap(version = "0.1", author = "Terence Lee <hone02@gmail.com>")]
pub struct Cli {
    /// 'explode' take a TOML file and convert to a filesystem layout.
    /// 'implode' will take a filesystem layout and construct a TOML file.
    #[clap(possible_values=&["explode", "implode"])]
    pub cmd: Command,
    /// Path to the source input
    pub source: String,
    /// Path to where the output is written
    pub destination: String,
}

#[derive(Clap)]
pub enum Command {
    Explode,
    Implode,
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "explode" => Ok(Command::Explode),
            "implode" => Ok(Command::Implode),
            _ => Err("Invalid Command"),
        }
    }
}
