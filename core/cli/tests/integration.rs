/// tests/integration_test.rs — Native Rust integration tests for SigmaOS
/// Replaces high-level Python tests to reduce system dependencies.

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::fs;
    use std::path::Path;

    fn sigmactl() -> Command {
        let mut cmd = Command::new("cargo");
        cmd.args(["run", "-p", "sigma-cli", "--"]);
        cmd
    }

    #[test]
    fn test_wizard_and_config() {
        let root = "test_env_rust";
        if Path::new(root).exists() { fs::remove_dir_all(root).unwrap(); }
        fs::create_dir(root).unwrap();

        // Run wizard
        let status = Command::new("cargo")
            .args(["run", "-p", "sigma-cli", "--", "wizard"])
            .env("SIGMA_ROOT", root)
            .status()
            .expect("Failed to run wizard");
        
        assert!(status.success());
        assert!(Path::new(root).join("sigma_config.json").exists());
        assert!(Path::new(root).join("profiles").join("default.json").exists());

        // Test set/get config
        let set_status = Command::new("cargo")
            .args(["run", "-p", "sigma-cli", "--", "set", "test_key", "test_val"])
            .env("SIGMA_ROOT", root)
            .status()
            .unwrap();
        assert!(set_status.success());

        let output = Command::new("cargo")
            .args(["run", "-p", "sigma-cli", "--", "get", "test_key"])
            .env("SIGMA_ROOT", root)
            .output()
            .unwrap();
        let out_str = String::from_utf8_lossy(&output.stdout);
        assert!(out_str.contains("test_key = test_val"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn test_shard_scaffolding() {
        let root = "test_shard_env";
        if Path::new(root).exists() { fs::remove_dir_all(root).unwrap(); }
        fs::create_dir(root).unwrap();
        fs::create_dir(Path::new(root).join("shards")).unwrap();
        fs::create_dir_all(Path::new(root).join("kernel/suites")).unwrap();

        let status = Command::new("cargo")
            .args(["run", "-p", "sigma-cli", "--", "shard", "add", "net-shard"])
            .env("SIGMA_ROOT", root)
            .status()
            .unwrap();
        
        assert!(status.success());
        assert!(Path::new(root).join("shards/net-shard/Cargo.toml").exists());
        assert!(Path::new(root).join("kernel/suites/SXX_NET-SHARD/SXX_NET-SHARD_Register.c").exists());

        fs::remove_dir_all(root).unwrap();
    }
}
