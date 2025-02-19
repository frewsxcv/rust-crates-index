# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 3.6.0 (2025-02-19)

### New Features

 - <csr-id-fd3aecd230e8a1a0b1428bd23d0a6eea46085848/> Add support for new index cargo hash implementation.
    This also adds `dirs::local_path_and_canonical_url_with_hash_kind()`
   and `SparseIndex::with_path_and_hash_kind()` to control
   which hash is used.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 58 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #184 from UebelAndre/stable_hash ([`1f3b4b0`](https://github.com/frewsxcv/rust-crates-index/commit/1f3b4b03ab3ed4d5aaca589bb3988ac4c1fff212))
    - Thanks clippy ([`bdd7919`](https://github.com/frewsxcv/rust-crates-index/commit/bdd79190deb2b461c61ba4d5ec49757392d95422))
    - Add support for new index cargo hash implementation. ([`fd3aecd`](https://github.com/frewsxcv/rust-crates-index/commit/fd3aecd230e8a1a0b1428bd23d0a6eea46085848))
    - Merge pull request #185 from UebelAndre/macos ([`973f2e5`](https://github.com/frewsxcv/rust-crates-index/commit/973f2e5eb12fdeae2b427000139f1050f49f3c08))
    - Fix test which could also fail CI ([`55ff57a`](https://github.com/frewsxcv/rust-crates-index/commit/55ff57a54beae94d57b2b0d17c876dd2d6956b80))
    - Add MacOS to CI matrix ([`ec3c643`](https://github.com/frewsxcv/rust-crates-index/commit/ec3c6432b7a6d11be547949b15eb0d5f1481abde))
</details>

## 3.5.0 (2024-12-22)

### New Features

 - <csr-id-daf488dd8efcec325cda6e48fba644ba2a770d51/> upgrade `gix` to v0.69

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 11 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v3.5.0 ([`0b0ceb8`](https://github.com/frewsxcv/rust-crates-index/commit/0b0ceb88203570325baf0a3febc148feac96721c))
    - Merge pull request #180 from paolobarbolini/gix-0.69 ([`07207bd`](https://github.com/frewsxcv/rust-crates-index/commit/07207bd37c867e08871cb833074fbbeddcf091b5))
    - Upgrade `gix` to v0.69 ([`daf488d`](https://github.com/frewsxcv/rust-crates-index/commit/daf488dd8efcec325cda6e48fba644ba2a770d51))
</details>

## 3.4.0 (2024-12-11)

### New Features

 - <csr-id-4d5a4e44ab0bfbde74eb269f4c21156d5d854cd3/> upgrade `gix` to v0.68

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 27 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v3.4.0 ([`4e2a1d4`](https://github.com/frewsxcv/rust-crates-index/commit/4e2a1d4ff78d78b9175dd1e39a0a6a3f9645bec1))
    - Merge pull request #179 from Eh2406/set_commit ([`42e2d22`](https://github.com/frewsxcv/rust-crates-index/commit/42e2d22e0fd8c909d70b742ad1f866b7a11aa5e9))
    - Refactor ([`b4e6379`](https://github.com/frewsxcv/rust-crates-index/commit/b4e63793b894d0e67fdcf1b947ecb953cac68ddc))
    - Upgrade `gix` to v0.68 ([`4d5a4e4`](https://github.com/frewsxcv/rust-crates-index/commit/4d5a4e44ab0bfbde74eb269f4c21156d5d854cd3))
    - Api to set the head commit ([`383e4f7`](https://github.com/frewsxcv/rust-crates-index/commit/383e4f7c6d7a20bce6cda7d1e88ceb46a994d1a4))
    - Remove head_commit_hex ([`b801850`](https://github.com/frewsxcv/rust-crates-index/commit/b801850c7dfe3182df0adcc7f643944d23085369))
</details>

## 3.3.0 (2024-11-14)

A release to update dependencies, namely `gix` is now at version 0.67.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v3.3.0 ([`adbc764`](https://github.com/frewsxcv/rust-crates-index/commit/adbc7649549376a1f6fabc4343856a009fe9a904))
    - Bump version and update changelog ([`1054148`](https://github.com/frewsxcv/rust-crates-index/commit/105414897d70187ea06eb38431c8223d2f22a227))
    - Merge branch 'bump-dependencies' ([`9fdf214`](https://github.com/frewsxcv/rust-crates-index/commit/9fdf214b6235c3efebd960a536451a6361f80974))
    - Update all dependencies ([`0c47c11`](https://github.com/frewsxcv/rust-crates-index/commit/0c47c11d67815ee4641fa0a2312a434046fc62ac))
</details>

## 3.2.0 (2024-08-23)

### New Features

 - <csr-id-15a503e74d1a4a6cbb115849824f6c814510c093/> upgrade `gix` to v0.66

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 23 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v3.2.0 ([`7b6c0e3`](https://github.com/frewsxcv/rust-crates-index/commit/7b6c0e349fe3578607323a259272ea6b0c0e2ddc))
    - Upgrade `gix` to v0.66 ([`15a503e`](https://github.com/frewsxcv/rust-crates-index/commit/15a503e74d1a4a6cbb115849824f6c814510c093))
</details>

## 3.1.0 (2024-07-30)

<csr-id-704225198d5ada22863f4f20eac0c193b1c0c4a3/>

### New Features

 - <csr-id-73b7bf2d6de441f62962bf7744cc30ce2d15e50e/> update `gix` to v0.64

### Other

 - <csr-id-704225198d5ada22863f4f20eac0c193b1c0c4a3/> expose dependency registry url
   This commit exposes the field `registry` on the `Dependency` struct. This field is always set by `cargo`, and it's needed to properly handle dependencies when multiple registries are used.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 63 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v3.1.0 ([`10289d7`](https://github.com/frewsxcv/rust-crates-index/commit/10289d7b9922a3711ac19c5afe6dca9e81c363e1))
    - Merge branch 'update-gix' ([`8cc981f`](https://github.com/frewsxcv/rust-crates-index/commit/8cc981fe6bacf4d3ddab69db3fdd97bccb407681))
    - Fix registry field ([`7a79f51`](https://github.com/frewsxcv/rust-crates-index/commit/7a79f51666ad0c54c4894818ff967d828a2f9d56))
    - Update all dependencies ([`39b6ff8`](https://github.com/frewsxcv/rust-crates-index/commit/39b6ff86f48b0960ff70aaa547d59e130d159ee1))
    - Update `gix` to v0.64 ([`73b7bf2`](https://github.com/frewsxcv/rust-crates-index/commit/73b7bf2d6de441f62962bf7744cc30ce2d15e50e))
    - Merge pull request #174 from demurgos/feature/dependency-registry ([`8b0a542`](https://github.com/frewsxcv/rust-crates-index/commit/8b0a5426f26ba7fc0fd0a8ff2027b61044628863))
    - Expose dependency registry url ([`7042251`](https://github.com/frewsxcv/rust-crates-index/commit/704225198d5ada22863f4f20eac0c193b1c0c4a3))
</details>

## 3.0.0 (2024-05-28)

### New Features (BREAKING)

 - <csr-id-8d85c9a8d08f90c92ae3f5190e20a72dccf21f18/> upgrade `http` to v1.0
   It's breaking as `http` types are in our public API with the `sparse` feature enabled.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v3.0.0 ([`9bfd7eb`](https://github.com/frewsxcv/rust-crates-index/commit/9bfd7eb33675a3f113e686185fc6aa4080552695))
    - Merge branch 'fix-breakage' ([`ffc0920`](https://github.com/frewsxcv/rust-crates-index/commit/ffc09204ec3547d9ef4f49c5efcc53cb83d69d95))
    - Upgrade `http` to v1.0 ([`8d85c9a`](https://github.com/frewsxcv/rust-crates-index/commit/8d85c9a8d08f90c92ae3f5190e20a72dccf21f18))
</details>

## 2.10.1 (2024-05-28)

This release fixes v2.10 which broke the `sparse` feature due to the upgrade to `http` 1.0, which was present in the public API.
This release uses `http` 0.2 again, whereas the upcoming v3.0 will be for `http` 1.0.

### Bug Fixes

 - <csr-id-2cbb0808738c9c5a6ab361fd00b03fbd0009a2f7/> export `http` crate to make more obvious we are using it in the public API

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v2.10.1 ([`e2ca88c`](https://github.com/frewsxcv/rust-crates-index/commit/e2ca88c81287227e9cbc580eb0cdee9099ce94d7))
    - Update changelog ([`3920ef3`](https://github.com/frewsxcv/rust-crates-index/commit/3920ef3a6c81af074e45d84d4bdb1b05e8243e28))
    - Export `http` crate to make more obvious we are using it in the public API ([`2cbb080`](https://github.com/frewsxcv/rust-crates-index/commit/2cbb0808738c9c5a6ab361fd00b03fbd0009a2f7))
    - Revert "Upgrade to http 1" - this was a breaking change ([`451026b`](https://github.com/frewsxcv/rust-crates-index/commit/451026bb817c862c7ea753588b9b97abbd97af74))
</details>

## 2.10.0 (2024-05-24)

### New Features

 - <csr-id-6223b70962579000aba8e01be925bbcf6f7fa95d/> upgrade `gix` to v0.63 for security fixes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 38 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v2.10.0 ([`dba8b14`](https://github.com/frewsxcv/rust-crates-index/commit/dba8b144d222b7da41ed57df613d93575905b2b5))
    - Bump version to next free one after messing up v2.9 ([`217bb86`](https://github.com/frewsxcv/rust-crates-index/commit/217bb868efe69c10f4ace62a46f8267dc38657ad))
    - Upgrade `gix` to v0.63 for security fixes ([`6223b70`](https://github.com/frewsxcv/rust-crates-index/commit/6223b70962579000aba8e01be925bbcf6f7fa95d))
    - Upgrade `gix` to version 0.63.0 ([`2dc7734`](https://github.com/frewsxcv/rust-crates-index/commit/2dc7734511fab8f6043615fc8fe5af3e49029118))
    - Merge pull request #170 from fenhl/http1 ([`eae6947`](https://github.com/frewsxcv/rust-crates-index/commit/eae69477dcb44f27c2a4f80e1d4a1107fe5b3087))
    - Upgrade to http 1 ([`a925945`](https://github.com/frewsxcv/rust-crates-index/commit/a925945966d1da60bed757c1b0788082ef7b0417))
    - Merge pull request #169 from nickspurry/master ([`4be1703`](https://github.com/frewsxcv/rust-crates-index/commit/4be1703d13817d76896f98084ee1a06a2533125a))
    - Correct Example code in README ([`4e23a38`](https://github.com/frewsxcv/rust-crates-index/commit/4e23a38068715337ab4908cf33dd4476687cdcc7))
</details>

## 2.8.0 (2024-04-15)

<csr-id-07c23f363d7a0494b52e0741794d6dfa96fd65a1/>

### Chore

 - <csr-id-07c23f363d7a0494b52e0741794d6dfa96fd65a1/> upgrade gix to v0.62

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 7 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v2.8.0 ([`b6314ac`](https://github.com/frewsxcv/rust-crates-index/commit/b6314acbd12ccb5effbf20b4e3d91b2c088db6b5))
    - Merge branch 'gix-upgrade' ([`a9d4660`](https://github.com/frewsxcv/rust-crates-index/commit/a9d4660e7411d4e8341d0825b82dc483ddb6aefb))
    - Bump minor version to indicate 'gix' upgrade. ([`731658f`](https://github.com/frewsxcv/rust-crates-index/commit/731658f97d2fd5f08cb65616f4edbdb096811f47))
    - Upgrade gix to v0.62 ([`07c23f3`](https://github.com/frewsxcv/rust-crates-index/commit/07c23f363d7a0494b52e0741794d6dfa96fd65a1))
</details>

## 2.7.0 (2024-04-08)

### New Features

 - <csr-id-3667cd02d53716aa4bc01fdbdf526f7070efbe47/> add `SparseIndex::make_config_request()` and `SparseIndex::parse_config_request()`.
   That way it's possible to handle the case where no sparse index exists yet.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#164](https://github.com/frewsxcv/rust-crates-index/issues/164)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#164](https://github.com/frewsxcv/rust-crates-index/issues/164)**
    - Add `SparseIndex::make_config_request()` and `SparseIndex::parse_config_request()`. ([`3667cd0`](https://github.com/frewsxcv/rust-crates-index/commit/3667cd02d53716aa4bc01fdbdf526f7070efbe47))
 * **Uncategorized**
    - Release crates-index v2.7.0 ([`2895ec1`](https://github.com/frewsxcv/rust-crates-index/commit/2895ec122a94ab097ff6ce5918308e7daa8d6f4f))
    - Refactor ([`49e8ba7`](https://github.com/frewsxcv/rust-crates-index/commit/49e8ba7ad882f6f6ed8252bb97b5320c997448d1))
    - Add support for downloading the sparse index config ([`2b8460c`](https://github.com/frewsxcv/rust-crates-index/commit/2b8460cd76e9836d98db231546654d0c38c0da3d))
    - Revert "chore: update to gix v0.59" - this version was yanked ([`289617b`](https://github.com/frewsxcv/rust-crates-index/commit/289617b1f9ed5f30448bb5b5e57ab3af1e8d10d8))
</details>

## 2.6.0 (2024-02-25)

<csr-id-110d4a8096b781e1cde8a35c08cb8c68d7a69612/>

### Chore

 - <csr-id-110d4a8096b781e1cde8a35c08cb8c68d7a69612/> update to gix v0.59

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 28 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v2.6.0 ([`fab4d1d`](https://github.com/frewsxcv/rust-crates-index/commit/fab4d1dff8fefe3f944e0182f6da94b23bd451b7))
    - Set next release version ([`bec65e9`](https://github.com/frewsxcv/rust-crates-index/commit/bec65e9de8948defb266fa0859fb5b9b920d4d73))
    - Update to gix v0.59 ([`110d4a8`](https://github.com/frewsxcv/rust-crates-index/commit/110d4a8096b781e1cde8a35c08cb8c68d7a69612))
</details>

## 2.5.1 (2024-01-28)

### Bug Fixes

 - <csr-id-4d75232a72cb5f87b6dafa042898b7249123b88a/> assure Git index updates to refs are actually written.
   The remote git repository may alter its references in such a way that
   local fast-forwards aren't possible anymore.
   
   This happens regularly as the history will be squashed on the remote.
   
   Now we forcefully store the updated references, which resolves
   the issue that calling `update()` didn't seem to do anything despite
   being busy (i.e. downloading a possibly huge pack, and resolving it).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v2.5.1 ([`3e1b769`](https://github.com/frewsxcv/rust-crates-index/commit/3e1b769353d2e6bbdfd5288637aa6a111b056939))
    - Merge branch 'fix-index-update' ([`99e8fe1`](https://github.com/frewsxcv/rust-crates-index/commit/99e8fe172f98e096607789f349c050663f416b1d))
    - Assure Git index updates to refs are actually written. ([`4d75232`](https://github.com/frewsxcv/rust-crates-index/commit/4d75232a72cb5f87b6dafa042898b7249123b88a))
</details>

## 2.5.0 (2024-01-21)

<csr-id-5960658acef322349d9c8e9ac291365321b6add0/>

### Chore

 - <csr-id-5960658acef322349d9c8e9ac291365321b6add0/> update `gix` to v0.58

### New Features

 - <csr-id-2efa665a28bcdb54d4d4faf6f75c6f4e80153a93/> upgrade to `gix-0.58`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 22 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v2.5.0 ([`234c7b1`](https://github.com/frewsxcv/rust-crates-index/commit/234c7b16295068e74fae8873be2a8f612cd83f05))
    - Upgrade to `gix-0.58` ([`2efa665`](https://github.com/frewsxcv/rust-crates-index/commit/2efa665a28bcdb54d4d4faf6f75c6f4e80153a93))
    - Update `gix` to v0.58 ([`5960658`](https://github.com/frewsxcv/rust-crates-index/commit/5960658acef322349d9c8e9ac291365321b6add0))
</details>

## 2.4.0 (2023-12-30)

<csr-id-81f70d7cbcac5d4dbef6477cd803b1d103099347/>
<csr-id-4ffad17947228bfeff47200d1ca969ae637c35cb/>
<csr-id-39d9fb66270d38d4a3933e0f52edd3e0afcad143/>

### Chore

 - <csr-id-81f70d7cbcac5d4dbef6477cd803b1d103099347/> update `gix` to v0.57

### Other

 - <csr-id-4ffad17947228bfeff47200d1ca969ae637c35cb/> Update readme to include the sparse protocol
 - <csr-id-39d9fb66270d38d4a3933e0f52edd3e0afcad143/> Link to examples of how to update the sparse index

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 53 calendar days.
 - 53 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v2.4.0 ([`b84f4b9`](https://github.com/frewsxcv/rust-crates-index/commit/b84f4b9371be12d7ae3d34422efd25dbb0181341))
    - Bump minor version for gix dependency change ([`3ae82f5`](https://github.com/frewsxcv/rust-crates-index/commit/3ae82f506aa7b67366cd59336e4245503c40edc9))
    - Merge branch 'maintenance' ([`be03b1f`](https://github.com/frewsxcv/rust-crates-index/commit/be03b1f1245555a81f9005b3ebf28069ba47f236))
    - Adapt to quite drastic changes in memory requirements. ([`c237796`](https://github.com/frewsxcv/rust-crates-index/commit/c237796a3ffd80c0c06a354f2a9cc3693e8d5e0c))
    - Update `gix` to v0.57 ([`81f70d7`](https://github.com/frewsxcv/rust-crates-index/commit/81f70d7cbcac5d4dbef6477cd803b1d103099347))
    - Fix cargo-check ([`2920beb`](https://github.com/frewsxcv/rust-crates-index/commit/2920beb86ede9cdca09c8f516ec5981e05912575))
    - Add new example to list the most recent version of a crate using the git index ([`61f3090`](https://github.com/frewsxcv/rust-crates-index/commit/61f3090271f5992ca621c2a13df5daaa91e0cca6))
    - Update readme to include the sparse protocol ([`4ffad17`](https://github.com/frewsxcv/rust-crates-index/commit/4ffad17947228bfeff47200d1ca969ae637c35cb))
    - Link to examples of how to update the sparse index ([`39d9fb6`](https://github.com/frewsxcv/rust-crates-index/commit/39d9fb66270d38d4a3933e0f52edd3e0afcad143))
    - Update ureq-sparse-example to use ureq 2.8.0 ([`239b009`](https://github.com/frewsxcv/rust-crates-index/commit/239b009fc8fdb42250a09f52ef351e84c29ab7e3))
</details>

## 2.3.0 (2023-11-06)

<csr-id-82002e7d8362b5e0736f994a1569d002333c7fad/>

### Chore

 - <csr-id-82002e7d8362b5e0736f994a1569d002333c7fad/> upgrade `gitoxide` to v0.55.2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v2.3.0 ([`639e425`](https://github.com/frewsxcv/rust-crates-index/commit/639e425b2b640a9f289ad394c1310cce3474102e))
    - Merge branch 'updates' ([`345d8ce`](https://github.com/frewsxcv/rust-crates-index/commit/345d8ce9461da90fb70cb894be30ecd5669fa6fd))
    - Bump feature level ([`c9a7744`](https://github.com/frewsxcv/rust-crates-index/commit/c9a7744c94a2e41f0ea2291cb5f184800a0c922e))
    - Upgrade `gitoxide` to v0.55.2 ([`82002e7`](https://github.com/frewsxcv/rust-crates-index/commit/82002e7d8362b5e0736f994a1569d002333c7fad))
</details>

## 2.2.0 (2023-09-25)

### New Features

 - <csr-id-9aec9abcd6b78bba27e8ea6000f6e4592bcc9397/> upgrade `gix` to v0.54

### Bug Fixes

 - <csr-id-61b93a1e9b21ac3a8c146570e6fdebdaaffdd698/> be less strict when determining if the crates-index remote matches the target URL.
   Previously a trailing slash could have caused it to think ti's not the same.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 36 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index v2.2.0 ([`87700cf`](https://github.com/frewsxcv/rust-crates-index/commit/87700cf530e37b5cbb1d7892953bdd6608ad6b4f))
    - Merge branch 'gix-upgrade' ([`fe1ac0c`](https://github.com/frewsxcv/rust-crates-index/commit/fe1ac0cab22b3a0a9529a15a5549ac8bbf93ad10))
    - Be less strict when determining if the crates-index remote matches the target URL. ([`61b93a1`](https://github.com/frewsxcv/rust-crates-index/commit/61b93a1e9b21ac3a8c146570e6fdebdaaffdd698))
    - Upgrade `toml` to latest version ([`fc79c67`](https://github.com/frewsxcv/rust-crates-index/commit/fc79c6711946c83f8a7fd6e892dd446de3345622))
    - Adjust test expectations ([`1f73994`](https://github.com/frewsxcv/rust-crates-index/commit/1f739940b00be746dcd06edc8b00b586817845eb))
    - Upgrade `gix` to v0.54 ([`9aec9ab`](https://github.com/frewsxcv/rust-crates-index/commit/9aec9abcd6b78bba27e8ea6000f6e4592bcc9397))
</details>

## 2.1.1 (2023-08-20)

<csr-id-72796a91835d05099fc20f9f5895288d9d3ff715/>

### Chore

 - <csr-id-72796a91835d05099fc20f9f5895288d9d3ff715/> upgrade gix to v0.51 from v0.50

### Bug Fixes

 - <csr-id-2d4bbdaf7147b556b36b60b7facf042b079bd19c/> Allow using git::URL without git feature active

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#149](https://github.com/frewsxcv/rust-crates-index/issues/149)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#149](https://github.com/frewsxcv/rust-crates-index/issues/149)**
    - Allow using git::URL without git feature active ([`2d4bbda`](https://github.com/frewsxcv/rust-crates-index/commit/2d4bbdaf7147b556b36b60b7facf042b079bd19c))
 * **Uncategorized**
    - Release crates-index v2.1.1 ([`7d6df0d`](https://github.com/frewsxcv/rust-crates-index/commit/7d6df0dd27eafcd2d9e6375d394010fd3756b093))
    - Upgrade gix to v0.51 from v0.50 ([`72796a9`](https://github.com/frewsxcv/rust-crates-index/commit/72796a91835d05099fc20f9f5895288d9d3ff715))
    - Fix build ([`57763f1`](https://github.com/frewsxcv/rust-crates-index/commit/57763f16a77be91e7744263bdbfeaa3e4fe73db9))
    - Allow using `git::URL` without `git` feature active ([`f5d398a`](https://github.com/frewsxcv/rust-crates-index/commit/f5d398a0eb60b6a459dcb2c4e0f50b1e51a779a9))
    - Add `tame-index` link to README ([`88ac53e`](https://github.com/frewsxcv/rust-crates-index/commit/88ac53e139a0990ba0e55139d645422b1a071207))
    - Test negative case of `GitIndex::try_*` as well as possible ([`238526f`](https://github.com/frewsxcv/rust-crates-index/commit/238526f61b1fd91d930be3441a9203beee9aed76))
</details>

## 2.1.0 (2023-08-02)

<csr-id-421de3512465f135af8d63ed276ceba9e882f8f3/>

### New Features

 - <csr-id-639b0818ce4919118af71c8bc2bfb19d791a215d/> add `GitIndex::try_new*()` and `GitIndex::try_with_path()` to open without cloning.
   These methods are naturally read-only and thus have no issues in concurrent contexts, while
   not providing an option to not auto-clone a whole index.
 - <csr-id-abe5d70bc3c1a78f3f762104f7cb29b5907471ad/> Add `Names` iterator as building block for fuzzy-lookups.
   It creates all allowed permutations regarding `-` and `_` in the crate name,
   so it should be possible to find a crate even if the name doesn't have the correct
   hyphens or underscores set.

### Bug Fixes

 - <csr-id-28ab782c1ece77af4885e58104fc28b2c8687b0e/> `GitIndex::new_*()` will not discover the git repository anymore.
   Previously, discovery was used which may traverse the directory structure
   upwards to find the index. This may be error prone as the index location is
   supposed to be well-known.
   
   Now the index path provided must either be `.../index` or `.../index.git` to be
   opened successfully.
 - <csr-id-c67033d2c1653eef69e791de0266c15cb7f6321e/> remove the usage of file locks in preference for documentation when opening a git index.
   Previously, to allow concurrently opening and possibly updating a crates-index, a file-lock was
   used for synchronization. However, it was rather specific to what the test-suite needed while
   adding another failure mode for production code which could leave lock-files behind that then
   lock the crates-index forever for this library at least.
   
   Instead, appropriate locking will be used in tests only, while the documentation of all
   `open` methods of `GitIndex` was adjusted to inform about ways to protect concurrent accesses
   on application level.
 - <csr-id-3bb46abf5a07db3c4b98dabc7efe6ddebc2174ff/> always use `/` for sparse URLs
   Previously on windows, backslashes could have snuck in which may cause problems.

### Other

 - <csr-id-421de3512465f135af8d63ed276ceba9e882f8f3/> add new example to print information using the sparse index: `list_recent_versions`.
   Run it with `cargo run --example list_recent_versions -- foo bar baz gix rustc gcc foobar blaz`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 37 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#62](https://github.com/frewsxcv/rust-crates-index/issues/62)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#62](https://github.com/frewsxcv/rust-crates-index/issues/62)**
    - Improve docs to better clarify the locking behaviour and implications ([`4919cb2`](https://github.com/frewsxcv/rust-crates-index/commit/4919cb2f00bc6cf0124d1f6cf0a35d158e249033))
 * **Uncategorized**
    - Release crates-index v2.1.0 ([`a9b6065`](https://github.com/frewsxcv/rust-crates-index/commit/a9b60653efb72d9e6be98c4f8fe56194475cbd3f))
    - Merge branch 'locking' ([`d8fc1c1`](https://github.com/frewsxcv/rust-crates-index/commit/d8fc1c1c7a9d59f042ceaf301b2b9c4c08a4eded))
    - Add `GitIndex::try_new*()` and `GitIndex::try_with_path()` to open without cloning. ([`639b081`](https://github.com/frewsxcv/rust-crates-index/commit/639b0818ce4919118af71c8bc2bfb19d791a215d))
    - `GitIndex::new_*()` will not discover the git repository anymore. ([`28ab782`](https://github.com/frewsxcv/rust-crates-index/commit/28ab782c1ece77af4885e58104fc28b2c8687b0e))
    - Remove the usage of file locks in preference for documentation when opening a git index. ([`c67033d`](https://github.com/frewsxcv/rust-crates-index/commit/c67033d2c1653eef69e791de0266c15cb7f6321e))
    - Minor refactor to make names in example more descriptive ([`6f8aa18`](https://github.com/frewsxcv/rust-crates-index/commit/6f8aa183228bee2047d9c14d503b847839bd0764))
    - Add new example to print information using the sparse index: `list_recent_versions`. ([`421de35`](https://github.com/frewsxcv/rust-crates-index/commit/421de3512465f135af8d63ed276ceba9e882f8f3))
    - Slighlty more informative printing in new example and improve usability ([`dc6537a`](https://github.com/frewsxcv/rust-crates-index/commit/dc6537a03b1b2b2213331a27b483e9ca2935c72d))
    - Refactor ([`fdf663e`](https://github.com/frewsxcv/rust-crates-index/commit/fdf663e9914752393bd456bf98cf6e9f6486dc4b))
    - Rename new example to fit what it does even better ([`5f73acd`](https://github.com/frewsxcv/rust-crates-index/commit/5f73acd3a4f8f8dd1cbc67bd49e70a04312dc37c))
    - Add example to Cargo.toml ([`6125624`](https://github.com/frewsxcv/rust-crates-index/commit/6125624f312f1bf1bfcd3a535a1761eee253b4e9))
    - Add a small example to show the full sparse workflow ([`2ef9dac`](https://github.com/frewsxcv/rust-crates-index/commit/2ef9dac7523454af48a059e10f72ca6e48907a4e))
    - Example "update_and_get_latest" requires git-https ([`f755b0f`](https://github.com/frewsxcv/rust-crates-index/commit/f755b0f20174768771c9ed219c85e0b8ec3b25bd))
    - Merge branch 'names-optimizations' ([`7b8683e`](https://github.com/frewsxcv/rust-crates-index/commit/7b8683e0a3861f51a751c9f8ee496e17a344e05e))
    - Improve docs for `Names` ([`6ab652e`](https://github.com/frewsxcv/rust-crates-index/commit/6ab652e52c10ae29bbcc69ae822fb2f03e54550d))
    - Minor refactor ([`a20138d`](https://github.com/frewsxcv/rust-crates-index/commit/a20138dbfc94fe2a9fa28b0560cd47d53dec8899))
    - First return all-hyphens & all_underscores ([`6b66356`](https://github.com/frewsxcv/rust-crates-index/commit/6b66356d9c1c463495c5f841a67e5c62152ebc97))
    - Update test to capture edge case ([`b63ec37`](https://github.com/frewsxcv/rust-crates-index/commit/b63ec378b7b6e248225848b0632b41882311e6f9))
    - Use max_count for count() ([`254d21e`](https://github.com/frewsxcv/rust-crates-index/commit/254d21e0a10c49cc2dc16b25569743f8d209ef80))
    - Fix typo in Names doc ([`c8aa392`](https://github.com/frewsxcv/rust-crates-index/commit/c8aa39223a65cd4e0ed087b46b7c1cd70b898a0b))
    - Improve documentation of `Names` iterator ([`5272d41`](https://github.com/frewsxcv/rust-crates-index/commit/5272d4159b1a7ff66d50a7e2554f9b6c321298f9))
    - Add `Names` iterator as building block for fuzzy-lookups. ([`abe5d70`](https://github.com/frewsxcv/rust-crates-index/commit/abe5d70bc3c1a78f3f762104f7cb29b5907471ad))
    - Return the input name first ([`797081e`](https://github.com/frewsxcv/rust-crates-index/commit/797081e1914432cab7af1be18513dc67c7f8d682))
    - Double the performance/throughput by absusing our knowledge about UTF-8 ([`61ddff0`](https://github.com/frewsxcv/rust-crates-index/commit/61ddff02a2caca4f4f6bd1cde3537833ba161e56))
    - Prefer hyphens over underscores as these are more common ([`1f542a5`](https://github.com/frewsxcv/rust-crates-index/commit/1f542a5c95b60769b08055e4b737781d11972862))
    - Avoid allocation of vector for separator indices ([`a7801b0`](https://github.com/frewsxcv/rust-crates-index/commit/a7801b0576c5be3fe77befa71559c2518a0ab9c7))
    - Allow the `Names` iterator to fail creation if too many permutations are possible ([`9b88659`](https://github.com/frewsxcv/rust-crates-index/commit/9b88659a14f5eb4dd13b3990d7db73756fb63bc6))
    - Refactor tests ([`95308c8`](https://github.com/frewsxcv/rust-crates-index/commit/95308c8fee8c71f82dd0af90d40f5247edc9e2fc))
    - Refactor structure ([`bc16839`](https://github.com/frewsxcv/rust-crates-index/commit/bc168393a17c806bf697ab0c668c76c8f462afe5))
    - Add PoC of NamePermutationIterator ([`1429c4e`](https://github.com/frewsxcv/rust-crates-index/commit/1429c4e2305f854ce645d18258ba0379b0e53f86))
    - Only test on stable Rust as this covers most use-cases ([`a0bba1c`](https://github.com/frewsxcv/rust-crates-index/commit/a0bba1c5603ee369158120e15b1411b08168a4de))
    - Change links from lib.rs to crates.io ([`5f1f245`](https://github.com/frewsxcv/rust-crates-index/commit/5f1f245169cebbbc8adb2882af7d094441bdcd9e))
    - Always use `/` for sparse URLs ([`3bb46ab`](https://github.com/frewsxcv/rust-crates-index/commit/3bb46abf5a07db3c4b98dabc7efe6ddebc2174ff))
    - Add a cache as well to speed up builds, hopefully, particularly on windows ([`e322e13`](https://github.com/frewsxcv/rust-crates-index/commit/e322e1340b9bf2e30891a6f8ffad09f35fc5f94f))
    - See if CI can handle windows tests as well ([`2b6e070`](https://github.com/frewsxcv/rust-crates-index/commit/2b6e070594665a15d5bed38cac7e76253167a1b5))
    - Always use / as a separator for sparse urls ([`1d8a895`](https://github.com/frewsxcv/rust-crates-index/commit/1d8a895fa21745437cb444bf9aa64dafd347fd7c))
</details>

## 2.0.0 (2023-07-29)

<csr-id-c293e35e43650bebbdbd869c4c9d01bfb2e836c0/>
<csr-id-2c5d33a51604f032ff1538b16cf0408a8fe2568a/>
<csr-id-7e86e3c625944cdeba55dda6086617796fb061e3/>
<csr-id-a8953e0939711940f2ef554155edcf3853030df3/>
<csr-id-260c103409ff08a96c465568363675d6dc8a2fa7/>
<csr-id-235e175022647f9ab63b024ca0780c907b9fd6ec/>
<csr-id-beb9f12885703574ba3c3307c368fb84c1a05028/>
<csr-id-42d89c2e84f0e81da3db046864be379a2ae9eb15/>
<csr-id-965f6e98788a62c380ed1daa61867817685d7371/>

This is a major release with many breaking changes to make the overall package structure, type-names and feature names more consistent.

**Note that the `default` features changed**, so if you relied on that you will have to change the dependency definition in your `cargo` manifest
to something like `default-features = false, features = ["git-performance", "git-https"]`. This is due to the sparse index now being the default,
just like with `cargo` itself.

Further, now `crate_index::Index` is `crates_index::GitIndex`, but when done all should work as before, maybe even a little bit faster thanks to
replacing `git2` with [`gix`](https://docs.rs/gix/0.50.1/gix/).

For details about all breaking changes, please take a look at the `(BREAKING)` paragraphs that follow.

### Other

 - <csr-id-965f6e98788a62c380ed1daa61867817685d7371/> make clear that `GitIndex` auto-clones any index as needed.

### New Features

 - <csr-id-0d893523aa682b10c50441be3ec1d8f5356bf2c0/> add `dirs::TBD` to make it possible to know where the index should be looked for.
   This might be interesting also for tools that deal with the data alone, like `cargo-cache`.

### Chore

 - <csr-id-c293e35e43650bebbdbd869c4c9d01bfb2e836c0/> Add `CHANGELOG.md` for a built-in version of it
 - <csr-id-2c5d33a51604f032ff1538b16cf0408a8fe2568a/> replace `git2` with `gix`.
   This change bringe performance improvements along with increased compatibilty
   with other build targets, as pure Rust builds are now possible.

### Refactor (BREAKING)

 - <csr-id-7e86e3c625944cdeba55dda6086617796fb061e3/> refactor code structure
   The goal is to keep related code together, instead of spreading it out into
   top-level modules exclusively.
   
   This also renames `Index` to `GitIndex`.
   Further changes involved renaming `ChangesIter` to `git::Changes`, and
   `INDEX_GIT_URL` to `git::URL`, and `CRATES_IO_HTTP_INDEX` to `sparse::URL`.

### Chore (BREAKING)

 - <csr-id-a8953e0939711940f2ef554155edcf3853030df3/> remove `ssh` feature, and rename many existing features, change defaults
   * `git-index` -> `git`
   * add `git-performance`
   * `https` -> `git-https`
   * `sparse-http` -> `sparse`
   
   The default features are now `sparse`, effectively adjusting to the fact
   that the default is now the http registry.
 - <csr-id-260c103409ff08a96c465568363675d6dc8a2fa7/> remove `changes` feature
   It only gated a little bit of code, but no dependencies. Thus it had no considerable
   effect on build times and can be removed.

### Other

 - <csr-id-235e175022647f9ab63b024ca0780c907b9fd6ec/> make clear that `GitIndex` auto-clones any index as needed.
 - <csr-id-beb9f12885703574ba3c3307c368fb84c1a05028/> crate features are now documented

### Chore

 - <csr-id-42d89c2e84f0e81da3db046864be379a2ae9eb15/> Add `CHANGELOG.md` for a built-in version of it

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 39 commits contributed to the release over the course of 7 calendar days.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#129](https://github.com/frewsxcv/rust-crates-index/issues/129)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#129](https://github.com/frewsxcv/rust-crates-index/issues/129)**
    - Replace `git2` with `gix`. ([`2c5d33a`](https://github.com/frewsxcv/rust-crates-index/commit/2c5d33a51604f032ff1538b16cf0408a8fe2568a))
 * **Uncategorized**
    - Release crates-index v2.0.0 ([`6b95b8f`](https://github.com/frewsxcv/rust-crates-index/commit/6b95b8f6e11f660b15bc0309d68e635792f1c4b2))
    - Fix `include` directive to allow publish to succeed ([`40caa8f`](https://github.com/frewsxcv/rust-crates-index/commit/40caa8f16b1e33603660a614918d2497bc9427c3))
    - Merge branch 'v2.0' ([`d3b0069`](https://github.com/frewsxcv/rust-crates-index/commit/d3b006976060cda7c09c407ee5707a73bea50b2b))
    - Enable cargo-fmt ([`365f9dc`](https://github.com/frewsxcv/rust-crates-index/commit/365f9dcf36a86ffda5c0280b5494bca3becfd3ed))
    - Add `dirs::TBD` to make it possible to know where the index should be looked for. ([`0d89352`](https://github.com/frewsxcv/rust-crates-index/commit/0d893523aa682b10c50441be3ec1d8f5356bf2c0))
    - Make clear that `GitIndex` auto-clones any index as needed. ([`965f6e9`](https://github.com/frewsxcv/rust-crates-index/commit/965f6e98788a62c380ed1daa61867817685d7371))
    - Remove `ssh` feature, and rename many existing features, change defaults ([`a8953e0`](https://github.com/frewsxcv/rust-crates-index/commit/a8953e0939711940f2ef554155edcf3853030df3))
    - Crate features are now documented ([`beb9f12`](https://github.com/frewsxcv/rust-crates-index/commit/beb9f12885703574ba3c3307c368fb84c1a05028))
    - Bump version to 2.0, update CHANGELOG.md with excerpt from README.md ([`b0836d1`](https://github.com/frewsxcv/rust-crates-index/commit/b0836d1974ea35fec6c6f40e72a62dd9ca0d65bc))
    - Run `cargo-diet` to optimize package size ([`2fdf3a8`](https://github.com/frewsxcv/rust-crates-index/commit/2fdf3a8eaef6bc2114f82050d5306d896b0fe76d))
    - Refactor code structure ([`7e86e3c`](https://github.com/frewsxcv/rust-crates-index/commit/7e86e3c625944cdeba55dda6086617796fb061e3))
    - Remove `changes` feature ([`260c103`](https://github.com/frewsxcv/rust-crates-index/commit/260c103409ff08a96c465568363675d6dc8a2fa7))
    - Rename `testdata` to `fixtures` ([`aba9606`](https://github.com/frewsxcv/rust-crates-index/commit/aba9606a02c84be8229a642c6b8d06914a1c5fc6))
    - Move tests of the public API into `tests/` where integration tests live ([`0096b92`](https://github.com/frewsxcv/rust-crates-index/commit/0096b924e437a6c310b18a72b2aa5e68605afdcc))
    - Add `CHANGELOG.md` for a built-in version of it ([`42d89c2`](https://github.com/frewsxcv/rust-crates-index/commit/42d89c2e84f0e81da3db046864be379a2ae9eb15))
    - Use `thiserror` for the error type. ([`fed6904`](https://github.com/frewsxcv/rust-crates-index/commit/fed6904d2bc92ed8ac83d39134b1a97fbd1c980d))
    - Improve "find_repo_head()" to be more resilient ([`5649466`](https://github.com/frewsxcv/rust-crates-index/commit/564946679e8867a364badffefa8470185442ee33))
    - Fix refspecs for updating the crates index ([`cc6b8f9`](https://github.com/frewsxcv/rust-crates-index/commit/cc6b8f9692eeb0e7a0660fd7a6c02459d1f05d0d))
    - Add an example that gets the latest changed crate right after updating the index. ([`7d70f8f`](https://github.com/frewsxcv/rust-crates-index/commit/7d70f8fdbf9bef73d9a11bbedc09be34e5bdc697))
    - Use the latest `gix` release for API improvements ([`f50308f`](https://github.com/frewsxcv/rust-crates-index/commit/f50308f2763941c337b58d4eb3c843a0b7f98b6d))
    - Run `cargo fmt` on everything that changed in `changes.rs` ([`084f226`](https://github.com/frewsxcv/rust-crates-index/commit/084f2266cc4b354a793a487e59208bf5a32cc0ae))
    - Re-use test utilities for a unified experience ([`90da01e`](https://github.com/frewsxcv/rust-crates-index/commit/90da01eba71e6470332b71cf040abfd9778d75d5))
    - Remove all remainders of `git2` ([`d649f95`](https://github.com/frewsxcv/rust-crates-index/commit/d649f95dec2b75435e0087964958cc9d3043ff2d))
    - Convert `Changes` from `git2` to `gix` ([`beddca6`](https://github.com/frewsxcv/rust-crates-index/commit/beddca636328edfb413e325112c4e546667c874d))
    - Additional protection against raciness when cloning in parallel ([`11a7522`](https://github.com/frewsxcv/rust-crates-index/commit/11a7522ce1b2855dce68766884b80414bd0d847f))
    - Fix alrogithm for finding the head-reference ([`50edb46`](https://github.com/frewsxcv/rust-crates-index/commit/50edb46259dbd817f519c5148608fad4ccfd17cd))
    - Cargo-fmt on all portions that changed ([`ab0f126`](https://github.com/frewsxcv/rust-crates-index/commit/ab0f1269bbb625500f754a29c9015de01859bf4f))
    - Switch to latest `gix` version to smoothen API usage ([`940ed59`](https://github.com/frewsxcv/rust-crates-index/commit/940ed5940ea2c94de8cd87ece1cdff5b96cbc0dd))
    - Thanks clippy ([`8939074`](https://github.com/frewsxcv/rust-crates-index/commit/8939074d70cbd68f8fd4209ecfc22b4646887bb6))
    - Use `gix` for `update()` ([`ebfda0f`](https://github.com/frewsxcv/rust-crates-index/commit/ebfda0f7b959b1a4a0a6fc0fb32a3a07461c0601))
    - Clone the crate index with `gix` ([`92291cf`](https://github.com/frewsxcv/rust-crates-index/commit/92291cff41b1782a5beb5f49e5b036974b1da0d6))
    - Use `gix` for implementing `crate_()` ([`2cf53dd`](https://github.com/frewsxcv/rust-crates-index/commit/2cf53dd7fd58e746cf1fbe52f3fcc0cad4089a74))
    - Use `gix` for `index_config()` ([`cd7f910`](https://github.com/frewsxcv/rust-crates-index/commit/cd7f9108c0ecdfb9c2f877f1c489f54a2a822040))
    - Convert `crates()` to `gix` ([`fbf169c`](https://github.com/frewsxcv/rust-crates-index/commit/fbf169c65690fb67482b3024adf06e5a02e51071))
    - Use `gix` for `crates_parallel()` ([`141285e`](https://github.com/frewsxcv/rust-crates-index/commit/141285ee544449d0f804c03d2995bf88d2b297d1))
    - Add more local git tests to run quickly (i.e. those that don't clone) ([`9cac8a8`](https://github.com/frewsxcv/rust-crates-index/commit/9cac8a87a1a22c37d3fccf3f66ce46729cb4e9e7))
    - Add `gix` as alternative ([`3fe885c`](https://github.com/frewsxcv/rust-crates-index/commit/3fe885c3e73409026fba5394b7201d5388b1a914))
    - Fix doc-tests for 'changes' feature and run that on CI as well ([`f234c9b`](https://github.com/frewsxcv/rust-crates-index/commit/f234c9b48e980e74058b311b495d0a0bac2fd9dd))
</details>

## v1.0.0 (2023-07-21)

### Migration Notes

* `git2` is now optional if you use `default-features = false`. If `git-index` feature is enabled, `git2` v0.17 is required. You'll want to enable `https` feature too.
* `SparseIndex.make_cache_request` returns `request::Builder` instead of `Request`. Call `.body(())` on it.


### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release.
 - 28 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Ensure disabled features work ([`34c76db`](https://github.com/frewsxcv/rust-crates-index/commit/34c76dbf32d1f446bdbaa6247dd9cf543a711105))
    - Bump ([`18eb3f4`](https://github.com/frewsxcv/rust-crates-index/commit/18eb3f4f281eedb0c458e5d52379b04ea689c223))
    - Iterator for recent index changes ([`3b39fab`](https://github.com/frewsxcv/rust-crates-index/commit/3b39fabb995dbd905344c4b50f26fc0865b1ed1e))
    - Require features for examples ([`05e48cc`](https://github.com/frewsxcv/rust-crates-index/commit/05e48cc90107c6fb2a019d9c88c6b8fedd5a52e4))
    - Merge pull request #131 from ToBinio/add_sparse_http_example ([`c44bea2`](https://github.com/frewsxcv/rust-crates-index/commit/c44bea238d54f19fc99a963a7c6b4106a8959167))
    - Add examples ([`8042c98`](https://github.com/frewsxcv/rust-crates-index/commit/8042c98bcb788f5dfe6165f645c5d231830af1dd))
    - Fix doc tests ([`0faf3c8`](https://github.com/frewsxcv/rust-crates-index/commit/0faf3c849cc2a3a90dfcb668f14c5ef7cee8d83a))
    - Make git2 dependency optional via new "git-index" feature ([`cf65d09`](https://github.com/frewsxcv/rust-crates-index/commit/cf65d090563692aa44777f47a3b3f45973cc890d))
    - Change return type to request::Builder ([`1f5bd6c`](https://github.com/frewsxcv/rust-crates-index/commit/1f5bd6cd7390500aa75243db1fba0dfef3a95f6f))
    - Fix typos ([`e0157d0`](https://github.com/frewsxcv/rust-crates-index/commit/e0157d00a2fd4cc7652be81f9529b553314c8463))
    - Build data ([`c725849`](https://github.com/frewsxcv/rust-crates-index/commit/c725849cdd51552524af15b320365690d0e149b5))
</details>

## v0.19.13 (2023-06-22)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 7 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix build with sparse-http ([`ff8bebe`](https://github.com/frewsxcv/rust-crates-index/commit/ff8bebe314cd79b5aa1e5b43221a6bbb1c3560c4))
    - Test with sparse-http ([`594edfd`](https://github.com/frewsxcv/rust-crates-index/commit/594edfd66127b71001713ed4f763b4cf9c5669ff))
</details>

## v0.19.12 (2023-06-15)

<csr-id-de7df1cb85b322e0e9cde387a01f426685d8a4a4/>
<csr-id-a69a7785347cfc7b5773c73e0b10b8e5b63a1e58/>
<csr-id-135179a0e95a97e3e57d16692c7b17c54ffe0d16/>

### Other

 - <csr-id-de7df1cb85b322e0e9cde387a01f426685d8a4a4/> Add `cargo check --all-targets --no-default-features`

### Refactor

 - <csr-id-a69a7785347cfc7b5773c73e0b10b8e5b63a1e58/> Move Index tests to base_index
   To make it easier to turn git2 into an optional dep.
 - <csr-id-135179a0e95a97e3e57d16692c7b17c54ffe0d16/> Move private fn fetch_opts() into mod bare_index
   To make it easier to turn git2 into an optional dep.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`f0f45b7`](https://github.com/frewsxcv/rust-crates-index/commit/f0f45b76ce9a5410486adf57326bf761350a0e04))
    - Shrink public API ([`9ba7cac`](https://github.com/frewsxcv/rust-crates-index/commit/9ba7cac94ee6563ef2819d6112bf6eef81366f66))
    - Add `cargo check --all-targets --no-default-features` ([`de7df1c`](https://github.com/frewsxcv/rust-crates-index/commit/de7df1cb85b322e0e9cde387a01f426685d8a4a4))
    - Make `cargo check --all-targets --no-default-features` build ([`8b1c6d8`](https://github.com/frewsxcv/rust-crates-index/commit/8b1c6d8e200add2d54c612298d6991d9c5b589e5))
    - Add back separator arg ([`cdc8fde`](https://github.com/frewsxcv/rust-crates-index/commit/cdc8fdea45fae88b12405f5fd195cbd59780210c))
    - Fix lint ([`276aab2`](https://github.com/frewsxcv/rust-crates-index/commit/276aab29c3eda1de4cea11656e7433073351558c))
    - Fixup rebase ([`07476e5`](https://github.com/frewsxcv/rust-crates-index/commit/07476e5fbbced67390221a81367645283dc7f1f8))
    - Add HTTP support to SparseIndex ([`b636afc`](https://github.com/frewsxcv/rust-crates-index/commit/b636afc5fecde87fbaee1a0d2e31fb2a44fc5a1b))
    - Cleanup index initialization ([`20eca12`](https://github.com/frewsxcv/rust-crates-index/commit/20eca12e8e2f661d1bfc76141f6867a57f656080))
    - Move Index tests to base_index ([`a69a778`](https://github.com/frewsxcv/rust-crates-index/commit/a69a7785347cfc7b5773c73e0b10b8e5b63a1e58))
    - Move private fn fetch_opts() into mod bare_index ([`135179a`](https://github.com/frewsxcv/rust-crates-index/commit/135179a0e95a97e3e57d16692c7b17c54ffe0d16))
    - Reduce generic function ([`f09ff32`](https://github.com/frewsxcv/rust-crates-index/commit/f09ff323a23e5f96d87a6daa4d487f577a4ed1e9))
    - Fix config parsing ([`82c2849`](https://github.com/frewsxcv/rust-crates-index/commit/82c28497cb96654046203992b796c807c2ddf0a7))
    - Cleanup index initialization ([`71a76b0`](https://github.com/frewsxcv/rust-crates-index/commit/71a76b0b3179e25a265d8eb7040194ff47412d84))
    - Merge pull request #114 from EmbarkStudios/fix-cache-deserializatoin ([`c5f6123`](https://github.com/frewsxcv/rust-crates-index/commit/c5f6123922d3836ded54cdd4bbf7d2f4ac337f4e))
    - Merge pull request #118 from EmbarkStudios/fix-lint ([`0ffb5bf`](https://github.com/frewsxcv/rust-crates-index/commit/0ffb5bf3f88285226dddcac6f6d9dd284b89e528))
    - Merge pull request #115 from EmbarkStudios/add-http-url ([`a58beb5`](https://github.com/frewsxcv/rust-crates-index/commit/a58beb5844e241604371c850fe358ac7dcb19b8a))
    - Merge pull request #113 from EmbarkStudios/disable-formatting ([`8d20801`](https://github.com/frewsxcv/rust-crates-index/commit/8d20801c6c7dd56f20aae1e766cba4cd3706deff))
    - Fix lint ([`2b51d50`](https://github.com/frewsxcv/rust-crates-index/commit/2b51d508384f3d8459a3954fdeb0157649af02d7))
    - Add CRATES_IO_HTTP_INDEX ([`a7690ef`](https://github.com/frewsxcv/rust-crates-index/commit/a7690ef9d5870f65636ebe316c47fd89959c680c))
    - Fix cache deserialization ([`f645b4c`](https://github.com/frewsxcv/rust-crates-index/commit/f645b4c8e098d01335846acbf77a442019e7f874))
    - Disable rustfmt ([`d1f51bf`](https://github.com/frewsxcv/rust-crates-index/commit/d1f51bf9ece463581b1ab54898f480ac58670ad7))
</details>

## v0.19.11 (2023-06-15)

<csr-id-8cf14fbe0317ba4fc443eb6926f4a952bf8e7e0e/>

### Chore

 - <csr-id-8cf14fbe0317ba4fc443eb6926f4a952bf8e7e0e/> Release crates-index version 0.19.11

### New Features

 - <csr-id-810fa8726d1b628550bb80d1f7aa716a411f64ed/> Allow for fetching indexes from private registries via SSH key

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 32 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index version 0.19.11 ([`8cf14fb`](https://github.com/frewsxcv/rust-crates-index/commit/8cf14fbe0317ba4fc443eb6926f4a952bf8e7e0e))
    - Merge pull request #106 from rohanku/ssh-key-from-agent ([`96f8444`](https://github.com/frewsxcv/rust-crates-index/commit/96f844467bad30f3e4964b33fde7d603bdb67f56))
    - Merge pull request #109 from EmbarkStudios/fix-32-bit-hash ([`1fe05d4`](https://github.com/frewsxcv/rust-crates-index/commit/1fe05d42ece1ae4ed11e5a437b477081a0c298c2))
    - Merge pull request #108 from Enselic/threshold ([`1b1a9bc`](https://github.com/frewsxcv/rust-crates-index/commit/1b1a9bcbfedf8aa206a9388f02689a4dcc99d241))
    - Fix hash calculation on 32-bit targets ([`8066188`](https://github.com/frewsxcv/rust-crates-index/commit/806618825691703a75ff6eae5c16a1d2c1b92a18))
    - Increase threshold in mem_usage() test ([`a6f0d85`](https://github.com/frewsxcv/rust-crates-index/commit/a6f0d85ad7228b4587d6084d335016c95dce7005))
    - Allow for fetching indexes from private registries via SSH key ([`810fa87`](https://github.com/frewsxcv/rust-crates-index/commit/810fa8726d1b628550bb80d1f7aa716a411f64ed))
    - Merge pull request #105 from Enselic/remove-num_cpus ([`5db0873`](https://github.com/frewsxcv/rust-crates-index/commit/5db087328bc11340c6f6c715ad978d7fc5c9ec69))
    - Remove unused num_cpus dependency ([`05c0bcb`](https://github.com/frewsxcv/rust-crates-index/commit/05c0bcb1097ae13504247d035abf357111eaec0e))
</details>

## v0.19.10 (2023-05-13)

<csr-id-286b2251ae8a286f8992831f7a845f88227107dd/>

### Chore

 - <csr-id-286b2251ae8a286f8992831f7a845f88227107dd/> Release crates-index version 0.19.10

### New Features

 - <csr-id-ab8c655d7835a93c87b348c86ecc928ecfaceaea/> Add support for 'rust_version'
   This was added to the index in rust-lang/crates.io#6267.  crates.io is
   automatically injecting it into the index, even without using the
   nightly cargo that supports it.
   
   My plan is to use this to make cargo-upgrade more MSRV aware (in version
   reqs despite the resolver not yet being MSRV aware).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 8 calendar days.
 - 29 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index version 0.19.10 ([`286b225`](https://github.com/frewsxcv/rust-crates-index/commit/286b2251ae8a286f8992831f7a845f88227107dd))
    - Merge pull request #103 from epage/msrv ([`87a5cd7`](https://github.com/frewsxcv/rust-crates-index/commit/87a5cd792b29dc82c37f1fc8eb7d86257dd84270))
    - Add support for 'rust_version' ([`ab8c655`](https://github.com/frewsxcv/rust-crates-index/commit/ab8c655d7835a93c87b348c86ecc928ecfaceaea))
    - Allow newer git2 ([`4ab6b51`](https://github.com/frewsxcv/rust-crates-index/commit/4ab6b513660e9dfa9434126557219dfb1e4f233a))
</details>

## v0.19.8 (2023-04-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump deps ([`ae95a95`](https://github.com/frewsxcv/rust-crates-index/commit/ae95a951960ffa750054c20cfac8839988543ca6))
</details>

## v0.19.7 (2023-03-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 11 calendar days.
 - 11 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Try sparse index ([`a6b2595`](https://github.com/frewsxcv/rust-crates-index/commit/a6b2595b7bf6a33e745c74e3b44c1aab260a6622))
    - Change test to tolerate 0-byte files left in the crates.io index ([`83bb44b`](https://github.com/frewsxcv/rust-crates-index/commit/83bb44b57a046ff8a340a4265823e354ffa5481f))
    - Hide helper fn from public API ([`1a0fd59`](https://github.com/frewsxcv/rust-crates-index/commit/1a0fd590a86b57dff307aa69c884344b040b2e56))
    - Report specific error from sparse index cache ([`70b07f5`](https://github.com/frewsxcv/rust-crates-index/commit/70b07f5d0a460450a0abce61a120281a6a1c6986))
    - Clippy ([`fb463b8`](https://github.com/frewsxcv/rust-crates-index/commit/fb463b82ce42f860d32816c7f7652deeb7664348))
    - Tweak inlining ([`29bf4ef`](https://github.com/frewsxcv/rust-crates-index/commit/29bf4ef23d69c363497f27253b78355daad5bd86))
    - Use specific types of ErrorKind ([`d2dc624`](https://github.com/frewsxcv/rust-crates-index/commit/d2dc624550ecf9be57e4db6dcf38e3475db9fe2c))
    - Merge pull request #100 from illicitonion/sparse-url ([`846f600`](https://github.com/frewsxcv/rust-crates-index/commit/846f600a1a1ea12919d3deb4c60343c847b9349d))
    - Support reading local sparse index cache ([`c5682e4`](https://github.com/frewsxcv/rust-crates-index/commit/c5682e4b98adb0fd28630bc94b83e7595f5abe5e))
    - Merge pull request #99 from obi1kenobi/patch-1 ([`efa9b25`](https://github.com/frewsxcv/rust-crates-index/commit/efa9b2506a73efea6dc1ec3b7ebb1ac6912c1279))
    - Fix minor README.md typos ([`819215d`](https://github.com/frewsxcv/rust-crates-index/commit/819215d0c9bb1dcedb52a0f1c4d148044bde6a2c))
    - Merge pull request #98 from illicitonion/sparse-index-dir ([`585b2a6`](https://github.com/frewsxcv/rust-crates-index/commit/585b2a604496216a378067a9b156a4747ae92db9))
</details>

## v0.19.6 (2023-02-24)

<csr-id-34501eae518292acb55a4821214eff9fc03e7aee/>

### Chore

 - <csr-id-34501eae518292acb55a4821214eff9fc03e7aee/> Release crates-index version 0.19.6

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 8 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index version 0.19.6 ([`34501ea`](https://github.com/frewsxcv/rust-crates-index/commit/34501eae518292acb55a4821214eff9fc03e7aee))
    - Make sure parent dir for registry clone exists ([`7dda605`](https://github.com/frewsxcv/rust-crates-index/commit/7dda6052e557c7bf18647087c3aef6b8721c22c4))
</details>

## v0.19.5 (2023-02-15)

<csr-id-6e5d42720fe76834213723ebe95e52e5dd788f15/>

### Chore

 - <csr-id-6e5d42720fe76834213723ebe95e52e5dd788f15/> Release crates-index version 0.19.5

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 8 calendar days.
 - 8 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index version 0.19.5 ([`6e5d427`](https://github.com/frewsxcv/rust-crates-index/commit/6e5d42720fe76834213723ebe95e52e5dd788f15))
    - More verbose error when the git repo has broken HEAD ([`1dbd75b`](https://github.com/frewsxcv/rust-crates-index/commit/1dbd75b41b07d321b9727da970ab33907afb57ed))
    - Support generating local dir for sparse indexes ([`eac9410`](https://github.com/frewsxcv/rust-crates-index/commit/eac9410c70b8af030969ac0a96e1bc21a45bd8c7))
    - Move url_to_local_dir to its own file ([`491c441`](https://github.com/frewsxcv/rust-crates-index/commit/491c4414beec40d99e124e4e3fd94c8cd892b182))
    - Merge pull request #96 from msrd0/https-feature ([`9b21638`](https://github.com/frewsxcv/rust-crates-index/commit/9b21638187728e151cabc392d110492bb49c26cb))
</details>

## v0.19.4 (2023-02-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 5 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump TOML crate ([`f4fea2d`](https://github.com/frewsxcv/rust-crates-index/commit/f4fea2d0fbfd2b9690aad159f7ceaf4fcc44fcb5))
    - Allow enabling the https feature by itself ([`196a5ae`](https://github.com/frewsxcv/rust-crates-index/commit/196a5ae0fce3a4a45d11632d4bec6a4b15ba8ee6))
</details>

## v0.19.3 (2023-02-01)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make INDEX_GIT_URL public ([`527e72b`](https://github.com/frewsxcv/rust-crates-index/commit/527e72bc2775b6ac8efd3d1e759091e06a702246))
</details>

## v0.19.2 (2023-01-29)

<csr-id-a3407ce2f58217e0b4dc30552cf65d4a11d67d5a/>

### Chore

 - <csr-id-a3407ce2f58217e0b4dc30552cf65d4a11d67d5a/> Release crates-index version 0.19.2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index version 0.19.2 ([`a3407ce`](https://github.com/frewsxcv/rust-crates-index/commit/a3407ce2f58217e0b4dc30552cf65d4a11d67d5a))
    - Merge pull request #94 from obi1kenobi/windows_path_handling ([`4493ff3`](https://github.com/frewsxcv/rust-crates-index/commit/4493ff33bc4e0d55e750b97c2826b2fbf468cd95))
    - Do not assume that `/` is the system's path separator char. ([`4c62196`](https://github.com/frewsxcv/rust-crates-index/commit/4c62196ead4cc82779289576b5fa0ca9a4c83b17))
</details>

## v0.19.1 (2023-01-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`11fd701`](https://github.com/frewsxcv/rust-crates-index/commit/11fd70142de64a87eb490a15273c0abd3455853d))
    - Merge pull request #90 from Enselic/smol_str ([`3329728`](https://github.com/frewsxcv/rust-crates-index/commit/3329728d2a6d33738dee69b302fd6d1862be8782))
    - Merge branch 'master' into smol_str ([`8737653`](https://github.com/frewsxcv/rust-crates-index/commit/8737653c6c8fdd949deb850c450ce12c47111618))
</details>

## v0.19.0 (2023-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 58 calendar days.
 - 81 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Upgrade deps ([`c146ddc`](https://github.com/frewsxcv/rust-crates-index/commit/c146ddcb6c59cad505384489f8fcf66e5f1a7554))
    - Bump git2 ([`415875c`](https://github.com/frewsxcv/rust-crates-index/commit/415875ccfa052bf07a7641db1e5fecd820e20c5f))
    - Programming in YAML ;( ([`75e967c`](https://github.com/frewsxcv/rust-crates-index/commit/75e967ce86ce785d9e6f3b46e83c369f23f89399))
    - Start using `smol_str` again to avoid MPL 2.0 license of `smartstring` ([`3fcf7be`](https://github.com/frewsxcv/rust-crates-index/commit/3fcf7be52bc076fadc69e8a498161983e47fd92d))
    - Fix method name in readme ([`868d651`](https://github.com/frewsxcv/rust-crates-index/commit/868d651f783fae41e79c9eee01d2679f53dd90e7))
</details>

## v0.18.11 (2022-11-04)

<csr-id-18253ffa6c5d837efdf607718270c5845ee76f70/>

### New Features

 - <csr-id-4c593aa9e8b4b84b048839321b2d091660df7602/> add support for replaced source in Cargo config.toml

### Bug Fixes

 - <csr-id-6780e1f979a1439d36a047b9466bec7c50a94884/> follow cargo's search order of .cargo/config.toml

### Style

 - <csr-id-18253ffa6c5d837efdf607718270c5845ee76f70/> fix format

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 1 calendar day.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`62c50c4`](https://github.com/frewsxcv/rust-crates-index/commit/62c50c4933f16d6ca3fcf75356c01446476eec5f))
    - Idioms fixes ([`cde56f4`](https://github.com/frewsxcv/rust-crates-index/commit/cde56f45ce38e579712397f1b452184fbd6ac927))
    - From for io::Error ([`0acf0ac`](https://github.com/frewsxcv/rust-crates-index/commit/0acf0ac0ac8f719550f0bda5cbe4abff94b124b2))
    - The index keeps growing ([`8d44a48`](https://github.com/frewsxcv/rust-crates-index/commit/8d44a48b50b81de1e41f928ceb55a8fd54608d25))
    - Fix format ([`18253ff`](https://github.com/frewsxcv/rust-crates-index/commit/18253ffa6c5d837efdf607718270c5845ee76f70))
    - Follow cargo's search order of .cargo/config.toml ([`6780e1f`](https://github.com/frewsxcv/rust-crates-index/commit/6780e1f979a1439d36a047b9466bec7c50a94884))
    - Add support for replaced source in Cargo config.toml ([`4c593aa`](https://github.com/frewsxcv/rust-crates-index/commit/4c593aa9e8b4b84b048839321b2d091660df7602))
</details>

## v0.18.10 (2022-10-06)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 55 calendar days.
 - 55 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Trim port ([`f490ed2`](https://github.com/frewsxcv/rust-crates-index/commit/f490ed28d0e1c6dfc4776f2e9ad3a48ce42f4213))
    - Typo ([`514e27b`](https://github.com/frewsxcv/rust-crates-index/commit/514e27b932fd912d238c4f8fd3d0c375a60bf554))
    - The index keeps growing ([`4e366ad`](https://github.com/frewsxcv/rust-crates-index/commit/4e366adf2711b6adb83fc38a3e5217d1a7d53f1c))
</details>

## v0.18.9 (2022-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 74 calendar days.
 - 80 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`c95ec0b`](https://github.com/frewsxcv/rust-crates-index/commit/c95ec0ba84ffc35eb9588947f10e45aa59473b37))
    - Bump git version spec to allow latest 0.15 ([`3d08a3c`](https://github.com/frewsxcv/rust-crates-index/commit/3d08a3ccbc1fd27e9cc0e2a4f4e3d52fbad34b4d))
    - Make highest_normal_version return non-yanked versions ([`b44a814`](https://github.com/frewsxcv/rust-crates-index/commit/b44a814b05bfb59bf20f9bcbd048197bc94ca138))
    - Avoid building libssh ([`95b9fdd`](https://github.com/frewsxcv/rust-crates-index/commit/95b9fdd4bab3e7c11dc5dc3ef2c43444873b314d))
</details>

## v0.18.8 (2022-05-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`2f41bee`](https://github.com/frewsxcv/rust-crates-index/commit/2f41beefcd7b4b08a1ecba4f1bfda6d1d17e6774))
    - Docs ([`4a1e080`](https://github.com/frewsxcv/rust-crates-index/commit/4a1e0801267eb942406d56f1f63b40be5f39044d))
    - Fmt/Clippy ([`5849a3a`](https://github.com/frewsxcv/rust-crates-index/commit/5849a3a844a4f7ebb54ae908b4718896caaceffd))
    - Expose index url ([`2441915`](https://github.com/frewsxcv/rust-crates-index/commit/24419158453ad2de6f7e8a5a96006d3e75606b91))
    - The index keeps growing ([`6556a22`](https://github.com/frewsxcv/rust-crates-index/commit/6556a227677d8c6c44d22d301d96145679fba8c1))
    - Replace tempdir crate with tempfile, as tempfile have superseded tempdir ([`ebd3506`](https://github.com/frewsxcv/rust-crates-index/commit/ebd3506b1c4a93655ac87e8cb905c072fc772358))
    - Update test ([`40f0d51`](https://github.com/frewsxcv/rust-crates-index/commit/40f0d5197b53bfbc00eeff591bd711c291ad7c9a))
    - Bump ([`8224ecf`](https://github.com/frewsxcv/rust-crates-index/commit/8224ecfd9646f67cdf36557ac3a6f7c7dfffd804))
</details>

## v0.18.6 (2022-02-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 21 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump deps ([`5a423f0`](https://github.com/frewsxcv/rust-crates-index/commit/5a423f07ae58059bbc428b3573db9a3731a54532))
    - Upgrade `git2` dependency ([`2f4d6be`](https://github.com/frewsxcv/rust-crates-index/commit/2f4d6be5c814a69c6d0819711bae73e17b926384))
</details>

## v0.18.5 (2022-02-04)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Parallel iterator ([`78a7611`](https://github.com/frewsxcv/rust-crates-index/commit/78a7611bee17b3841365441eeb19675284a0d11a))
    - Avoid allocating lowercase str ([`9ed2cca`](https://github.com/frewsxcv/rust-crates-index/commit/9ed2cca8039f79623a37218dc50a4cb59c0e7af6))
    - Dedupe across all crates where possible ([`a5c9d72`](https://github.com/frewsxcv/rust-crates-index/commit/a5c9d729cfcaa03e2a11c0f0375ddc9e6926d2e4))
    - Dedupe features using a hashset ([`9e3497f`](https://github.com/frewsxcv/rust-crates-index/commit/9e3497fe74a7d7354a73c38b10251843f9130f09))
</details>

## v0.18.4 (2022-02-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 10 calendar days.
 - 13 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Revert "Resolve default at parse time" ([`3846c73`](https://github.com/frewsxcv/rust-crates-index/commit/3846c733c56a6d5d29aa8ecb18b0d99cdf96ee3b))
    - Reduce mem of unused features ([`ccf7a83`](https://github.com/frewsxcv/rust-crates-index/commit/ccf7a834c7fe1fcbcc0fe10cf8459c1e9467ce65))
    - Resolve default at parse time ([`aeb596f`](https://github.com/frewsxcv/rust-crates-index/commit/aeb596ffbd93bf41d4479542505e695babfa8c2f))
    - Reduce memory cost of features2 field ([`d8da78d`](https://github.com/frewsxcv/rust-crates-index/commit/d8da78d4fe05e0517878e698dba64c5b8aa162cf))
    - Avoid temp string when computing path prefix ([`c84ff49`](https://github.com/frewsxcv/rust-crates-index/commit/c84ff49690a3c11bc6b605ea1839bf97039b9e87))
    - Merge pull request #76 from pinkforest/master ([`49258c8`](https://github.com/frewsxcv/rust-crates-index/commit/49258c8bb4d2ed3166cd995eb9711ad0d1d9f312))
    - Docs + fmt: Unconstrained expectations ([`0eda4d8`](https://github.com/frewsxcv/rust-crates-index/commit/0eda4d829b9ca49b86a3e7e4cee734d795ad1074))
    - Support features2 ([`3750c75`](https://github.com/frewsxcv/rust-crates-index/commit/3750c75bb1aae43406576d4886b701189bfd5374))
    - Docs ([`297a101`](https://github.com/frewsxcv/rust-crates-index/commit/297a101821157ea2e4ff197bff14b05aa1e450a6))
</details>

## v0.18.2 (2022-01-20)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.18.2 ([`a03b64f`](https://github.com/frewsxcv/rust-crates-index/commit/a03b64ffc714c00164c3ad4f929ea3c364de2220))
    - Remove self-referential struct ([`5b67f4d`](https://github.com/frewsxcv/rust-crates-index/commit/5b67f4d60f114c4e7c91b20c145788c81fa60b9c))
</details>

## v0.18.1 (2021-10-25)

<csr-id-9984f8920bea2fbeea999137b33aae8d8eb2f094/>

### Chore

 - <csr-id-9984f8920bea2fbeea999137b33aae8d8eb2f094/> Switch to shields.io for badge, closes #67

### New Features

 - <csr-id-8e3c29369d6adec961a1e9b421bc8699a48a04de/> Support non-crates.io registries
   This matches cargo's behavior for registries that are not hosted on
   GitHub (like Cloudsmith), and uses the configured git credential helper
   to allow cloning private registries.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 7 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #71 from bearcove/amos/fix-badge ([`55355b3`](https://github.com/frewsxcv/rust-crates-index/commit/55355b318b46f59f43f24bfb48ec8f9ebd7b1a5b))
    - Switch to shields.io for badge, closes #67 ([`9984f89`](https://github.com/frewsxcv/rust-crates-index/commit/9984f8920bea2fbeea999137b33aae8d8eb2f094))
    - Bump ([`1448bc0`](https://github.com/frewsxcv/rust-crates-index/commit/1448bc058932c0eaf0870243d8b7b5f7768ab20e))
    - Fix test ([`1a2d471`](https://github.com/frewsxcv/rust-crates-index/commit/1a2d47162d73e1715c1ffaadd6504e8d718a6f21))
    - Merge pull request #69 from bearcove/private-crate-registries ([`8977189`](https://github.com/frewsxcv/rust-crates-index/commit/89771894f3e24a747e3215ee1714d2ffd0461dd8))
    - Support non-crates.io registries ([`8e3c293`](https://github.com/frewsxcv/rust-crates-index/commit/8e3c29369d6adec961a1e9b421bc8699a48a04de))
</details>

## v0.18.0 (2021-10-18)

It should work without any code changes. Only the `git2` and `toml` dependencies were updated.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 63 calendar days.
 - 143 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #68 from UebelAndre/nossl ([`0a79562`](https://github.com/frewsxcv/rust-crates-index/commit/0a79562358b44ffb96129e430ae4b9d11d4fb86e))
    - Allow building without an openssl dependency ([`bc72a0f`](https://github.com/frewsxcv/rust-crates-index/commit/bc72a0fec430c043d1dadb8539b192e663e7ea7e))
    - Merge pull request #59 from frewsxcv/bare-index ([`17c27a3`](https://github.com/frewsxcv/rust-crates-index/commit/17c27a3c81bc9e517e16dc9f74efc1f8e532301e))
    - Test ([`868d870`](https://github.com/frewsxcv/rust-crates-index/commit/868d8706cf80b2e2f34091b58c2b15f85af0de64))
    - Optimize ([`4632408`](https://github.com/frewsxcv/rust-crates-index/commit/4632408fb6639ea2f9bd5e79265d10217f4ee329))
    - Docs ([`542669e`](https://github.com/frewsxcv/rust-crates-index/commit/542669e71f8bf9d819e641d6c002c77a16f3cc75))
    - Skip cache dir ([`17e2462`](https://github.com/frewsxcv/rust-crates-index/commit/17e2462f4feaaf49a64983866aa9940ed979b979))
    - Delete checkout-based implementation ([`67181db`](https://github.com/frewsxcv/rust-crates-index/commit/67181dbf9e3485ea2340b02fe908e0fae72eda07))
    - Rename CratesRefs iter ([`eda4e15`](https://github.com/frewsxcv/rust-crates-index/commit/eda4e1597bea2abcdd9d785269c4d56fe00efaf3))
    - Back-compat shim ([`c23932d`](https://github.com/frewsxcv/rust-crates-index/commit/c23932d5b9343f56fa4e376b3942a3ca74d84a30))
    - Unify BareIndex into a single type ([`164e58c`](https://github.com/frewsxcv/rust-crates-index/commit/164e58cce84b69709285d6973d15596c28006031))
    - The index is retrieved automatically, so this is an update ([`b7f83df`](https://github.com/frewsxcv/rust-crates-index/commit/b7f83df2e0ce95b36a5346985bc6408432e2153e))
</details>

## v0.17.0 (2021-05-27)

### Migration Notes

* `BareIndex` and `BareIndexRepo` have become the `Index`.
* `Index::new_cargo_default()?` is the preferred way of accessing the index. Use `with_path()` to clone to a different directory.
* There's no need to call `retrieve()` or `exists()`. It's always retrieved and always exists.
* `retrieve_or_update()` is just `update()`.
* `highest_version()` returns crate metadata rather than just the version number. Call `highest_version().version().parse()` to get `semver::Version`.
* There's no `crate_index_paths()`, because there are no files any more. Use `crate_` to get individual crates.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update docs ([`8f194b7`](https://github.com/frewsxcv/rust-crates-index/commit/8f194b72febf3f1dd64a4445d7b01dfd371c35e0))
    - Return all crate version data instead of a SemVer object ([`cc5e6d9`](https://github.com/frewsxcv/rust-crates-index/commit/cc5e6d9ea88b7be09a3340941722bcdc5108bde9))
    - Don't expose Cargo-internal format ([`38a027d`](https://github.com/frewsxcv/rust-crates-index/commit/38a027d9231bf8fa9ad4de6e5ffd89a714805918))
    - Remove deprecated functions ([`e147cc6`](https://github.com/frewsxcv/rust-crates-index/commit/e147cc674ca8a2da56bbd68ad7eb3ca10c4dac4f))
    - Upgrade semver crate ([`00a80d6`](https://github.com/frewsxcv/rust-crates-index/commit/00a80d669c16097d4e6d384828d0a6a15fe7f0bb))
</details>

## v0.16.7 (2021-05-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`95d4372`](https://github.com/frewsxcv/rust-crates-index/commit/95d437278b210835285e2bc4b57b953d00a311c6))
    - Merge pull request #57 from frewsxcv/fetch ([`7e125f9`](https://github.com/frewsxcv/rust-crates-index/commit/7e125f97f8a564a567be77eb5bd525ddaa65c488))
    - Fetch both HEAD and master of the repo ([`d99c446`](https://github.com/frewsxcv/rust-crates-index/commit/d99c4462e504d828bd6891b385154b00443b1d22))
    - Merge pull request #56 from pksunkara/pksunkara-patch-1 ([`b3ad852`](https://github.com/frewsxcv/rust-crates-index/commit/b3ad852a4750c725a9f554b02af610955facc594))
    - Get index directory for bare index ([`0e9e407`](https://github.com/frewsxcv/rust-crates-index/commit/0e9e4078d11263b4351e7fe0aedb2a3d7a9ea61f))
    - Bump deps ([`80ee809`](https://github.com/frewsxcv/rust-crates-index/commit/80ee809e85afb7a894c8d11a263fb936508d5e1b))
</details>

## v0.16.6 (2021-05-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Lower memory usage by deduplicating version's data ([`0b6b817`](https://github.com/frewsxcv/rust-crates-index/commit/0b6b817a71b1c8b3b55383ed9591654eb31fb2a1))
    - Bump ([`d4eecf3`](https://github.com/frewsxcv/rust-crates-index/commit/d4eecf3d0620b85235997053684777d738387d1b))
    - Merge pull request #55 from illicitonion/config.json ([`4af7f4f`](https://github.com/frewsxcv/rust-crates-index/commit/4af7f4f22ff486705307bc6650ca483e76cf86d5))
    - Allow parsing config.json data from an index ([`a7f882c`](https://github.com/frewsxcv/rust-crates-index/commit/a7f882c5dcfffdce89637c7be2e59ad4c69fc17f))
    - Try to make self-referential struct safer for LLVM noalias ([`8884604`](https://github.com/frewsxcv/rust-crates-index/commit/88846040af4e5da9c68b35d7b128771057580184))
    - Unused field ([`b423e25`](https://github.com/frewsxcv/rust-crates-index/commit/b423e2522f5e846145e23afdc7068fcef964ac62))
</details>

## v0.16.3 (2021-03-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 128 calendar days.
 - 128 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`ea18f98`](https://github.com/frewsxcv/rust-crates-index/commit/ea18f985f9862d93dd2dab3d5e2ae5de13e0e8cc))
    - Merge pull request #53 from jtgeibel/checkout-head-not-master ([`8f47eea`](https://github.com/frewsxcv/rust-crates-index/commit/8f47eeab03310f02a24d8f0dfe700ff326c82a93))
    - Checkout `HEAD` instead of `master` ([`64fab0f`](https://github.com/frewsxcv/rust-crates-index/commit/64fab0f0fc2078df0782ebb01e15db3cb5af671d))
    - Bye Travis ([`183db53`](https://github.com/frewsxcv/rust-crates-index/commit/183db53404d00dbfdcd9b736e60668a8546067b3))
    - Add GitHub Actions ([`0db1992`](https://github.com/frewsxcv/rust-crates-index/commit/0db199251f4bbd2decce51957daccd3921a97008))
    - Merge pull request #48 from Eh2406/small-checkout ([`1c4a0bb`](https://github.com/frewsxcv/rust-crates-index/commit/1c4a0bbb7657c9f30ad13247a0b97d46d4d251bf))
    - Blobs to refs ([`376bf5d`](https://github.com/frewsxcv/rust-crates-index/commit/376bf5d1db43a12fe32a05f0947d0ae7ffe85510))
    - Fetch HEAD instead of master ([`31d7e72`](https://github.com/frewsxcv/rust-crates-index/commit/31d7e72c29f7eeac2375c8498e0cb70772055b0c))
    - Avoid recursion ([`4ad7bc2`](https://github.com/frewsxcv/rust-crates-index/commit/4ad7bc20e75346ce90441c6409e8995f69d2e8ac))
</details>

## v0.16.2 (2020-11-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 2 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Separate blob iterator ([`df15945`](https://github.com/frewsxcv/rust-crates-index/commit/df159459d8cdae52ab496d0ff029e49c23f14ae6))
    - Deprecate filesystem paths ([`bcf9cad`](https://github.com/frewsxcv/rust-crates-index/commit/bcf9cad0aa458fceed79107debd9e64f8770207c))
    - Adjust inlining ([`7ed3263`](https://github.com/frewsxcv/rust-crates-index/commit/7ed32635d02c18c6f37cf921349da38c21eae1d2))
    - Ignore files at the top level of bare repos ([`bf69757`](https://github.com/frewsxcv/rust-crates-index/commit/bf697577c361a929aa0e7349f3b8b925085a0f8a))
    - Add a crates iterator on bare indexes ([`3310a19`](https://github.com/frewsxcv/rust-crates-index/commit/3310a19d86aa36d9aad0851faba76445929510b9))
    - Reduce the size of chekouts ([`d628c1b`](https://github.com/frewsxcv/rust-crates-index/commit/d628c1b063de871f2d2f72202275f80eaaa86057))
    - Bump ([`ca3b2e4`](https://github.com/frewsxcv/rust-crates-index/commit/ca3b2e4ad725e989c55cbbd4485804c5ba09bfe9))
    - Add a links field ([`8bf04c5`](https://github.com/frewsxcv/rust-crates-index/commit/8bf04c5da79b84fcd296a2238130da41c18ebc98))
</details>

## v0.16.0 (2020-10-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Docs ([`d6a310d`](https://github.com/frewsxcv/rust-crates-index/commit/d6a310d678d107c99108e702648c654d44a3c105))
    - Bump SemVer crate ([`62f793a`](https://github.com/frewsxcv/rust-crates-index/commit/62f793ac4637a3ed5df7b54b4f67598f95b0be5f))
</details>

## v0.15.5 (2020-10-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 30 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Remove smol_str due to MSRV ([`7e945b3`](https://github.com/frewsxcv/rust-crates-index/commit/7e945b37dc94a442438bd1d8c2da9ee81ac3f0ad))
</details>

## v0.15.4 (2020-09-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#41](https://github.com/frewsxcv/rust-crates-index/issues/41)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#41](https://github.com/frewsxcv/rust-crates-index/issues/41)**
    - Add support for bare repos ([`007a12a`](https://github.com/frewsxcv/rust-crates-index/commit/007a12a66d9c2d006ddee19ddfc5311621a2b63e))
 * **Uncategorized**
    - Bump ([`6499dba`](https://github.com/frewsxcv/rust-crates-index/commit/6499dba44e91fda5e8b2e058d11163fab2ca72e6))
    - Keep self-referential fields together ([`01cb3f2`](https://github.com/frewsxcv/rust-crates-index/commit/01cb3f210932b88ce76885bdfc4c5e061bd42dc8))
    - Inline unwrap ([`38d0f9c`](https://github.com/frewsxcv/rust-crates-index/commit/38d0f9ccdfc6ac1959c2854b6fab0a02dcdc5d77))
    - Test ([`e91e8c5`](https://github.com/frewsxcv/rust-crates-index/commit/e91e8c5df47834a1d1bc09a9a342025408e4d609))
</details>

## v0.15.3 (2020-09-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 128 calendar days.
 - 128 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`1636fd2`](https://github.com/frewsxcv/rust-crates-index/commit/1636fd2f3a592c992f5484327ea6ba7c5f618bc1))
    - Add vendored-openssl feature ([`5162881`](https://github.com/frewsxcv/rust-crates-index/commit/5162881dcc7d9614289fa9990e03d23f50cde39b))
    - Fix iteration of 3 character crate names ([`edf74e8`](https://github.com/frewsxcv/rust-crates-index/commit/edf74e8bf86f2d1f9dc20efef14fa53a4976c7e4))
    - Make code less readable ([`b43ad41`](https://github.com/frewsxcv/rust-crates-index/commit/b43ad418992502bac6e97d8f0350deacee9a86aa))
    - Bump ([`0100225`](https://github.com/frewsxcv/rust-crates-index/commit/01002253f272f973a7867dd57f56d48624102797))
    - Merge pull request #39 from SirWindfield/feat_semver ([`fe13ae5`](https://github.com/frewsxcv/rust-crates-index/commit/fe13ae513251fb7bb0f6d04e9e4ad090c5f46983))
    - Fix docs ([`0e0a830`](https://github.com/frewsxcv/rust-crates-index/commit/0e0a8303145d97f53ec26ef9b9672cd44be54315))
    - `make highest_stable_version` inline ([`7325fca`](https://github.com/frewsxcv/rust-crates-index/commit/7325fca645072fa2ea5795e8a67bf5ee8e26c3bd))
    - Add documentation ([`81b157e`](https://github.com/frewsxcv/rust-crates-index/commit/81b157e07322b89fb03ef613cc52509ca129befb))
    - Rename methods ([`9edd872`](https://github.com/frewsxcv/rust-crates-index/commit/9edd87283abee7d9f6f442a06b84d131a36b46b6))
    - Add `latest_semver_version` method ([`edb6a2e`](https://github.com/frewsxcv/rust-crates-index/commit/edb6a2e84931598608d09045ff5dcd8b29979b09))
    - Bump ([`e4bf5c4`](https://github.com/frewsxcv/rust-crates-index/commit/e4bf5c40517bd412277b09fb3ec5159e120dabf4))
    - Expose Crate::from_slice ([`367a02f`](https://github.com/frewsxcv/rust-crates-index/commit/367a02ff752d458e3754986e917f8e41b13a0aa7))
    - Bump ([`7578a44`](https://github.com/frewsxcv/rust-crates-index/commit/7578a44906964188c7d493f6d284c87ec775d0cf))
    - Merge pull request #36 from frewsxcv/breaking ([`7046c8f`](https://github.com/frewsxcv/rust-crates-index/commit/7046c8fb337a271905200bdc59765e1213a4c199))
    - Use enum for dependency kind ([`02c87dd`](https://github.com/frewsxcv/rust-crates-index/commit/02c87ddc1aa6330bbd8ae0fb17e612bf34734735))
    - Rename new_checked to new ([`383d303`](https://github.com/frewsxcv/rust-crates-index/commit/383d303cb1768c5a658ed8b7dc339047bfb4174f))
    - Parse checksum as 32 bytes ([`d69e66a`](https://github.com/frewsxcv/rust-crates-index/commit/d69e66accc8008b005d56469952c65981f4fcc67))
    - Drop error-chain ([`aa09d49`](https://github.com/frewsxcv/rust-crates-index/commit/aa09d490cdbf4b48f8d117e3a26795f8f0c95138))
    - Merge pull request #35 from frewsxcv/ed2018 ([`fa094a5`](https://github.com/frewsxcv/rust-crates-index/commit/fa094a54998f66d38a63e4f9bb43a13d739c32cb))
    - Bump ([`ce8e3b8`](https://github.com/frewsxcv/rust-crates-index/commit/ce8e3b888a6006666b0ed57313d894bef52d8331))
    - Edition 2018 ([`69d2256`](https://github.com/frewsxcv/rust-crates-index/commit/69d225677d004b99d9880cf6999ea04f2b8fd88a))
</details>

## v0.14.4 (2020-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`6c498c1`](https://github.com/frewsxcv/rust-crates-index/commit/6c498c17e3b601611131d5cd158a057ad1dae900))
    - Fewer allocations ([`a117018`](https://github.com/frewsxcv/rust-crates-index/commit/a117018713b27276371c658727e6d21824472eb4))
    - Internal from_slice() ([`d9f6571`](https://github.com/frewsxcv/rust-crates-index/commit/d9f65710f68210e5d2de7e8858d686d96b8c2b07))
    - Inline getters ([`eacca00`](https://github.com/frewsxcv/rust-crates-index/commit/eacca0094d476596563d453834509e8137def1b1))
</details>

## v0.14.3 (2020-03-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 11 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update deps ([`d93eb92`](https://github.com/frewsxcv/rust-crates-index/commit/d93eb92068c14192d7c0dba15eed5fdb7a985697))
</details>

## v0.14.2 (2020-03-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 3 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`aefb8c3`](https://github.com/frewsxcv/rust-crates-index/commit/aefb8c3c6d73e246a5d8a6b233708e4cd66c7802))
    - Quicker code example ([`0d9a202`](https://github.com/frewsxcv/rust-crates-index/commit/0d9a2020e132af84f7ffca174440fdd30a84bdc6))
    - Merge pull request #34 from frewsxcv/smol ([`c0703bd`](https://github.com/frewsxcv/rust-crates-index/commit/c0703bd301926142fb0af99ad66dd3c03b51d98e))
    - Use boxed slices instead of resizeable Vecs ([`9cc5fe0`](https://github.com/frewsxcv/rust-crates-index/commit/9cc5fe0475947e51fa7aee45fdf31a5674afc211))
    - Reduce reallocations during parsing ([`538fcd0`](https://github.com/frewsxcv/rust-crates-index/commit/538fcd048809909e3f281e15f7151e74b6ac1eec))
    - Use smaller string types ([`5064ded`](https://github.com/frewsxcv/rust-crates-index/commit/5064dedad02804b1396ce3844824eb0303ac9840))
</details>

## v0.14.1 (2020-02-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump ([`8c0d236`](https://github.com/frewsxcv/rust-crates-index/commit/8c0d2360a736f19120d7e0e2e3220a4ef7ec8c69))
    - Merge pull request #33 from DCjanus/master ([`6b3e070`](https://github.com/frewsxcv/rust-crates-index/commit/6b3e07008f4c3d9bd65f93960b93bae82e4506e3))
    - Upgrade dependencies(git2) 0.11 -> 0.12 ([`7eff5a0`](https://github.com/frewsxcv/rust-crates-index/commit/7eff5a0c9341bb2a6b670c2ff027813e323725d7))
</details>

## v0.14.0 (2020-02-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#30](https://github.com/frewsxcv/rust-crates-index/issues/30)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#30](https://github.com/frewsxcv/rust-crates-index/issues/30)**
    - Generate path instead of searching for it ([`7aa6a0a`](https://github.com/frewsxcv/rust-crates-index/commit/7aa6a0a0bd20daa5372b2f0d1ceade4341de0b62))
 * **Uncategorized**
    - Prepare for 0.14.0 release ([`6ca2508`](https://github.com/frewsxcv/rust-crates-index/commit/6ca25085fbed544252c4177f7cbd2285405a8ceb))
    - Merge pull request #31 from tjodden/autoproxy ([`2623474`](https://github.com/frewsxcv/rust-crates-index/commit/26234746040b0707feacaa10de462036ea121eda))
    - Merge pull request #32 from tjodden/clippy ([`e98cf29`](https://github.com/frewsxcv/rust-crates-index/commit/e98cf29d6f8523fae5361c328e51223fe4d96393))
    - Simplify getters by using Option::as_deref() ([`0056c96`](https://github.com/frewsxcv/rust-crates-index/commit/0056c9696203029080accaaea71fe5becb2471cb))
    - Remove redundant 'static ([`bcad902`](https://github.com/frewsxcv/rust-crates-index/commit/bcad902ef4a5463417f4f2a928e6bf4efee7c69d))
    - Try to auto-detect proxy settings ([`830c29c`](https://github.com/frewsxcv/rust-crates-index/commit/830c29ccfc5882e547fd17bff34eb37ddc4385e7))
    - Use shorthand struct field initialization syntax ([`597f735`](https://github.com/frewsxcv/rust-crates-index/commit/597f735131415b017e7c431d34eb6557d362e419))
    - Avoid doctest polluting source dir ([`1d009a6`](https://github.com/frewsxcv/rust-crates-index/commit/1d009a6299c92089366727b1bd591c3ea63a3280))
    - Bump ([`a06a9db`](https://github.com/frewsxcv/rust-crates-index/commit/a06a9db7a599eb9609081964296248c0e7278476))
    - Don't serialize default kind ([`d90e65d`](https://github.com/frewsxcv/rust-crates-index/commit/d90e65dbcc98e45909070df3617e90588ba4ba04))
    - Handle errors when loading crate files ([`64e9c78`](https://github.com/frewsxcv/rust-crates-index/commit/64e9c7880b97d05dcfed4558fdb616cce5cecb3a))
    - Generate path instead of searching for it ([`1f8d06a`](https://github.com/frewsxcv/rust-crates-index/commit/1f8d06ab82d2720a9ca82e8a942832654586314e))
    - Skip serializing null package ([`e5d21bb`](https://github.com/frewsxcv/rust-crates-index/commit/e5d21bb68d369cdcec8603b57fab0fda97cb93c9))
</details>

## v0.13.3 (2019-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 37 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump deps ([`42da721`](https://github.com/frewsxcv/rust-crates-index/commit/42da721154ac3f97d313319b50f5ebcde5be2af6))
    - Check that the index works ([`4919c3f`](https://github.com/frewsxcv/rust-crates-index/commit/4919c3f2ffba1d33b5774bfa3592ace410bc0f43))
    - Bump ([`39dfa99`](https://github.com/frewsxcv/rust-crates-index/commit/39dfa99227d5dabccc6f4ea2d274b2830345a248))
    - Merge pull request #29 from kornelski/master ([`e26c15f`](https://github.com/frewsxcv/rust-crates-index/commit/e26c15f1140eb8c23398497fd616a733b644ef0c))
    - Update dependencies ([`f466d9f`](https://github.com/frewsxcv/rust-crates-index/commit/f466d9f56ca0000df1d0da06944c432c5d5e4fcc))
    - Merge pull request #28 from kornelski/cargo-index ([`969060f`](https://github.com/frewsxcv/rust-crates-index/commit/969060ff3adb3e7c7c9789a7efdc85e168a8f53e))
    - Merge pull request #27 from kornelski/patch-1 ([`ad80ab8`](https://github.com/frewsxcv/rust-crates-index/commit/ad80ab85a47173ebce281fffca84a9aefbffb10e))
    - Support reading Cargo's own clone directly ([`53197ff`](https://github.com/frewsxcv/rust-crates-index/commit/53197ff6a12cd7de8fca6cdf3084112009120d26))
    - Add categories ([`438800d`](https://github.com/frewsxcv/rust-crates-index/commit/438800dc5d2826dce00a67fd7c01d3f53b702938))
</details>

## v0.13.1 (2019-06-22)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare for 0.13.0 release ([`3487d54`](https://github.com/frewsxcv/rust-crates-index/commit/3487d54480faeef7abda34d4757a683d923ae69a))
    - Merge pull request #26 from theduke/crate_deserialize ([`97f0d3e`](https://github.com/frewsxcv/rust-crates-index/commit/97f0d3ef04c01b69b17423196a063bded1c6fc4c))
    - Implement Deserialize and Clone for Crate ([`9a723ab`](https://github.com/frewsxcv/rust-crates-index/commit/9a723ab7770da38c62e2151ce2d3aa0c125de236))
</details>

## v0.13.0 (2019-03-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare for 0.13.0 release ([`4c70a2e`](https://github.com/frewsxcv/rust-crates-index/commit/4c70a2edd68b7409204b946bbcf030da3e8e5921))
    - Merge pull request #25 from theduke/serde-serialize ([`b319667`](https://github.com/frewsxcv/rust-crates-index/commit/b3196672ed686aad56ae9107205e3140e97de01c))
    - Allow data types to be serialized. ([`199b0b0`](https://github.com/frewsxcv/rust-crates-index/commit/199b0b0b42e2cdcc2e22b4f4febc3f507316069c))
    - Merge pull request #23 from kornelski/master ([`e562ad2`](https://github.com/frewsxcv/rust-crates-index/commit/e562ad2ce78c0a1dbf500db452f8d8cc917b07c9))
    - Bump dependencies ([`9e0ab33`](https://github.com/frewsxcv/rust-crates-index/commit/9e0ab3344280551a8cc1dc63393ada985af6c46f))
</details>

## v0.12.1 (2018-12-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare for 0.12.1 release. ([`476c25a`](https://github.com/frewsxcv/rust-crates-index/commit/476c25a7857b771a41cdfa09475918cae65b1067))
    - Merge #22 ([`9cffd56`](https://github.com/frewsxcv/rust-crates-index/commit/9cffd566ecb5e865b4be732c920a9eef4308d983))
    - Update README.md ([`05fd0c6`](https://github.com/frewsxcv/rust-crates-index/commit/05fd0c6e596f3c5e451d83f1190b1a3a3bfa617f))
    - Add the method `Dependency::crate_name()`. ([`a892ef6`](https://github.com/frewsxcv/rust-crates-index/commit/a892ef6f61d985973f802b9857326f88d8cef0f4))
</details>

## v0.12.0 (2018-04-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare for 0.12.0 release. ([`9eafffa`](https://github.com/frewsxcv/rust-crates-index/commit/9eafffa6a275bfc26ec54d1c3e2e0280535cb3f5))
    - Bump git2: 0.6 -> 0.7. ([`962f054`](https://github.com/frewsxcv/rust-crates-index/commit/962f054fd79461ce53b1e1aed9b202786e696946))
    - Merge pull request #19 from kornelski/master ([`0d8c4a3`](https://github.com/frewsxcv/rust-crates-index/commit/0d8c4a3fddc5c85f53928179ed58377e44ea3607))
    - Remove try!() ([`111367f`](https://github.com/frewsxcv/rust-crates-index/commit/111367f96f0443601896775373d800e24b979613))
    - Make tests runnable in parallel ([`1bb4526`](https://github.com/frewsxcv/rust-crates-index/commit/1bb45260009f0587e04857450f334e83adbe4908))
    - Implement Debug ([`aab4c74`](https://github.com/frewsxcv/rust-crates-index/commit/aab4c74814815ca3518d6dc9208807561f172597))
</details>

## v0.11.0 (2018-02-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version ([`6ca5581`](https://github.com/frewsxcv/rust-crates-index/commit/6ca5581676604d5b7fe2da30244b98fd91b1e2f2))
    - Remove now-unneeded import ([`0203cc8`](https://github.com/frewsxcv/rust-crates-index/commit/0203cc828d72c269545cdb77e99043663afac90b))
    - Bump error-chain ([`4a7dc3f`](https://github.com/frewsxcv/rust-crates-index/commit/4a7dc3f7c9260c75451912b7005841135fde0ee7))
    - Merge pull request #18 from Michael-F-Bryan/master ([`bc9be6e`](https://github.com/frewsxcv/rust-crates-index/commit/bc9be6e2d979d07d98488a6e6823df2c5ee30fd0))
    - Made the Index more generic and implement Debug ([`9acedfa`](https://github.com/frewsxcv/rust-crates-index/commit/9acedfad56b8194a4a432cc7988e79f697e0be42))
    - Prepare for 0.10.0 release. ([`3726d81`](https://github.com/frewsxcv/rust-crates-index/commit/3726d81c1544b6ee39b500443259628e89bced51))
    - Add error/result types. ([`eeb8e6f`](https://github.com/frewsxcv/rust-crates-index/commit/eeb8e6f745f4e5258c83f006a06b48261f182bba))
    - Simplify git reset origin/master logic. ([`e4ed982`](https://github.com/frewsxcv/rust-crates-index/commit/e4ed9822462cd9f76b016d32e0248b044e636905))
    - Bump Travis CI base image. ([`02f87fa`](https://github.com/frewsxcv/rust-crates-index/commit/02f87face8613f0ac795f911f8feb89f9ce4403b))
</details>

## v0.9.0 (2017-05-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Replace rustc-serialize with serde, prepare for 0.9.0 release. ([`13795a5`](https://github.com/frewsxcv/rust-crates-index/commit/13795a57f23ff8ff6bb41593a399c7aa779565f7))
</details>

## v0.8.0 (2017-05-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add `name` method, prepare for 0.8.0 release. ([`b660054`](https://github.com/frewsxcv/rust-crates-index/commit/b66005471ed55d1c574b1a324f8a3041c5c1e7f9))
</details>

## v0.7.0 (2017-05-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.7.0. ([`360d4c8`](https://github.com/frewsxcv/rust-crates-index/commit/360d4c8a658fd581b42243893b8e5c4103d26a38))
    - Add update methods. ([`ccfcaca`](https://github.com/frewsxcv/rust-crates-index/commit/ccfcacae9ca5532ba424d16b5542feb8d713db1c))
    - Simplify import path. ([`1103b2b`](https://github.com/frewsxcv/rust-crates-index/commit/1103b2b892a9ebf70f7b81f60f8fcc0ad4f310b8))
    - Remove unnecessary check. ([`0a978b7`](https://github.com/frewsxcv/rust-crates-index/commit/0a978b703516b2b01c4309e5499068643f673d4f))
    - Harden `exists` checking. ([`b865922`](https://github.com/frewsxcv/rust-crates-index/commit/b86592244203cc5528ccbb4ad5e2a6b114ee6123))
    - Use docs.rs. ([`cae3dcf`](https://github.com/frewsxcv/rust-crates-index/commit/cae3dcfa74ef6653b727b5ec84ae324cd2b3effe))
    - Kill most of the Travis config. ([`3f912be`](https://github.com/frewsxcv/rust-crates-index/commit/3f912be86614437fafc2140794788c9917b8deb5))
    - Rustfmt. ([`b73834b`](https://github.com/frewsxcv/rust-crates-index/commit/b73834b1e3b7a1a01470c282a70e80cf71a8d6eb))
    - Rename `fetch` method to `retrieve`. ([`e16d39a`](https://github.com/frewsxcv/rust-crates-index/commit/e16d39ad9a25d8287b3a071bdf013869d4c5278f))
    - Add basic examples. ([`c9241db`](https://github.com/frewsxcv/rust-crates-index/commit/c9241db337267db9a665ec86353ad3411fc48a09))
    - Update documentation link. ([`89cc81b`](https://github.com/frewsxcv/rust-crates-index/commit/89cc81be2f54ba323ff3f582c0bc77f20849e91a))
</details>

## v0.6.0 (2017-01-31)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare for 0.6.0 release. ([`86ed7c7`](https://github.com/frewsxcv/rust-crates-index/commit/86ed7c72c9e25d07a31b289593f83cc4fa63f7e7))
    - Cargo fmt. ([`798c0a4`](https://github.com/frewsxcv/rust-crates-index/commit/798c0a490021737607ad6528d39f4badcd300ab4))
    - Updated `PathBuf` types to `AsRef<Path>` ([`d09acfd`](https://github.com/frewsxcv/rust-crates-index/commit/d09acfdbccaed5c8128a35a8a74c3d8c9b4428b5))
    - Merge pull request #14 from icefoxen/clone-rename ([`397c0de`](https://github.com/frewsxcv/rust-crates-index/commit/397c0de67a2a76bbda81701d4f5d55824b4a6a1a))
    - Rename `clone()` to `fetch()`. ([`e9076b4`](https://github.com/frewsxcv/rust-crates-index/commit/e9076b41b5861c11b1ec841fa9096792ddaee53c))
    - No need to take borrowed owned PathBuf. ([`2c0456c`](https://github.com/frewsxcv/rust-crates-index/commit/2c0456cf9ef2e4bdee4ca42a0775a30f207e5202))
</details>

## v0.5.1 (2016-12-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version. ([`247126e`](https://github.com/frewsxcv/rust-crates-index/commit/247126e7c747acc9e2782109bc420851224d3f75))
    - Bump 'git2' dependency to 0.6. ([`a30a687`](https://github.com/frewsxcv/rust-crates-index/commit/a30a6877bb47fc12f4f57da145994829ff4edb66))
</details>

## v0.5.0 (2016-11-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Some small fixes ([`e5a66e2`](https://github.com/frewsxcv/rust-crates-index/commit/e5a66e2b0b46d695b62459ebecf14de3e5602cd6))
    - Remove 1.0 line ([`36e669c`](https://github.com/frewsxcv/rust-crates-index/commit/36e669cc0132c2d5cc757b2c54e8e963356f787f))
    - Bump version. ([`9c25368`](https://github.com/frewsxcv/rust-crates-index/commit/9c25368aa7ed3f3f95326a1712dec65bfac67314))
    - Add _test to gitignore ([`3ff4943`](https://github.com/frewsxcv/rust-crates-index/commit/3ff49431d24b175c969691ed0a82d4f379556588))
    - Bump version of git2 ([`1928ab2`](https://github.com/frewsxcv/rust-crates-index/commit/1928ab2997f662c51d2588c2d2cef9d39ec90933))
</details>

## v0.4.0 (2015-11-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #9 from steveklabnik/master ([`16ac43c`](https://github.com/frewsxcv/rust-crates-index/commit/16ac43c9422c7ff01ff8b63061dae018728e45b6))
    - Update version to 0.4.0 ([`99f4b19`](https://github.com/frewsxcv/rust-crates-index/commit/99f4b19bb3f3fa17ca04d51a53bef2f978f66d11))
    - Update git2 dependency ([`f707f24`](https://github.com/frewsxcv/rust-crates-index/commit/f707f24b23adc5e1fff9a1514a18d4dd7c1a6af6))
    - Merge pull request #7 from natemara/patch-1 ([`7b515a7`](https://github.com/frewsxcv/rust-crates-index/commit/7b515a7ba3993fd0aa9aeab974ba5104ff9de6ba))
    - Add documentation link to Cargo.toml ([`c96b138`](https://github.com/frewsxcv/rust-crates-index/commit/c96b138e8eb49b30a0b7f51cf1fc26bd6f39baca))
    - Add helper method for getting earliest version ([`e7681c5`](https://github.com/frewsxcv/rust-crates-index/commit/e7681c5640c71985651a6bc2ed12e37ee21034e9))
    - Very basic method documentation ([`2825c7e`](https://github.com/frewsxcv/rust-crates-index/commit/2825c7ec8d5fa7881692e1114bce700a3edccc70))
    - Skip over invalid filenames in index instead of crashing ([`f0ba02b`](https://github.com/frewsxcv/rust-crates-index/commit/f0ba02bf963f9bfc90d87a846adbeee25f2076fe))
</details>

## v0.3.0 (2015-09-20)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.3.0 ([`0354988`](https://github.com/frewsxcv/rust-crates-index/commit/0354988fae742d3fd4b7bda801be2522355604a4))
    - Prevent construction of struct ([`9af9c32`](https://github.com/frewsxcv/rust-crates-index/commit/9af9c327a7c0cf1126e3c4e8815c5635aac18dc9))
</details>

## v0.2.2 (2015-09-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump to 0.2.2. ([`5a44c7b`](https://github.com/frewsxcv/rust-crates-index/commit/5a44c7bd7da9538f729c893886b183829e73308a))
    - Publicize Crate::versions vec ([`5a61666`](https://github.com/frewsxcv/rust-crates-index/commit/5a61666b260e28f458261d86e8770afda1a84372))
</details>

## v0.2.1 (2015-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version ([`9b4b2c5`](https://github.com/frewsxcv/rust-crates-index/commit/9b4b2c55e4fe4ef7cfcbbf385a72d5a01bdf68e2))
    - Dependency/Version should derive Clone ([`1231a5f`](https://github.com/frewsxcv/rust-crates-index/commit/1231a5fd5ce9fe0deae5f272cda898c3dbf363de))
</details>

## v0.2.0 (2015-09-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.2.0 ([`647d694`](https://github.com/frewsxcv/rust-crates-index/commit/647d694517d5cb9ff35cac5ef547fa0ed4510aa9))
    - Add some documentation ([`f9cebc3`](https://github.com/frewsxcv/rust-crates-index/commit/f9cebc3b655fcd9312dd2a68f9956f9410292b92))
    - Remove no longer necessary dependency_map method ([`582e8a1`](https://github.com/frewsxcv/rust-crates-index/commit/582e8a1b6270e167de3ad67df0e6a86955976f45))
    - Add link to documentation ([`a6cb77d`](https://github.com/frewsxcv/rust-crates-index/commit/a6cb77d69b72579e8c0046d73fd0970cd7288579))
    - Merge pull request #2 from frewsxcv/travis ([`1de087b`](https://github.com/frewsxcv/rust-crates-index/commit/1de087bbf9d131241a1ab35743d0282be0af4566))
    - Init travis config ([`8cc6d22`](https://github.com/frewsxcv/rust-crates-index/commit/8cc6d2280730dc54d9a079c9601c16bf5ae42d6c))
    - Extend the extremely lame test case to an extremely lamer test case ([`4bf5c5c`](https://github.com/frewsxcv/rust-crates-index/commit/4bf5c5c9eb8f5fc501a8c7a18c9d4b87c670917e))
    - Fix off-by-one error with latest_version ([`b9f8e02`](https://github.com/frewsxcv/rust-crates-index/commit/b9f8e02c7bccc5b6dbab870329ff11f40396a60a))
    - Simpler constructor name ([`46472b4`](https://github.com/frewsxcv/rust-crates-index/commit/46472b46c963f74d91fc44faabf23438ea6bee16))
    - Fewer Vecs and more Iterators ([`5c67700`](https://github.com/frewsxcv/rust-crates-index/commit/5c677007a9d720698ea373ca4b0ada8c970926e9))
    - Consistent naming ([`089c6e4`](https://github.com/frewsxcv/rust-crates-index/commit/089c6e43c8c1b1a92b736e80ee92571e4247c107))
    - Rename a few structs and fields ([`18254a8`](https://github.com/frewsxcv/rust-crates-index/commit/18254a83b8c3308c79f7425ea4e7265e80b82714))
    - Fix borrow issue ([`0117424`](https://github.com/frewsxcv/rust-crates-index/commit/0117424358cccecf101516e702ecb07dc71104f1))
    - Add/use CrateIndex::crates method, add Crate::latest_version method ([`72cd05c`](https://github.com/frewsxcv/rust-crates-index/commit/72cd05c3d6c45543c10baf6291834b601d4e61b1))
    - Add Crate struct representing all crate versions ([`c46159b`](https://github.com/frewsxcv/rust-crates-index/commit/c46159bd2118db1cd581daf16c5b8e564635cb10))
    - Add comment clarifying logic ([`730535a`](https://github.com/frewsxcv/rust-crates-index/commit/730535a5fd823837f0cc13f2680a5303b7a86a10))
    - Add extremely basic, virtually dummy test ([`1156273`](https://github.com/frewsxcv/rust-crates-index/commit/11562739ce48c52515351b3717f29b518b03cc83))
    - Remove redundant suffix on CratesIndex::clone_index ([`d73f4c3`](https://github.com/frewsxcv/rust-crates-index/commit/d73f4c3a50605d68354b086201c28ca425596285))
    - Return git Error in Result when cloning ([`2cfce52`](https://github.com/frewsxcv/rust-crates-index/commit/2cfce52fc7306aa8a721453143fa2cb134f7b136))
    - Add crates.io badge ([`fb5a78b`](https://github.com/frewsxcv/rust-crates-index/commit/fb5a78bbf0ab8222c30915374f158c968b9024b2))
    - Remove redundant word ([`d62130b`](https://github.com/frewsxcv/rust-crates-index/commit/d62130b52891a79016def44ee2ce7fd40974d3c3))
</details>

## v0.1.2 (2015-05-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version ([`7df1bb6`](https://github.com/frewsxcv/rust-crates-index/commit/7df1bb636318c2bf5f44b30a84dd237efdd4e01f))
    - Sort names before deduplicating ([`39d6d41`](https://github.com/frewsxcv/rust-crates-index/commit/39d6d41b5e6791e2562c5979e611bf8532db9c0b))
</details>

## v0.1.1 (2015-05-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version ([`b45d09b`](https://github.com/frewsxcv/rust-crates-index/commit/b45d09bde5bb9377dfcee484874d7112b463923e))
    - Deduplicate dependency crate names for map ([`e265a15`](https://github.com/frewsxcv/rust-crates-index/commit/e265a15672fa8c2adba1aa235710b83a6672f9cb))
</details>

## v0.1.0 (2015-05-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Specify license in cargo.toml ([`ce61e9a`](https://github.com/frewsxcv/rust-crates-index/commit/ce61e9a938e585e6a8d658d04f4bf4c0f8e819ad))
    - Swap invalid keyword with a valid one ([`5951e1d`](https://github.com/frewsxcv/rust-crates-index/commit/5951e1d8dab599f2d9f2aa9cb30ea25b7126086a))
    - Add link to repository ([`b22aa2a`](https://github.com/frewsxcv/rust-crates-index/commit/b22aa2a986b4c6326ae54bf21f8264f81be93770))
    - Initial checkin of code ([`3aeb4f5`](https://github.com/frewsxcv/rust-crates-index/commit/3aeb4f58a6de3d66f1236c3962873add31971d85))
</details>

