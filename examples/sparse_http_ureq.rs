use crates_index::SparseIndex;
use std::io;

///
/// **important**:<br>
/// dont forget to enable the **http-interop** feature of **ureq**
///
/// command to run:<br>
/// cargo run --example sparse_http_ureq -F sparse-http
///

const CRATE_TO_FETCH: &str = "inferno";

fn main() {
    let mut index = SparseIndex::new_cargo_default().unwrap();

    print_crate(&mut index);
    update(&mut index);
    print_crate(&mut index);
}

fn print_crate(index: &mut SparseIndex) {
    match index.crate_from_cache(CRATE_TO_FETCH) {
        Ok(krate) => {
            println!("{:?}", krate.highest_normal_version().unwrap().version());
        }
        Err(_err) => {
            println!("could not find crate {}", CRATE_TO_FETCH)
        }
    }
}

fn update(index: &mut SparseIndex) {
    let request: ureq::Request = index.make_cache_request(CRATE_TO_FETCH).unwrap().into();

    let response: http::Response<String> = request
        .call()
        .map_err(|_e| io::Error::new(io::ErrorKind::InvalidInput, "connection error"))
        .unwrap()
        .into();

    let (parts, body) = response.into_parts();
    let response = http::Response::from_parts(parts, body.into_bytes());

    index.parse_cache_response(CRATE_TO_FETCH, response, true).unwrap();
}
