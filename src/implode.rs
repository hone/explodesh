use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{self, DirEntry},
    path::Path,
    str::FromStr,
};

/// Deserialize path of files/folders into a `toml::Value`
pub fn deserialize_any(path: impl AsRef<Path>) -> anyhow::Result<toml::Value> {
    let attr = fs::metadata(&path)?;
    if attr.is_file() {
        let contents = fs::read_to_string(&path)?;
        let value = deserialize_bool(&contents)
            .or_else(|_| deserialize_str(&contents))
            .or_else(|_| deserialize_i64(&contents))
            .or_else(|_| deserialize_f64(&contents))
            .or_else(|_| deserialize_datetime(&contents));
        Ok(value?)
    } else if attr.is_dir() {
        let files = fs::read_dir(&path)?
            .filter_map(|entry| entry.ok())
            .collect();

        deserialize_array(&files).or_else(|_| deserialize_table(&files))
    } else {
        Err(anyhow!("Not a file or a dictory."))
    }
}

/// Deserialize string into a `toml::Value::Boolean`
/// # Examples
/// ```
/// use explodesh::implode;
///
/// assert_eq!(toml::Value::Boolean(true), implode::deserialize_bool("true").unwrap());
/// assert_eq!(toml::Value::Boolean(false), implode::deserialize_bool("false").unwrap());
/// assert!(!implode::deserialize_bool("foo").is_ok())
/// ```
pub fn deserialize_bool(input: impl AsRef<str>) -> anyhow::Result<toml::Value> {
    Ok(toml::Value::Boolean(input.as_ref().parse::<bool>()?))
}

/// Deserialize string into a `toml::Value::String`
/// # Examples
/// ```
/// use explodesh::implode;
///
/// assert_eq!(toml::Value::String(String::from("foo")), implode::deserialize_str("\"foo\"").unwrap());
/// ```
pub fn deserialize_str(input: impl AsRef<str>) -> anyhow::Result<toml::Value> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^"(.+)"$"#).unwrap();
    }
    match RE.captures(&input.as_ref().parse::<String>()?) {
        Some(captures) => {
            // if it returns any captures, it will always return the first capture group
            let string = captures.get(1).unwrap();
            Ok(toml::Value::String(String::from(string.as_str())))
        }
        None => Err(anyhow!("Not a valid TOML string")),
    }
}

/// Deserialize string into a `toml::Value::Integer`
/// # Examples
/// ```
/// use explodesh::implode;
///
/// assert_eq!(toml::Value::Integer(1), implode::deserialize_i64("1").unwrap());
/// ```
pub fn deserialize_i64(input: impl AsRef<str>) -> anyhow::Result<toml::Value> {
    Ok(toml::Value::Integer(input.as_ref().parse::<i64>()?))
}

/// Deserialize string into a `toml::Value::Float`
/// # Examples
/// ```
/// use explodesh::implode;
///
/// assert_eq!(toml::Value::Float(3.14), implode::deserialize_f64("3.14").unwrap());
/// ```
pub fn deserialize_f64(input: impl AsRef<str>) -> anyhow::Result<toml::Value> {
    Ok(toml::Value::Float(input.as_ref().parse::<f64>()?))
}

/// Deserialize string into a `toml::Value::Datetime`
/// # Examples
/// ```
/// use explodesh::implode;
/// use std::str::FromStr;
///
/// let date = "1979-05-27T07:32:00-08:00";
/// assert_eq!(toml::Value::Datetime(toml::value::Datetime::from_str(&date).unwrap()), implode::deserialize_datetime(&date).unwrap());
/// ```
pub fn deserialize_datetime(input: impl AsRef<str>) -> anyhow::Result<toml::Value> {
    Ok(toml::Value::Datetime(toml::value::Datetime::from_str(
        input.as_ref(),
    )?))
}

/// Deserialize a directory into a `toml::Value::Array`
/// # Examples
/// ```
/// use std::fs;
/// use tempfile::TempDir;
/// use explodesh::implode;
///
/// let tmp_dir = TempDir::new().unwrap();
/// fs::write(tmp_dir.path().join("0"), "true");
/// fs::write(tmp_dir.path().join("1"), "23");
/// fs::write(tmp_dir.path().join("2"), r#""hello""#);
/// let files = fs::read_dir(tmp_dir.path()).unwrap().filter_map(|entry| entry.ok()).collect();
/// let value = implode::deserialize_array(&files).unwrap();
///
/// assert_eq!(value[0], toml::Value::Boolean(true));
/// assert_eq!(value[1], toml::Value::Integer(23));
/// assert_eq!(value[2], toml::Value::String(String::from("hello")));
/// ```
pub fn deserialize_array(files: &Vec<DirEntry>) -> anyhow::Result<toml::Value> {
    // need to create a collection since `Iterator::map` will consume the iterator.
    let mut indexes: Vec<usize> = files
        .iter()
        .map(|entry| {
            entry
                .file_name()
                .as_os_str()
                .to_str()
                .unwrap_or("Not valid UTF-8")
                .parse::<usize>()
                .map_err(|_| "Invalid Integer")
        })
        .collect::<Result<Vec<usize>, &'static str>>()
        // the trait `StdError` is not implemented for Result<_, &str> so can't use `?'
        .map_err(|err| anyhow!(err))?;
    indexes.sort();
    indexes.dedup();
    if indexes[0] == 0
        && indexes[indexes.len() - 1] == files.len() - 1
        && indexes.len() == files.len()
    {
        let array = files
            .iter()
            .map(|entry| deserialize_any(&entry.path()).unwrap())
            .collect::<Vec<toml::value::Value>>();

        Ok(toml::Value::Array(array))
    } else {
        Err(anyhow!("Not a valid TOML array"))
    }
}

/// Deserialize a directory into a `toml::Value::Table`
/// # Examples
/// ```
/// use std::fs;
/// use tempfile::TempDir;
/// use explodesh::implode;
///
/// let tmp_dir = TempDir::new().unwrap();
/// fs::write(tmp_dir.path().join("foo"), r#""bar""#);
/// fs::write(tmp_dir.path().join("0"), "42");
/// let files = fs::read_dir(tmp_dir.path()).unwrap().filter_map(|entry| entry.ok()).collect();
/// let value = implode::deserialize_table(&files).unwrap();
///
/// assert_eq!(value.get("foo"), Some(&toml::Value::String(String::from("bar"))));
/// assert_eq!(value.get("0"), Some(&toml::Value::Integer(42)));
/// ```
pub fn deserialize_table(files: &Vec<DirEntry>) -> anyhow::Result<toml::Value> {
    let mut table = toml::value::Table::new();
    for entry in files.iter() {
        // this unwrap is handled by everything being a valid DirEntry
        let key = String::from(entry.file_name().as_os_str().to_str().unwrap());
        table.insert(key, deserialize_any(entry.path()).unwrap());
    }
    Ok(toml::Value::Table(table))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn arrays_must_have_sequential_keys() -> anyhow::Result<()> {
        let tmp_dir = TempDir::new()?;
        fs::write(tmp_dir.path().join("0"), "true")?;
        fs::write(tmp_dir.path().join("2"), "false")?;
        let files = fs::read_dir(tmp_dir.path())?
            .filter_map(|entry| entry.ok())
            .collect();

        assert!(deserialize_array(&files).is_err());

        Ok(())
    }
}
