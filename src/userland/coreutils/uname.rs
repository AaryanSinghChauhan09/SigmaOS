//! uname - print system information
//! POSIX, GNU & BSD compatible uname implementation

/// Options for uname execution
#[derive(Debug, Clone, Copy, Default)]
pub struct UnameOptions {
    pub kernel_name: bool,
    pub nodename: bool,
    pub kernel_release: bool,
    pub kernel_version: bool,
    pub machine: bool,
    pub operating_system: bool,
}

impl UnameOptions {
    pub fn all() -> Self {
        Self {
            kernel_name: true,
            nodename: true,
            kernel_release: true,
            kernel_version: true,
            machine: true,
            operating_system: true,
        }
    }
}

/// System information payload
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
    pub os_name: String,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            sysname: String::from("SigmaOS"),
            nodename: String::from("sigma-node"),
            release: String::from("1.0.0-sovereign"),
            version: String::from("SigmaOS Kernel 1.0.0 #1 SMP Rust"),
            machine: String::from("x86_64"),
            os_name: String::from("SigmaOS/Linux/BSD-Hybrid"),
        }
    }
}

/// Uname execution entrypoint
pub fn run(opts: UnameOptions, info: Option<SystemInfo>) -> Result<String, String> {
    let sys = info.unwrap_or_default();
    let mut parts = Vec::new();

    let show_all = !opts.kernel_name
        && !opts.nodename
        && !opts.kernel_release
        && !opts.kernel_version
        && !opts.machine
        && !opts.operating_system;

    if opts.kernel_name || show_all {
        parts.push(sys.sysname);
    }
    if opts.nodename {
        parts.push(sys.nodename);
    }
    if opts.kernel_release {
        parts.push(sys.release);
    }
    if opts.kernel_version {
        parts.push(sys.version);
    }
    if opts.machine {
        parts.push(sys.machine);
    }
    if opts.operating_system {
        parts.push(sys.os_name);
    }

    Ok(parts.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uname_default() {
        let opts = UnameOptions::default();
        let res = run(opts, None).unwrap();
        assert_eq!(res, "SigmaOS");
    }

    #[test]
    fn test_uname_all() {
        let opts = UnameOptions::all();
        let res = run(opts, None).unwrap();
        assert!(res.contains("SigmaOS"));
        assert!(res.contains("x86_64"));
    }
}
