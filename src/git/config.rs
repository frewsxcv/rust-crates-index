use crate::Error;
use std::path::{Path, PathBuf};

/// Calls the specified function for each cargo config located according to
/// cargo's standard hierarchical structure
///
/// Note that this only supports the use of `.cargo/config.toml`, which is not
/// supported below cargo 1.39.0
///
/// See https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure
fn read_cargo_config<T>(
    root: Option<&Path>,
    cargo_home: Option<&Path>,
    callback: impl Fn(&toml::Value) -> Option<T>,
) -> Result<Option<T>, Error> {
    use std::borrow::Cow;

    if let Some(mut path) = root.map(PathBuf::from).or_else(|| std::env::current_dir().ok()) {
        loop {
            path.push(".cargo/config.toml");
            if let Some(toml) = try_read_toml(&path)? {
                if let Some(value) = callback(&toml) {
                    return Ok(Some(value));
                }
            }
            path.pop();
            path.pop();

            // Walk up to the next potential config root
            if !path.pop() {
                break;
            }
        }
    }

    if let Some(home) = cargo_home
        .map(Cow::Borrowed)
        .or_else(|| home::cargo_home().ok().map(Cow::Owned))
    {
        let path = home.join("config.toml");
        if let Some(toml) = try_read_toml(&path)? {
            if let Some(value) = callback(&toml) {
                return Ok(Some(value));
            }
        }
    }

    Ok(None)
}

fn try_read_toml(path: &Path) -> Result<Option<toml::Value>, Error> {
    if !path.exists() {
        return Ok(None);
    }

    let toml = toml::from_str(&std::fs::read_to_string(path)?).map_err(Error::Toml)?;
    Ok(Some(toml))
}

/// Gets the url of a replacement registry for crates.io if one has been configured
///
/// See https://doc.rust-lang.org/cargo/reference/source-replacement.html
#[inline]
pub(crate) fn get_crates_io_replacement(
    root: Option<&Path>,
    cargo_home: Option<&Path>,
) -> Result<Option<String>, Error> {
    read_cargo_config(root, cargo_home, |config| {
        config.get("source").and_then(|sources| {
            sources
                .get("crates-io")
                .and_then(|v| v.get("replace-with"))
                .and_then(|v| v.as_str())
                .and_then(|v| sources.get(v))
                .and_then(|v| v.get("registry"))
                .and_then(|v| v.as_str().map(String::from))
        })
    })
}
