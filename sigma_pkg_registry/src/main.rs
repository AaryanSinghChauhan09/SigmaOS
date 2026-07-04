use sigma_pkg_registry::SigPkgManifest;
use std::fs;
use toml;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: sigpkg [install|uninstall|create]");
        return;
    }

    match args[1].as_str() {
        "create" => {
            let manifest = SigPkgManifest::new("hello-sigma".to_string(), "0.1.0".to_string());
            let toml_str = toml::to_string(&manifest).unwrap();
            fs::write("sigpkg.toml", toml_str).unwrap();
            println!("Created sigpkg.toml");
        }
        "install" => {
            println!("Installing package...");
        }
        "uninstall" => {
            println!("Uninstalling package...");
        }
        _ => println!("Unknown command"),
    }
}
