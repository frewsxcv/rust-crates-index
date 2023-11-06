use crates_index::SparseIndex;

///
/// **important**:<br>
/// dont forget to enable the **["blocking", "gzip"]** feature of **reqwest**
///
/// command to run:<br>
/// cargo run --example sparse_http_reqwest -F sparse
///

const CRATE_TO_FETCH: &str = "names";

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
    let req = index.make_cache_request(CRATE_TO_FETCH).unwrap().body(()).unwrap();

    let (parts, _) = req.into_parts();
    let req = http::Request::from_parts(parts, vec![]);

    let req: reqwest::blocking::Request = req.try_into().unwrap();

    let client = reqwest::blocking::ClientBuilder::new().gzip(true).build().unwrap();

    let res = client.execute(req).unwrap();

    let mut builder = http::Response::builder().status(res.status()).version(res.version());

    builder
        .headers_mut()
        .unwrap()
        .extend(res.headers().iter().map(|(k, v)| (k.clone(), v.clone())));

    let body = res.bytes().unwrap();
    let res = builder.body(body.to_vec()).unwrap();

    index.parse_cache_response(CRATE_TO_FETCH, res, true).unwrap();
}
