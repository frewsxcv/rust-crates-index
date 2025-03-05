use crate::Error;

/// Get the disk location of the specified `url`, as well as its canonical form,
/// exactly as cargo would.
///
/// `cargo_home` is used to root the directory at specific location, if not
/// specified `CARGO_HOME` or else the default cargo location is used as the root.
pub fn local_path_and_canonical_url(
    url: &str,
    cargo_home: Option<&std::path::Path>,
) -> Result<(std::path::PathBuf, String), Error> {
    local_path_and_canonical_url_with_hash_kind(url, cargo_home, &DEFAULT_HASHER_KIND)
}

/// Like [`local_path_and_canonical_url`] but accepts [`HashKind`] for determining the crate index path.
pub fn local_path_and_canonical_url_with_hash_kind(
    url: &str,
    cargo_home: Option<&std::path::Path>,
    hash_kind: &HashKind,
) -> Result<(std::path::PathBuf, String), Error> {
    let (dir_name, canonical_url) = url_to_local_dir(url, hash_kind)?;

    let mut path = match cargo_home {
        Some(path) => path.to_owned(),
        None => home::cargo_home()?,
    };

    path.push("registry");
    path.push("index");
    path.push(dir_name);

    Ok((path, canonical_url))
}

pub(crate) fn crate_prefix(accumulator: &mut String, crate_name: &str, separator: char) -> Option<()> {
    match crate_name.len() {
        0 => return None,
        1 => accumulator.push('1'),
        2 => accumulator.push('2'),
        3 => {
            accumulator.push('3');
            accumulator.push(separator);
            accumulator.extend(
                crate_name
                    .as_bytes()
                    .get(0..1)?
                    .iter()
                    .map(|c| c.to_ascii_lowercase() as char),
            );
        }
        _ => {
            accumulator.extend(
                crate_name
                    .as_bytes()
                    .get(0..2)?
                    .iter()
                    .map(|c| c.to_ascii_lowercase() as char),
            );
            accumulator.push(separator);
            accumulator.extend(
                crate_name
                    .as_bytes()
                    .get(2..4)?
                    .iter()
                    .map(|c| c.to_ascii_lowercase() as char),
            );
        }
    };
    Some(())
}

pub(crate) fn crate_name_to_relative_path(crate_name: &str, separator: Option<char>) -> Option<String> {
    let separator = separator.unwrap_or(std::path::MAIN_SEPARATOR);
    let mut rel_path = String::with_capacity(crate_name.len() + 6);
    crate_prefix(&mut rel_path, crate_name, separator)?;
    rel_path.push(separator);
    rel_path.extend(crate_name.as_bytes().iter().map(|c| c.to_ascii_lowercase() as char));

    Some(rel_path)
}

/// Matches https://github.com/rust-lang/cargo/blob/2928e32734b04925ee51e1ae88bea9a83d2fd451/crates/cargo-util-schemas/src/core/source_kind.rs#L5
type SourceKind = u64;
const SOURCE_KIND_REGISTRY: SourceKind = 2;
const SOURCE_KIND_SPASE_REGISTRY: SourceKind = 3;

/// Determine the crate registry hashing strategy for locating local crate indexes.
#[derive(Debug)]
pub enum HashKind {
    /// Use the new hashing behavior introduced in Rust `1.85.0`.
    Stable,

    /// Use a hashing strategy that matches Cargo versions less than `1.85.0`
    Legacy,
}

// For now, this acts as a centralized place to change the default. Ideally
// this would be compiled conditionally based on the version of rustc as
// a nice approximation of when consumers will be using the associated hash
// implementation but this behavior is not yet stable: https://github.com/rust-lang/rust/issues/64796
pub(crate) const DEFAULT_HASHER_KIND: HashKind = HashKind::Legacy;

/// Converts a full url, eg https://github.com/rust-lang/crates.io-index, into
/// the root directory name where cargo itself will fetch it on disk
fn url_to_local_dir(url: &str, hash_kind: &HashKind) -> Result<(String, String), Error> {
    #[allow(deprecated)]
    fn legacy_hash_u64(url: &str, registry_kind: u64) -> u64 {
        use std::hash::{Hash, Hasher, SipHasher};

        let mut hasher = SipHasher::new_with_keys(0, 0);
        // Registry
        registry_kind.hash(&mut hasher);
        // Url
        url.hash(&mut hasher);
        hasher.finish()
    }

    // Matches https://github.com/rust-lang/cargo/blob/2928e32734b04925ee51e1ae88bea9a83d2fd451/src/cargo/util/hasher.rs#L6
    fn stable_hash_u64(url: &str, registry_kind: u64) -> u64 {
        use rustc_stable_hash::StableSipHasher128 as StableHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = StableHasher::new();

        // Type has an impact in the `rustc_stable_hasher`.
        (registry_kind as isize).hash(&mut hasher);

        url.hash(&mut hasher);

        Hasher::finish(&hasher)
    }

    fn has_path_past_base(url: &str) -> bool {
        if let Some(protocol_end) = url.find("://") {
            // skip past protocol
            let base_url_end = protocol_end + 3;
            let rest_of_url = &url[base_url_end..];

            // Check if there's any path or meaningful content after the domain (ignoring any trailing slashes)
            return rest_of_url.trim_end_matches('/').contains('/');
        }
        false
    }

    // Matches https://github.com/rust-lang/cargo/blob/2928e32734b04925ee51e1ae88bea9a83d2fd451/src/cargo/util/hex.rs#L6
    fn to_hex(num: u64) -> String {
        hex::encode(num.to_le_bytes())
    }

    let hash_u64 = match hash_kind {
        HashKind::Stable => stable_hash_u64,
        HashKind::Legacy => legacy_hash_u64,
    };

    let mut registry_kind = SOURCE_KIND_REGISTRY;

    // Ensure we have a registry or bare url
    let (mut url, scheme_ind) = {
        let scheme_ind = url
            .find("://")
            .ok_or_else(|| Error::Url(format!("'{url}' is not a valid url")))?;
        let scheme_str = &url[..scheme_ind];
        if scheme_str.starts_with("sparse+http") {
            registry_kind = SOURCE_KIND_SPASE_REGISTRY;
            (url, scheme_ind)
        } else if let Some(ind) = scheme_str.find('+') {
            if &scheme_str[..ind] != "registry" {
                return Err(Error::Url(format!("'{url}' is not a valid registry url")));
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

    // if a custom url ends with a slash it messes up the
    // hash.  But if we remove it from just a base url such as
    // https://index.crates.io/ it messes it up
    // as well. So we strip if it has a path
    // past the base url
    let needs_to_strip = has_path_past_base(url);
    if needs_to_strip {
        if let Some(stripped_url) = url.strip_suffix('/') {
            url = stripped_url;
        }
    }

    // trim port
    let host = host.split(':').next().unwrap();

    let (ident, url) = if registry_kind == SOURCE_KIND_REGISTRY {
        // cargo special cases github.com for reasons, so do the same
        let mut canonical = if host == "github.com" {
            url.to_lowercase()
        } else {
            url.to_owned()
        };

        let ident = match hash_kind {
            HashKind::Stable => {
                // Locate the the first instance of params/fragments.
                let mut params_index = {
                    let question = canonical.find('?');
                    let hash = canonical.rfind('#');

                    question.zip(hash).map(|(q, h)| q.min(h)).or(question).or(hash)
                };

                // Attempt to trim `.git` from the end of url paths.
                canonical = if let Some(idx) = params_index {
                    let base_url = &canonical[..idx];
                    let params = &canonical[idx..];

                    if let Some(sanitized) = base_url.strip_suffix(".git") {
                        params_index = Some(idx - 4);
                        format!("{}{}", sanitized, params)
                    } else {
                        canonical
                    }
                } else {
                    if canonical.ends_with(".git") {
                        canonical.truncate(canonical.len() - 4);
                    }
                    canonical
                };

                let ident = to_hex(hash_u64(&canonical, registry_kind));

                // Strip params
                if let Some(idx) = params_index {
                    canonical.truncate(canonical.len() - (canonical.len() - idx));
                }

                ident
            }
            HashKind::Legacy => {
                // Chop off any query params/fragments
                if let Some(hash) = canonical.rfind('#') {
                    canonical.truncate(hash);
                }

                if let Some(query) = canonical.rfind('?') {
                    canonical.truncate(query);
                }

                if canonical.ends_with('/') {
                    canonical.pop();
                }

                let ident = to_hex(hash_u64(&canonical, registry_kind));

                // Only GitHub (crates.io) repositories have their .git suffix truncated
                if canonical.contains("github.com/") && canonical.ends_with(".git") {
                    canonical.truncate(canonical.len() - 4);
                }

                ident
            }
        };

        (ident, canonical)
    } else {
        (to_hex(hash_u64(url, registry_kind)), url.to_owned())
    };
    Ok((format!("{host}-{ident}"), url))
}

#[cfg(test)]
mod test {
    use crate::dirs::HashKind;

    #[test]
    fn http_index_url_matches_cargo() {
        use crate::sparse::URL;
        assert_eq!(
            super::url_to_local_dir(URL, &HashKind::Legacy).unwrap(),
            ("index.crates.io-6f17d22bba15001f".to_owned(), URL.to_owned(),)
        );
        assert_eq!(
            super::url_to_local_dir(URL, &HashKind::Stable).unwrap(),
            ("index.crates.io-1949cf8c6b5b557f".to_owned(), URL.to_owned(),)
        );

        // I've confirmed this also works with a custom registry, unfortunately
        // that one includes a secret key as part of the url which would allow
        // anyone to publish to the registry, so uhh...here's a fake one instead
        assert_eq!(
            super::url_to_local_dir(
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index.git",
                &HashKind::Legacy
            )
            .unwrap(),
            (
                "dl.cloudsmith.io-ff79e51ddd2b38fd".to_owned(),
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index.git".to_owned()
            )
        );
        assert_eq!(
            super::url_to_local_dir(
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index.git",
                &HashKind::Stable
            )
            .unwrap(),
            (
                "dl.cloudsmith.io-5e6de3fada793d05".to_owned(),
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index".to_owned()
            )
        );
    }

    #[test]
    fn http_index_url_matches_index_slash() {
        assert_eq!(
            super::url_to_local_dir(
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index/",
                &HashKind::Stable
            )
            .unwrap(),
            (
                "dl.cloudsmith.io-5e6de3fada793d05".to_owned(),
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index".to_owned()
            )
        );
    }

    #[test]
    #[cfg(feature = "git")]
    fn git_url_matches_cargo() {
        use crate::git::URL;
        assert_eq!(
            crate::dirs::url_to_local_dir(URL, &HashKind::Legacy).unwrap(),
            ("github.com-1ecc6299db9ec823".to_owned(), URL.to_owned())
        );
        assert_eq!(
            crate::dirs::url_to_local_dir(URL, &HashKind::Stable).unwrap(),
            ("github.com-25cdd57fae9f0462".to_owned(), URL.to_owned())
        );

        // Ensure we actually strip off the irrelevant parts of a url, note that
        // the .git suffix is not part of the canonical url, but *is* used when hashing
        assert_eq!(
            crate::dirs::url_to_local_dir(&format!("registry+{}.git?one=1&two=2#fragment", URL), &HashKind::Legacy)
                .unwrap(),
            ("github.com-c786010fb7ef2e6e".to_owned(), URL.to_owned())
        );
        assert_eq!(
            crate::dirs::url_to_local_dir(&format!("registry+{}.git?one=1&two=2#fragment", URL), &HashKind::Stable)
                .unwrap(),
            ("github.com-e78ed0bbfe5f35d7".to_owned(), URL.to_owned())
        );
    }
}
