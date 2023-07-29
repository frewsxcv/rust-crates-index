use serde_derive::Deserialize;
use crate::dirs::crate_prefix;

/// Global configuration of an index, reflecting the [contents of config.json](https://doc.rust-lang.org/cargo/reference/registries.html#index-format).
#[derive(Clone, Debug, Deserialize)]
pub struct IndexConfig {
    /// Pattern for creating download URLs. Use [`IndexConfig::download_url`] instead.
    pub dl: String,
    /// Base URL for publishing, etc.
    pub api: Option<String>,
}

impl IndexConfig {
    /// Get the URL from where the specified package can be downloaded.
    /// This method assumes the particular version is present in the registry,
    /// and does not verify that it is.
    #[must_use]
    pub fn download_url(&self, name: &str, version: &str) -> Option<String> {
        if !self.dl.contains("{crate}")
            && !self.dl.contains("{version}")
            && !self.dl.contains("{prefix}")
            && !self.dl.contains("{lowerprefix}")
        {
            let mut new = String::with_capacity(self.dl.len() + name.len() + version.len() + 10);
            new.push_str(&self.dl);
            new.push('/');
            new.push_str(name);
            new.push('/');
            new.push_str(version);
            new.push_str("/download");
            Some(new)
        } else {
            let mut prefix = String::with_capacity(5);
            crate_prefix(&mut prefix, name, '/')?;
            Some(
                self.dl
                    .replace("{crate}", name)
                    .replace("{version}", version)
                    .replace("{prefix}", &prefix)
                    .replace("{lowerprefix}", &prefix.to_ascii_lowercase()),
            )
        }
    }
}
