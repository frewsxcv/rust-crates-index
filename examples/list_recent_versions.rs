//! prints the latest versions of a give crate name
//!
//! it first checks all possible [names](Names) locally <br>
//! if it couldnt not find the crate locally it starts fetching the most likely [names](Names)
//!
//! how to run: <br>
//! `cargo run --example get_versions_cli <crate-name>`

use crates_index::{Crate, Names, SparseIndex};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = match args.get(1) {
        None => {
            println!("missing crate-name argument, please run the example like shown in the doc");
            return;
        }
        Some(name) => name,
    };

    let sparse_index = SparseIndex::new_cargo_default().unwrap();

    match find_locally(name, &sparse_index) {
        Some(krate) => print_crate(krate),
        None => match fetch_crate(name, &sparse_index) {
            None => println!("could not find '{}'", name),
            Some(krate) => print_crate(krate),
        },
    }
}

/// loop though all possible Name permutations and return the crate if found
///
/// more info about [Name permutations](Names)
fn find_locally(name: &str, sparse_index: &SparseIndex) -> Option<Crate> {
    for name in Names::new(name).unwrap() {
        println!("checking for '{}' locally", name);

        if let Ok(krate) = sparse_index.crate_from_cache(&name) {
            return Some(krate);
        }
    }

    None
}

/// fetch the first 3 Name permutations and return the crate if found
///
/// here we only use the first 3 names as these are the most likely to be correct
/// and we skip the rest to minimize the performance hit if a crate does not exist
///
/// more info about [Name permutations](Names)
fn fetch_crate(name: &str, sparse_index: &SparseIndex) -> Option<Crate> {
    for name in Names::new(name).unwrap().take(3) {
        println!("fetching for '{}'", name);

        if let Some(krate) = update(&name, sparse_index) {
            return Some(krate);
        }
    }

    None
}

/// a small helper to update sparse data
fn update(name: &str, index: &SparseIndex) -> Option<Crate> {
    let request: ureq::Request = index.make_cache_request(name).unwrap().into();

    let response: http::Response<String> = match request.call() {
        Ok(response) => response.into(),
        Err(_) => return None,
    };

    let (parts, body) = response.into_parts();
    let response = http::Response::from_parts(parts, body.into_bytes());

    index.parse_cache_response(name, response, true).unwrap()
}

/// a small helper to print the found crate in a formatted way
fn print_crate(krate: Crate) {
    println!();
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
    }
}
