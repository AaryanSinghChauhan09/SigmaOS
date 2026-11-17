//! __COMPONENT_NAME__ — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use __COMPONENT_CRATE_LIB__::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("__COMPONENT_NAME__: {e}");
        std::process::exit(1);
    }
    println!("__COMPONENT_NAME__: ok");
}
