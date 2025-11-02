//! Print the 5 most recent versions of a give crate name.
//!
//! It first checks all possible [names](Names) using the local cache and on failure
//! updates the cache by fetching the most likely [names](Names) from the sparse registry.

use crates_index::{Crate, Names, SparseIndex};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sparse_index = SparseIndex::new_cargo_default()?;
    let mut count = 0;
    let mut missing = Vec::new();
    for name in std::env::args().skip(1) {
        count += 1;
        let krate = match find_in_cache(&name, &sparse_index)? {
            Some(krate) => krate,
            None => match fetch_crate(&name, &sparse_index)? {
                Some(krate) => krate,
                None => {
                    eprintln!("{name} not found");
                    missing.push(name);
                    continue;
                }
            },
        };

        print_crate(krate);
    }

    if count == 0 {
        Err("Please provide one or more crate names to lookup".into())
    } else if !missing.is_empty() {
        Err(format!("The following crates could not be found: {}", missing.join(", ")).into())
    } else {
        Ok(())
    }
}

/// Loop though all possible permutations of `name` and return the crate if found.
/// This is feasible as local lookups are fast.
///
/// Read more about [name permutations](Names).
fn find_in_cache(name: &str, sparse_index: &SparseIndex) -> Result<Option<Crate>, Box<dyn Error>> {
    for name in names(name)? {
        eprintln!("checking for '{}' locally", name);

        if let Ok(krate) = sparse_index.crate_from_cache(&name) {
            return Ok(Some(krate));
        }
    }
    Ok(None)
}

/// Fetch the first 3 permutations of `name` from the sparse registry and return the crate if found.
///
/// Here we only use the first 3 names which are most likely to be correct
/// and  skip the rest to minimize the performance hit if a crate does not exist.
///
/// Read more about [name permutations](Names).
fn fetch_crate(name: &str, sparse_index: &SparseIndex) -> Result<Option<Crate>, Box<dyn Error>> {
    for name in names(name)? {
        eprintln!("fetching for '{}'", name);

        if let Some(krate) = update_cache(&name, sparse_index)? {
            return Ok(Some(krate));
        }
    }
    Ok(None)
}

fn names(name: &str) -> Result<impl Iterator<Item = String>, Box<dyn Error>> {
    Ok(Names::new(name).ok_or("Too many hyphens in crate name")?.take(3))
}

/// Create a request to the sparse `index` and parse the response with the side-effect of yielding
/// the desired crate and updating the local cache.
fn update_cache(name: &str, index: &SparseIndex) -> Result<Option<Crate>, Box<dyn Error>> {
    let request = index
        .make_cache_request(name)?
        .version(ureq::http::Version::HTTP_11)
        .body(())?;

    let response = ureq::run(request)?;

    let (parts, mut body) = response.into_parts();
    let response = http::Response::from_parts(parts, body.read_to_vec()?);
    Ok(index.parse_cache_response(name, response, true)?)
}

fn print_crate(krate: Crate) {
    const MAX_VERSIONS: usize = 5;
    println!("{}", krate.name());

    let versions = krate
        .versions()
        .iter()
        .rev()
        .take(5)
        .map(|version| version.version())
        .collect::<Vec<_>>();

    print!("versions: {}", versions.join(", "));
    if krate.versions().len() > MAX_VERSIONS {
        println!(", [{} more skipped]", krate.versions().len() - MAX_VERSIONS)
    } else {
        println!()
    }
}
