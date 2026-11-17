//! Integration tests for SigmaOS-systemd-random-seed.

use sigmaos_systemd_random_seed_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
