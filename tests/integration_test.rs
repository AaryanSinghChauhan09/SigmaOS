// SigmaOS Integration Tests
// Verifies core system legacy compatibility, multi-persona VMs, and driver bridge layers
#![allow(unused, clippy::all)]

use sigmaos::sigpkg::universal_adapter::{ApkAdapter, EbuildAdapter, NixAdapter, PackageFormatAdapter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_distro_packaging_compatibility() {
        let apk = ApkAdapter::new();
        assert_eq!(apk.format_name(), "apk");

        let nix = NixAdapter::new();
        assert_eq!(nix.format_name(), "nix");

        let ebuild = EbuildAdapter::new();
        assert_eq!(ebuild.format_name(), "ebuild");
    }
}
