//! false - Return false
//!
//! Exit with status 1, indicating failure.

use std::process;

fn main() {
    process::exit(1);
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_false_returns_one() {
        let result = Command::new("rustc")
            .arg("--edition=2021")
            .arg("src/userland/coreutils/false.rs")
            .arg("-o")
            .arg("/tmp/sigma-false")
            .output();
        
        assert!(result.is_ok());
        
        let result = Command::new("/tmp/sigma-false").status();
        assert!(result.is_ok());
        assert!(!result.unwrap().success());
    }
}
