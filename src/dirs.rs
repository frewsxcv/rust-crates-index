use crate::Error;

/// Get the disk location of the specified url, as well as its canonical form,
/// exactly as cargo would
///
/// `cargo_home` is used to root the directory at specific location, if not
/// specified `CARGO_HOME` or else the default cargo location is used as the root
pub(crate) fn get_index_details(
    url: &str,
    cargo_home: Option<&std::path::Path>,
) -> Result<(std::path::PathBuf, String), Error> {
    let (dir_name, canonical_url) = url_to_local_dir(url)?;

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
            accumulator.extend(crate_name.as_bytes().get(0..1)?.iter().map(|c| c.to_ascii_lowercase() as char));
        }
        _ => {
            accumulator.extend(crate_name.as_bytes().get(0..2)?.iter().map(|c| c.to_ascii_lowercase() as char));
            accumulator.push(separator);
            accumulator.extend(crate_name.as_bytes().get(2..4)?.iter().map(|c| c.to_ascii_lowercase() as char));
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


/// Converts a full url, eg https://github.com/rust-lang/crates.io-index, into
/// the root directory name where cargo itself will fetch it on disk
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
    fn hash_u64(url: &str, registry_kind: u64) -> u64 {
        use std::hash::{Hash, Hasher, SipHasher};

        let mut hasher = SipHasher::new_with_keys(0, 0);
        // Registry
        registry_kind.hash(&mut hasher);
        // Url
        url.hash(&mut hasher);
        hasher.finish()
    }

    // SourceKind::Registry
    let mut registry_kind = 2;

    // Ensure we have a registry or bare url
    let (url, scheme_ind) = {
        let scheme_ind = url
            .find("://")
            .ok_or_else(|| Error::Url(format!("'{url}' is not a valid url")))?;

        let scheme_str = &url[..scheme_ind];
        if scheme_str.starts_with("sparse+http") {
            registry_kind = 3;
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

    // trim port
    let host = host.split(':').next().unwrap();

    let (ident, url) = if registry_kind == 2 {
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

        let ident = to_hex(hash_u64(&canonical, registry_kind));

        if canonical.ends_with('/') {
            canonical.pop();
        }

        if canonical.contains("github.com/") && canonical.ends_with(".git") {
            // Only GitHub (crates.io) repositories have their .git suffix truncated
            canonical.truncate(canonical.len() - 4);
        }

        (ident, canonical)
    } else {
        (to_hex(hash_u64(url, registry_kind)), url.to_owned())
    };

    Ok((format!("{host}-{ident}"), url))
}

#[cfg(test)]
mod test {

    #[test]
    fn http_index_url_matches_cargo() {
        use crate::sparse::URL;
        assert_eq!(
            super::url_to_local_dir(URL).unwrap(),
            (
                "index.crates.io-6f17d22bba15001f".to_owned(),
                URL.to_owned(),
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
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index.git".to_owned()
            )
        );
    }
    
    #[test]
    #[cfg(feature = "git")]
    fn git_url_matches_cargo() {
        use crate::git::URL;
        assert_eq!(
            crate::dirs::url_to_local_dir(URL).unwrap(),
            (
                "github.com-1ecc6299db9ec823".to_owned(),
                URL.to_owned()
            )
        );

        // Ensure we actually strip off the irrelevant parts of a url, note that
        // the .git suffix is not part of the canonical url, but *is* used when hashing
        assert_eq!(
            crate::dirs::url_to_local_dir(&format!(
                "registry+{}.git?one=1&two=2#fragment",
                URL
            ))
                .unwrap(),
            (
                "github.com-c786010fb7ef2e6e".to_owned(),
                URL.to_owned()
            )
        );
    }
}
