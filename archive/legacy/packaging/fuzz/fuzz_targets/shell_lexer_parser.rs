#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = core::str::from_utf8(data) {
        let mut engine = sigmaos::shell::InteractiveShellEngine::new();
        let _ = engine.handle_key_input(input);
        let _ = engine.is_incomplete_input(input);
        let _ = sigmaos::shell::ShellDiagnosticsEngine::format_parse_error(1, 1, input, Some(input));
    }
});
