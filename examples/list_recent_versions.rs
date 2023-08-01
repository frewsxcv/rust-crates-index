//! Print the 5 most recent versions of a give crate name.
//!
//! It first checks all possible [names](Names) using the local cache and on failure
//! updates the cache by fetching the most likely [names](Names) from the sparse registry.

use crates_index::{Crate, Names, SparseIndex};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sparse_index = SparseIndex::new_cargo_default()?;
    let mut count = 0;
    for name in std::env::args().skip(1) {
        count += 1;
        let krate = match find_locally(&name, &sparse_index)? {
            Some(krate) => krate,
            None => fetch_crate(&name, &sparse_index)?.ok_or_else(|| format!("could not find '{name}'"))?,
        };

        print_crate(krate);
    }

    if count == 0 {
        Err("Please provide one or more crate names to lookup".into())
    } else {
        Ok(())
    }
}

/// Loop though all possible permutations of `name` and return the crate if found.
/// This is feasible as local lookups are fast.
///
/// Read more about [name permutations](Names).
fn find_locally(name: &str, sparse_index: &SparseIndex) -> Result<Option<Crate>, Box<dyn Error>> {
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

        if let Some(krate) = update(&name, sparse_index)? {
            return Ok(Some(krate));
        }
    }
    Ok(None)
}

fn names(name: &str) -> Result<impl Iterator<Item = String>, Box<dyn Error>> {
    Ok(Names::new(name)
        .ok_or_else(|| "Too many hyphens in crate name")?
        .take(3))
}

/// Create a request to the sparse `index` and parse the response with the side-effect of yielding
/// the desired crate and updating the local cache.
fn update(name: &str, index: &SparseIndex) -> Result<Option<Crate>, Box<dyn Error>> {
    let request: ureq::Request = index.make_cache_request(name)?.into();

    let response: http::Response<String> = match request.call() {
        Ok(response) => response.into(),
        Err(_) => return Ok(None),
    };

    let (parts, body) = response.into_parts();
    let response = http::Response::from_parts(parts, body.into_bytes());

    Ok(index.parse_cache_response(name, response, true)?)
}

fn print_crate(krate: Crate) {
    println!("{}", krate.name());

    let versions = krate
        .versions()
        .iter()
        .map(|version| version.version())
        .rev()
        .take(5)
        .collect::<Vec<&str>>();

    print!("versions: {}", versions.join(", "));

    if krate.versions().len() > 5 {
        println!(", ...")
    } else {
        println!()
    }
}
