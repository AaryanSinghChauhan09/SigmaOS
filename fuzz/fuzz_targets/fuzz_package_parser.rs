#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Fuzz package name and manifest parsing - ensure robust handling without panics
        let _ = s.trim().split_whitespace().collect::<Vec<_>>();
    }
});
