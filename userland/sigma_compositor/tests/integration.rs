use sigma_compositor::{ZenithCompositor, WindowManager, SigmaDesktop};

#[test]
fn test_surface_creation_and_destruction() {
    let mut comp = ZenithCompositor::new();
    
    let s1 = comp.create_surface(1, 800, 600);
    assert_eq!(s1.id, 1);
    assert_eq!(comp.layout.surfaces.len(), 1);

    let destroy_res = comp.destroy_surface(1);
    assert!(destroy_res.is_ok());
    assert_eq!(comp.layout.surfaces.len(), 0);

    let err_res = comp.destroy_surface(999);
    assert!(err_res.is_err());
}

#[test]
fn test_window_manager_tiling_layout() {
    let mut comp = ZenithCompositor::new();
    let _s1 = comp.create_surface(10, 800, 600);
    let _s2 = comp.create_surface(20, 1024, 768);

    let wm = WindowManager::new();
    let layouts = wm.arrange_surfaces(&comp.layout.surfaces, 1920, 1080);
    
    assert_eq!(layouts.len(), 2);
    // Layout 1: id=10, x=0, y=0, w=960, h=1080
    assert_eq!(layouts[0], (10, 0, 0, 960, 1080));
    // Layout 2: id=20, x=960, y=0, w=960, h=1080
    assert_eq!(layouts[1], (20, 960, 0, 960, 1080));
}

#[test]
fn test_sigma_desktop_configuration() {
    let mut desktop = SigmaDesktop::new();
    assert_eq!(desktop.panel.position, "Top");
    
    desktop.set_background("file:///custom.png");
    assert_eq!(desktop.background_uri, "file:///custom.png");
}
