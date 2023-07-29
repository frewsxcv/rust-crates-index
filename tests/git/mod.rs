#[cfg(feature = "https")]
pub(crate)  mod with_https {
    use std::time::SystemTime;
    use crates_index::{GitIndex};
    use crates_index::git::URL;

    #[test]
    fn changes() {
        let index = shared_index();
        let ch = index.changes().unwrap();
        let mut last_time = SystemTime::now();
        let desired = 500;
        let mut count = 0;
        for c in ch.take(desired) {
            let c = c.unwrap();
            count += 1;
            index.crate_(&c.crate_name()).unwrap();
            assert!(last_time >= c.time());
            last_time = c.time();
        }
        assert_eq!(count, desired);
    }

    #[test]
    fn crates() {
        let repo = shared_index();
        assert_eq!("time", repo.crate_("time").unwrap().name());

        let mut found_first_crate = false;
        let mut found_second_crate = false;

        // Note that crates are roughly ordered in reverse.
        for c in repo.crates() {
            if c.name() == "zzzz" {
                found_first_crate = true;
            } else if c.name() == "zulip" {
                found_second_crate = true;
            }
            if found_first_crate && found_second_crate {
                break;
            }
        }
        assert!(found_first_crate);
        assert!(found_second_crate);
    }

    #[test]
    #[serial_test::serial]
    fn with_path_clones_bare_index_automatically() {
        let tmp_dir = tempfile::TempDir::new().unwrap();
        let path = tmp_dir.path().join("some/sub/dir/testing/abc");

        let mut repo =
            GitIndex::with_path(path, URL).expect("Failed to clone crates.io index");

        fn test_sval(repo: &GitIndex) {
            let krate = repo
                .crate_("sval")
                .expect("Could not find the crate sval in the index");

            let version = krate
                .versions()
                .iter()
                .find(|v| v.version() == "0.0.1")
                .expect("Version 0.0.1 of sval does not exist?");
            let dep_with_package_name = version
                .dependencies()
                .iter()
                .find(|d| d.name() == "serde_lib")
                .expect("sval does not have expected dependency?");
            assert_ne!(
                dep_with_package_name.name(),
                dep_with_package_name.package().unwrap()
            );
            assert_eq!(
                dep_with_package_name.crate_name(),
                dep_with_package_name.package().unwrap()
            );
        }

        test_sval(&repo);

        repo.update().expect("Failed to fetch crates.io index");

        test_sval(&repo);
    }

    #[test]
    #[serial_test::serial]
    fn opens_bare_index_and_can_update_it() {
        let mut repo = shared_index();
        fn test_sval(repo: &GitIndex) {
            let krate = repo
                .crate_("sval")
                .expect("Could not find the crate sval in the index");

            let version = krate
                .versions()
                .iter()
                .find(|v| v.version() == "0.0.1")
                .expect("Version 0.0.1 of sval does not exist?");
            let dep_with_package_name = version
                .dependencies()
                .iter()
                .find(|d| d.name() == "serde_lib")
                .expect("sval does not have expected dependency?");
            assert_ne!(
                dep_with_package_name.name(),
                dep_with_package_name.package().unwrap()
            );
            assert_eq!(
                dep_with_package_name.crate_name(),
                dep_with_package_name.package().unwrap()
            );
        }

        test_sval(&repo);

        repo.update().expect("Failed to fetch crates.io index");

        test_sval(&repo);
    }

    #[test]
    fn reads_replaced_source() {
        let index = shared_index();
        let _config = index
            .index_config()
            .expect("we are able to obtain and parse the configuration of the default registry");
    }

    #[test]
    fn crate_dependencies_can_be_read() {
        let index = shared_index();

        let crate_ = index
            .crate_("sval")
            .expect("Could not find the crate libnotify in the index");
        let _ = format!("supports debug {crate_:?}");

        let version = crate_
            .versions()
            .iter()
            .find(|v| v.version() == "0.0.1")
            .expect("Version 0.0.1 of sval does not exist?");
        let dep_with_package_name = version
            .dependencies()
            .iter()
            .find(|d| d.name() == "serde_lib")
            .expect("sval does not have expected dependency?");
        assert_ne!(
            dep_with_package_name.name(),
            dep_with_package_name.package().unwrap()
        );
        assert_eq!(
            dep_with_package_name.crate_name(),
            dep_with_package_name.package().unwrap()
        );
    }

    #[test]
    #[serial_test::serial]
    fn can_update_index_explicitly() {
        let mut index = shared_index();
        index
            .update()
            .map_err(|e| {
                format!(
                    "could not fetch cargo's index in {}: {}",
                    index.path().display(),
                    e
                )
            })
            .unwrap();
        assert!(index.crate_("crates-index").is_some());
        assert!(index.crate_("toml").is_some());
        assert!(index.crate_("gcc").is_some());
        assert!(index.crate_("cc").is_some());
        assert!(index.crate_("CC").is_some());
        assert!(index.crate_("無").is_none());
    }

    pub(crate) fn shared_index() -> GitIndex {
        let index_path = "tests/fixtures/git-registry";
        if is_ci::cached() {
            GitIndex::new_cargo_default()
                .expect("CI has just cloned this index and its ours and valid")
        } else {
            GitIndex::with_path(index_path, URL).expect("clone works and there is no racing")
        }
    }
}
