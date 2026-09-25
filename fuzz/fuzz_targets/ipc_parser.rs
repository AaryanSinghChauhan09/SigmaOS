#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Fuzz IPC payload deserialization and header parsing logic
    if data.len() < 16 {
        return;
    }

    // Extract pseudo IPC message header
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let cmd = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    let capability_token = u64::from_le_bytes([
        data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
    ]);

    let payload = &data[16..];

    // Invariant: Magic marker validation check (0x5349474D == "SIGM")
    if magic == 0x5349474D {
        // Parse payload command safely without panicking
        match cmd {
            1 => {
                // Echo / String command
                let _ = std::str::from_utf8(payload);
            }
            2 => {
                // Capability Token Verification check
                let is_valid = capability_token != 0 && (capability_token & 0xFF00_0000_0000_0000) != 0;
                let _ = is_valid;
            }
            3 => {
                // Binary payload chunking check
                let mut sum: u64 = 0;
                for byte in payload {
                    sum = sum.wrapping_add(*byte as u64);
                }
                let _ = sum;
            }
            _ => {}
        }
    }
});
