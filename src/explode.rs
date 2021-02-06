use serde::ser::Serialize;
use std::{fs, path::Path};

/// Leaf node visitor method for serializing non-collection `toml::Value`s into a string on disk.
/// # Examples
/// ```
/// use explodesh::explode;
/// use tempfile::NamedTempFile;
/// let file = NamedTempFile::new().unwrap();
/// explode::visit_serialize(toml::Value::String(String::from("hello")), file.path());
/// assert_eq!("\"hello\"", std::fs::read_to_string(file.path()).unwrap());
/// ```
pub fn visit_serialize(value: impl Serialize, path: impl AsRef<Path>) -> anyhow::Result<()> {
    Ok(fs::write(path, toml::to_string(&value)?)?)
}

/// Visitor method for serializing `toml::Value::Table` variant on disk.
/// # Examples
/// ```
/// use explodesh::explode;
/// use std::fs;
/// use tempfile::tempdir;
///
/// let dir = tempdir().unwrap();
/// let mut table = toml::value::Table::new();
/// table.insert(String::from("foo"), toml::Value::String(String::from("hello")));
/// table.insert(String::from("bar"), toml::Value::String(String::from("world")));
/// explode::visit_table(&table, dir.path());
///
/// assert_eq!("\"hello\"", fs::read_to_string(dir.path().join("foo")).unwrap());
/// assert_eq!("\"world\"", fs::read_to_string(dir.path().join("bar")).unwrap());
/// ```
pub fn visit_table(table: &toml::value::Table, path: impl AsRef<Path>) -> anyhow::Result<()> {
    fs::create_dir_all(&path)?;
    for (key, val) in table.iter() {
        visit_value(val, path.as_ref().join(key))?
    }

    Ok(())
}

/// Visitor method for serializing `toml::Value::Array` variant on disk.
/// # Examples
/// ```
/// use explodesh::explode;
/// use std::fs;
/// use tempfile::tempdir;
///
/// let dir = tempdir().unwrap();
/// let array = vec!["foo", "bar", "baz"]
///         .into_iter()
///         .map(|s| toml::Value::String(String::from(s)))
///         .collect::<Vec<toml::Value>>();
/// explode::visit_array(&array, dir.path());
///
/// assert_eq!("\"foo\"", fs::read_to_string(dir.path().join("0")).unwrap());
/// assert_eq!("\"bar\"", fs::read_to_string(dir.path().join("1")).unwrap());
/// assert_eq!("\"baz\"", fs::read_to_string(dir.path().join("2")).unwrap());
/// ```
pub fn visit_array(array: &toml::value::Array, path: impl AsRef<Path>) -> anyhow::Result<()> {
    for (i, val) in array.iter().enumerate() {
        visit_value(val, path.as_ref().join(i.to_string()))?
    }

    Ok(())
}

/// Visitor for serializing `toml::Value`
pub fn visit_value(value: &toml::Value, path: impl AsRef<Path>) -> anyhow::Result<()> {
    match value {
        toml::Value::Table(table) => visit_table(&table, path)?,
        toml::Value::Array(array) => visit_array(&array, path)?,
        val => visit_serialize(val, path)?,
    }

    Ok(())
}
