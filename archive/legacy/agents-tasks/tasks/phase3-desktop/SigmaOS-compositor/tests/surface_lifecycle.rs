//! Integration tests — surface lifecycle.

use sigmaos_compositor::{init_compositor, SurfaceManager};

#[test]
fn surfaces_start_empty() {
    let server = init_compositor().expect("compositor init failed");
    assert!(server.surface_manager.surfaces().is_empty());
}

#[test]
fn surface_register_and_query() {
    let mut sm = SurfaceManager::default();
    sm.register(10);
    sm.register(20);
    assert!(sm.surfaces().contains(&10));
    assert!(sm.surfaces().contains(&20));
}

#[test]
fn surface_unregister_removes_only_target() {
    let mut sm = SurfaceManager::default();
    for id in [1u64, 2, 3] {
        sm.register(id);
    }
    sm.unregister(2);
    assert!(!sm.surfaces().contains(&2));
    assert_eq!(sm.surfaces().len(), 2);
}

#[test]
fn unregister_nonexistent_is_noop() {
    let mut sm = SurfaceManager::default();
    sm.register(5);
    sm.unregister(99); // should not panic or change count
    assert_eq!(sm.surfaces().len(), 1);
}
