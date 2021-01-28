use clap::Clap;
use serde::ser::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Clap)]
#[clap(version = "0.1", author = "Terence Lee <hone02@gmail.com>")]
struct Opts {
    #[clap(possible_values=&["explode", "implode"])]
    cmd: Command,
    source: String,
    destination: String,
}

#[derive(Clap)]
enum Command {
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

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let source = PathBuf::from(opts.source);
    let destination = PathBuf::from(opts.destination);

    match opts.cmd {
        Command::Explode => {
            let doc: toml::Value = toml::from_str(&fs::read_to_string(source)?)?;
            visit_value(&doc, destination)?;
        }
        Command::Implode => {
            println!("IMPLODE");
        }
    }

    Ok(())
}

/// Leaf node visitor method for serializing non-collection `toml::Value`s into a string on disk.
fn visit_serialize(value: impl Serialize, path: impl AsRef<Path>) -> anyhow::Result<()> {
    Ok(fs::write(path, toml::to_string(&value)?)?)
}

/// Visitor method for serializing `toml::Value::Table` variant on disk.
fn visit_table(table: &toml::value::Table, path: impl AsRef<Path>) -> anyhow::Result<()> {
    fs::create_dir_all(&path)?;
    for (key, val) in table.iter() {
        visit_value(val, path.as_ref().join(key))?
    }

    Ok(())
}

/// Visitor method for serializing `toml::Value::Array` variant on disk.
fn visit_array(array: &toml::value::Array, path: impl AsRef<Path>) -> anyhow::Result<()> {
    for (i, val) in array.iter().enumerate() {
        visit_value(val, path.as_ref().join(i.to_string()))?
    }

    Ok(())
}

/// Visitor for serializing `toml::Value`
fn visit_value(value: &toml::Value, path: impl AsRef<Path>) -> anyhow::Result<()> {
    match value {
        toml::Value::Table(table) => visit_table(&table, path)?,
        toml::Value::Array(array) => visit_array(&array, path)?,
        val => visit_serialize(val, path)?,
    }

    Ok(())
}
