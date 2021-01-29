use clap::Clap;
use std::str::FromStr;

#[derive(Clap)]
#[clap(version = "0.1", author = "Terence Lee <hone02@gmail.com>")]
pub struct Cli {
    #[clap(possible_values=&["explode", "implode"])]
    pub cmd: Command,
    pub source: String,
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
