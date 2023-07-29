mod bare_index;
mod sparse_index;
mod error {
    #[test]
    fn error_is_send() {
        fn is_send<T: Send>() {}
        is_send::<crates_index::Error>();
    }
}

