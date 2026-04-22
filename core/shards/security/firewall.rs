#![no_std]
/// shards/security/firewall.rs — Sovereign Neural Firewall
/// Character-level silicon protection.

pub struct NeuralFirewall;

impl NeuralFirewall {
    pub fn validate_request(req: &str) -> bool {
        // Loophole protection: Block known attack vectors at the silicon level
        let forbidden = ["../", "/etc/", "system32", "cmd.exe", "powershell"];
        for pattern in forbidden {
            if Self::manual_contains(req, pattern) { return false; }
        }
        true
    }

    fn manual_contains(s: &str, p: &str) -> bool {
        if p.is_empty() { return true; }
        let sb = s.as_bytes();
        let pb = p.as_bytes();
        if pb.len() > sb.len() { return false; }
        for i in 0..=(sb.len() - pb.len()) {
            if &sb[i..i + pb.len()] == pb { return true; }
        }
        false
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
