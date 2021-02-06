use clap::Clap;
use explodesh::{
    cli::{Cli, Command},
    explode, implode,
};
use std::{fs, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let opts: Cli = Cli::parse();

    let source = PathBuf::from(opts.source);
    let destination = PathBuf::from(opts.destination);

    match opts.cmd {
        Command::Explode => {
            let doc = toml::from_str(&fs::read_to_string(source)?)?;
            explode::visit_value(&doc, destination)?;
        }
        Command::Implode => {
            let doc = implode::walk(&source)?;
            fs::write(&destination, toml::to_string(&doc)?)?;
        }
    }

    Ok(())
}
