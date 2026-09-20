//! true - Return true
//!
//! Exit with status 0, indicating success.

use std::process;

pub fn main() {
    process::exit(0);
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_true_returns_zero() {
        let result = Command::new("rustc")
            .arg("--edition=2021")
            .arg("src/userland/coreutils/true.rs")
            .arg("-o")
            .arg("/tmp/sigma-true")
            .output();
        
        assert!(result.is_ok());
        
        let result = Command::new("/tmp/sigma-true").status();
        assert!(result.is_ok());
        assert!(result.unwrap().success());
    }
}
