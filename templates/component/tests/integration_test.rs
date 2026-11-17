//! Integration tests for __COMPONENT_NAME__.

use __COMPONENT_CRATE_LIB__::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
