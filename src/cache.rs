use std::io::{Error, ErrorKind, Result};

pub(crate) const CURRENT_CACHE_VERSION: u8 = 3;
pub(crate) const CURRENT_INDEX_FORMAT_VERSION: u32 = 2;

impl crate::Crate {
    /// Parse crate index entry from a .cache file, this can fail for a number of reasons
    ///
    /// 1. There is no entry for this crate
    /// 2. The entry was created with an older version than the one specified
    /// 3. The entry is a newer version than what can be read, would only
    /// happen if a future version of cargo changed the format of the cache entries
    /// 4. The cache entry is malformed somehow
    #[inline(never)]
    pub(crate) fn from_cache_slice(bytes: &[u8], index_version: Option<&str>) -> Result<Self> {
        // See src/cargo/sources/registry/index.rs
        let (first_byte, mut rest) = bytes.split_first().ok_or(ErrorKind::UnexpectedEof)?;

        match *first_byte {
            // This is the current 1.54.0 - 1.70.0+ version of cache entries
            CURRENT_CACHE_VERSION => {
                let index_v_bytes = rest.get(..4).ok_or(ErrorKind::UnexpectedEof)?;
                let index_v = u32::from_le_bytes(index_v_bytes.try_into().unwrap());
                if index_v != CURRENT_INDEX_FORMAT_VERSION {
                    return Err(Error::new(ErrorKind::Unsupported,
                        format!("wrong index format version: {index_v} (expected {CURRENT_INDEX_FORMAT_VERSION}))")));
                }
                rest = &rest[4..];
            }
            // This is only to support ancient <1.52.0 versions of cargo https://github.com/rust-lang/cargo/pull/9161
            1 => {}
            // Note that the change from 2 -> 3 was only to invalidate cache
            // entries https://github.com/rust-lang/cargo/pull/9476 and
            // version 2 entries should only be emitted by cargo 1.52.0 and 1.53.0,
            // but rather than _potentially_ parse bad cache entries as noted in
            // the PR we explicitly tell the user their version of cargo is suspect
            // these versions are so old (and specific) it shouldn't affect really anyone
            2 => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "potentially invalid version 2 cache entry found",
                ));
            }
            version => {
                return Err(Error::new(
                    ErrorKind::Unsupported,
                    format!("cache version '{version}' not currently supported"),
                ));
            }
        }

        let mut iter = split(rest, 0);
        let update = iter.next().ok_or(ErrorKind::UnexpectedEof)?;
        if let Some(index_version) = index_version {
            if update != index_version.as_bytes() {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "cache out of date: current index ({index_version}) != cache ({})",
                        String::from_utf8_lossy(update)
                    ),
                ));
            }
        }

        Self::from_version_entries_iter(iter)
    }

    pub(crate) fn from_version_entries_iter<'a, I: Iterator<Item = &'a [u8]> + 'a>(
        mut iter: I,
    ) -> Result<Self> {
        let mut versions = Vec::new();

        // Each entry is a tuple of (semver, version_json)
        while let Some(_version) = iter.next() {
            let version_slice = iter.next().ok_or(ErrorKind::UnexpectedEof)?;
            let version: crate::Version = serde_json::from_slice(version_slice)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            versions.push(version);
        }

        Ok(Self {
            versions: versions.into_boxed_slice(),
        })
    }

    #[cfg(test)]
    pub(crate) fn write_to_file(&self, version: &str, path: &std::path::Path) {
        let mut v = Vec::new();
        v.push(CURRENT_CACHE_VERSION);
        v.extend_from_slice(&CURRENT_INDEX_FORMAT_VERSION.to_le_bytes());
        v.extend_from_slice(version.as_bytes());
        v.push(0);

        for version in self.versions() {
            v.extend_from_slice(version.version().as_bytes());
            v.push(0);
            v.append(&mut serde_json::to_vec(version).unwrap());
            v.push(0);
        }

        std::fs::write(path, v).unwrap();
    }
}

pub(crate) fn split(haystack: &[u8], needle: u8) -> impl Iterator<Item = &[u8]> + '_ {
    struct Split<'a> {
        haystack: &'a [u8],
        needle: u8,
    }

    impl<'a> Iterator for Split<'a> {
        type Item = &'a [u8];

        #[inline]
        fn next(&mut self) -> Option<&'a [u8]> {
            if self.haystack.is_empty() {
                return None;
            }
            let (ret, remaining) = match memchr::memchr(self.needle, self.haystack) {
                Some(pos) => (&self.haystack[..pos], &self.haystack[pos + 1..]),
                None => (self.haystack, &[][..]),
            };
            self.haystack = remaining;
            Some(ret)
        }
    }

    Split { haystack, needle }
}
