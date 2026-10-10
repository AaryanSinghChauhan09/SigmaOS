#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let filename = if s.is_empty() { "test.deb" } else { s };
        let _fmt = sigmaos::package::universal::PackageFormat::from_filename(filename);
        let mapper = sigmaos::sigpkg::universal_adapter::UniversalDependencyMapper::new();
        let _canonical = mapper.to_canonical_name(filename);
    }
});
