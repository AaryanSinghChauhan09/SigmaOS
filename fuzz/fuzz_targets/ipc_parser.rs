#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }

    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let cmd = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    let payload = &data[8..];

    // Fuzz IPC payload parsing invariants
    if magic == 0x5349474D { // 'SIGM'
        match cmd {
            1 => {
                // Fuzz CapabilityToken handle decoding
                if payload.len() >= 16 {
                    let token_id = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                    let perms = u64::from_le_bytes(payload[8..16].try_into().unwrap());
                    let _ = (token_id, perms);
                }
            }
            2 => {
                // Fuzz IPC message header bounds
                if payload.len() <= 1024 {
                    let mut buffer = [0u8; 1024];
                    let copy_len = payload.len().min(buffer.len());
                    buffer[..copy_len].copy_from_slice(&payload[..copy_len]);
                }
            }
            _ => {}
        }
    }
});
