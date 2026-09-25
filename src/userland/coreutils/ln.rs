//! ln - make links between files
//! POSIX, GNU & BSD compatible ln implementation

use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;

/// Options for ln execution
#[derive(Debug, Clone, Copy, Default)]
pub struct LnOptions {
    pub symbolic: bool,
    pub force: bool,
}

/// Create hard link or symbolic link
pub fn run(target: &str, link_name: &str, opts: LnOptions) -> Result<(), String> {
    let target_path = Path::new(target);
    let link_path = Path::new(link_name);

    if opts.force && link_path.exists() {
        let _ = fs::remove_file(link_path);
    }

    if opts.symbolic {
        unix_fs::symlink(target_path, link_path)
            .map_err(|e| format!("ln: failed to create symlink '{}': {}", link_name, e))
    } else {
        fs::hard_link(target_path, link_path)
            .map_err(|e| format!("ln: failed to create link '{}': {}", link_name, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ln_options() {
        let opts = LnOptions {
            symbolic: true,
            force: true,
        };
        assert!(opts.symbolic);
        assert!(opts.force);
    }
}
