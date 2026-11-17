// Integration test for SigmaOS-systemd-random-seed
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use systemd_random_seed as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
