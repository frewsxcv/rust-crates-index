#[test]
fn crate_from_cache() {
    let index = crates_index::SparseIndex::with_path(
        std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("tests/fixtures/sparse_registry_cache/cargo_home"),
        crates_index::sparse::URL
    ).unwrap();

    let crate_ = index.crate_from_cache("autocfg").unwrap();

    assert_eq!(crate_.name(), "autocfg");
    assert_eq!(crate_.versions().len(), 13);
    assert_eq!(crate_.earliest_version().version(), "0.0.1");
    assert_eq!(crate_.highest_version().version(), "1.1.0");
}

#[cfg(all(test, feature = "sparse-http"))]
mod with_sparse_http_feature {
    use crates_index::SparseIndex;

    #[inline]
    fn crates_io() -> SparseIndex {
        SparseIndex::with_path(
            std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("tests/fixtures/sparse_registry_cache/cargo_home"),
            crates_index::sparse::URL
        ).unwrap()
    }

    mod make_cache_request {
        use http::{header, Request};
        use crate::sparse_index::with_sparse_http_feature::crates_io;

        // Validates that a valid request is generated when there is no cache entry
        // for a crate
        #[test]
        fn generates_request_for_missing_cache_entry() {
            let index = crates_io();
            let builder = index.make_cache_request("serde").unwrap();
            let req: Request<Vec<u8>> = builder.body(vec![]).unwrap();

            assert_eq!(req.uri(), format!("{}se/rd/serde", index.url()).as_str());
            assert!(req.headers().get(header::IF_NONE_MATCH).is_none());
            assert!(req.headers().get(header::IF_MODIFIED_SINCE).is_none());
            assert_eq!(req.headers().get(header::ACCEPT_ENCODING).unwrap(), "gzip,identity");
            assert_eq!(req.headers().get(header::HeaderName::from_static("cargo-protocol")).unwrap(), "version=1");
            assert_eq!(req.headers().get(header::ACCEPT).unwrap(), "text/plain");
        }

        // Validates that a valid request is generated when there is a local cache
        // entry for a crate
        #[test]
        fn generates_request_for_local_cache_entry() {
            let index = crates_io();
            let builder = index.make_cache_request("autocfg").unwrap();
            let req: Request<Vec<u8>> = builder.body(vec![]).unwrap();

            assert_eq!(req.uri(), format!("{}au/to/autocfg", index.url()).as_str());
            assert_eq!(req.headers().get(header::IF_NONE_MATCH).unwrap(), "W/\"aa975a09419f9c8f61762a3d06fdb67d\"");
            assert!(req.headers().get(header::IF_MODIFIED_SINCE).is_none());
        }
    }
    
    mod parse_cache_response {
        use http::header;
        use crate::sparse_index::with_sparse_http_feature::crates_io;

        // curl -v -H 'accept-encoding: gzip,identity' -H 'if-none-match: W/"aa975a09419f9c8f61762a3d06fdb67d"' https://index.crates.io/au/to/autocfg
        // as of 2023-06-15
        const AUTOCFG_INDEX_ENTRY: &[u8] = include_bytes!("../../tests/fixtures/autocfg.txt");

        // Validates that a response with the full index contents are properly parsed
        #[test]
        fn parses_modified_response() {
            let index = crates_io();
            let response = http::Response::builder()
                .status(http::StatusCode::OK)
                .header(header::ETAG, "W/\"5f15de4a723e10b3f9eaf048d693cccc\"")
                .body(AUTOCFG_INDEX_ENTRY.to_vec()).unwrap();

            let krate = index.parse_cache_response("autocfg", response, false).unwrap().unwrap();
            assert_eq!(krate.highest_version().version(), "1.1.0");
        }

        // Validates that a response for an index entry that has not been modified is
        // parsed correctly
        #[test]
        fn parses_unmodified_response() {
            let index = crates_io();
            let response = http::Response::builder()
                .status(http::StatusCode::NOT_MODIFIED)
                .header(header::ETAG, "W/\"5f15de4a723e10b3f9eaf048d693cccc\"")
                .body(Vec::new()).unwrap();

            let krate = index.parse_cache_response("autocfg", response, false).unwrap().unwrap();
            assert_eq!(krate.name(), "autocfg");
            assert_eq!(krate.versions().len(), 13);
            assert_eq!(krate.earliest_version().version(), "0.0.1");
            assert_eq!(krate.highest_version().version(), "1.1.0");
        }

        // Validates that a response for an index entry that does not exist is
        // parsed correcty
        #[test]
        fn parses_missing_response() {
            let index = crates_io();
            let response = http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(Vec::new()).unwrap();

            assert!(index.parse_cache_response("serde", response, false).unwrap().is_none());
        }
    }
}
