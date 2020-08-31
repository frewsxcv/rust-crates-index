use crate::{Crate, Error};
use std::path::PathBuf;

pub struct BareIndex {
    path: PathBuf,
    url: String,
}

impl BareIndex {
    pub fn from_url(url: &str) -> Result<Self, Error> {
        let (dir_name, canonical_url) = url_to_local_dir(url)?;
        let mut path = home::cargo_home().unwrap_or_default();

        path.push("registry/index");
        path.push(dir_name);

        Ok(Self {
            path,
            url: canonical_url,
        })
    }

    /// Creates and index for the default crates.io registry
    pub fn crates_io() -> Result<Self, Error> {
        Self::from_url(crate::INDEX_GIT_URL)
    }

    /// Opens the local index, which acts as a kind of lock for source control
    /// operations
    pub fn open(&'a self) -> BareIndexRepo<'a> {
        BareIndexRepo::new(self)
    }

    pub fn krate(&self, name: &str) -> Option<Crate> {
        let rel_path = match crate::crate_name_to_relative_path(name) {
            Some(rp) => rp,
            None => return None,
        };

        // Attempt to load the .cache/ entry first, this is purely an acceleration
        // mechanism and can fail for a few reasons that are non-fatal
        {
            let mut cache_path = self.path.join(".cache");
            cache_path.push(rel_path);
            if let Ok(cache_bytes) = std::fs::read(&cache_path) {
                if let Some(krate) = Crate::from_cache_slice(&cache_bytes) {
                    return Some(krate);
                }
            }
        }

        // // Fallback to reading the blob directly if we don't have a valid cache entry
        // let repo = self.repo()?;
        // let tree = self.tree()?;
        // let entry = tree.get_path(path)?;
        // let object = entry.to_object(repo)?;
        // let blob = match object.as_blob() {
        //     Some(blob) => blob,
        //     None => anyhow::bail!("path `{}` is not a blob in the git repo", path.display()),
        // };

        unimplemented!()
    }
}

pub struct BareIndexRepo<'a> {
    inner: &'a BareIndex,
    head: Option<git2::Oid>,
    repo: Option<git2::Repository>,
}

impl<'a> BareIndexRepo<'a> {
    fn new(index: &'a BareIndex) -> Self {
        Self {
            inner: index,
            head: None,
            repo: None,
        }
    }

    pub fn exists(&self) -> bool {
        git2::Repository::discover(&self.inner.path)
            .map(|repository| {
                repository
                    .find_remote("origin")
                    .ok()
                    // Cargo creates a checkout without an origin set,
                    // so default to true in case of missing origin
                    .map_or(true, |remote| {
                        remote.url().map_or(true, |url| url == self.inner.url)
                    })
            })
            .unwrap_or(false)
    }

/// Converts a full url, eg, into the root directory name where cargo itself
/// will fetch it on disk
fn url_to_local_dir(url: &str) -> Result<(String, String), Error> {
    fn to_hex(num: u64) -> String {
        const CHARS: &[u8] = b"0123456789abcdef";

        let bytes = &[
            num as u8,
            (num >> 8) as u8,
            (num >> 16) as u8,
            (num >> 24) as u8,
            (num >> 32) as u8,
            (num >> 40) as u8,
            (num >> 48) as u8,
            (num >> 56) as u8,
        ];

        let mut output = vec![0u8; 16];

        let mut ind = 0;

        for &byte in bytes {
            output[ind] = CHARS[(byte >> 4) as usize];
            output[ind + 1] = CHARS[(byte & 0xf) as usize];

            ind += 2;
        }

        String::from_utf8(output).expect("valid utf-8 hex string")
    }

    #[allow(deprecated)]
    fn hash_u64(url: &str) -> u64 {
        use std::hash::{Hash, Hasher, SipHasher};

        let mut hasher = SipHasher::new_with_keys(0, 0);
        // Registry
        2usize.hash(&mut hasher);
        // Url
        url.hash(&mut hasher);
        hasher.finish()
    }

    // Ensure we have a registry or bare url
    let (url, scheme_ind) = {
        let scheme_ind = url
            .find("://")
            .ok_or_else(|| Error::Url(format!("'{}' is not a valid url", url)))?;

        let scheme_str = &url[..scheme_ind];
        if let Some(ind) = scheme_str.find('+') {
            if &scheme_str[..ind] != "registry" {
                return Err(Error::Url(format!("'{}' is not a valid registry url", url)));
            }

            (&url[ind + 1..], scheme_ind - ind - 1)
        } else {
            (url, scheme_ind)
        }
    };

    // Could use the Url crate for this, but it's simple enough and we don't
    // need to deal with every possible url (I hope...)
    let host = match url[scheme_ind + 3..].find('/') {
        Some(end) => &url[scheme_ind + 3..scheme_ind + 3 + end],
        None => &url[scheme_ind + 3..],
    };

    // cargo special cases github.com for reasons, so do the same
    let mut canonical = if host == "github.com" {
        url.to_lowercase()
    } else {
        url.to_owned()
    };

    // Chop off any query params/fragments
    if let Some(hash) = canonical.rfind('#') {
        canonical.truncate(hash);
    }

    if let Some(query) = canonical.rfind('?') {
        canonical.truncate(query);
    }

    println!("hashing {}", canonical);
    let ident = to_hex(hash_u64(&canonical));

    if canonical.ends_with('/') {
        canonical.pop();
    }

    if canonical.ends_with(".git") {
        canonical.truncate(canonical.len() - 4);
    }

    Ok((format!("{}-{}", host, ident), canonical))
}

#[cfg(test)]
mod test {
    #[test]
    fn matches_cargo() {
        assert_eq!(
            super::url_to_local_dir(crate::INDEX_GIT_URL).unwrap(),
            (
                "github.com-1ecc6299db9ec823".to_owned(),
                crate::INDEX_GIT_URL.to_owned()
            )
        );

        // I've confirmed this also works with a custom registry, unfortunately
        // that one includes a secret key as part of the url which would allow
        // anyone to publish to the registry, so uhh...here's a fake one instead
        assert_eq!(
            super::url_to_local_dir(
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index.git"
            )
            .unwrap(),
            (
                "dl.cloudsmith.io-ff79e51ddd2b38fd".to_owned(),
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index".to_owned()
            )
        );

        // Ensure we actually strip off the irrelevant parts of a url, note that
        // the .git suffix is not part of the canonical url, but it used when hashing
        assert_eq!(
            super::url_to_local_dir(&format!(
                "registry+{}.git?one=1&two=2#fragment",
                crate::INDEX_GIT_URL
            ))
            .unwrap(),
            (
                "github.com-c786010fb7ef2e6e".to_owned(),
                crate::INDEX_GIT_URL.to_owned()
            )
        );
    }
}
