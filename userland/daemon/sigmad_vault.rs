// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/daemon/sigmad_vault.rs — sigmad-vault: Secrets Manager Daemon
// Language: Rust (std) — OOP via VaultDaemon + IPC server

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};

// ── IPC Protocol (Unix domain socket, length-prefixed JSON) ──────────────────

const SOCKET_PATH: &str = "/run/sigma/vault.sock";
const MAX_MSG: usize = 65536;

#[derive(Debug)]
enum VaultRequest {
    Get { key: String },
    Set { key: String, value: Vec<u8> },
    Delete { key: String },
    List,
    Lock,
    Unlock { passphrase: String },
}

#[derive(Debug)]
enum VaultResponse {
    Value(Vec<u8>),
    Keys(Vec<String>),
    Ok,
    Error(String),
    Locked,
}

fn encode_response(r: &VaultResponse) -> Vec<u8> {
    match r {
        VaultResponse::Value(v)  => { let mut out = vec![b'V']; out.extend_from_slice(v); out }
        VaultResponse::Keys(ks)  => { let s = ks.join("\n"); let mut out = vec![b'K']; out.extend_from_slice(s.as_bytes()); out }
        VaultResponse::Ok        => b"O".to_vec(),
        VaultResponse::Error(e)  => { let mut out = vec![b'E']; out.extend_from_slice(e.as_bytes()); out }
        VaultResponse::Locked    => b"L".to_vec(),
    }
}

fn decode_request(data: &[u8]) -> Option<VaultRequest> {
    if data.is_empty() { return None; }
    match data[0] {
        b'G' => Some(VaultRequest::Get { key: String::from_utf8_lossy(&data[1..]).to_string() }),
        b'S' => {
            let sep = data[1..].iter().position(|&b| b == b'\0')?;
            let key = String::from_utf8_lossy(&data[1..sep+1]).to_string();
            let val = data[sep+2..].to_vec();
            Some(VaultRequest::Set { key, value: val })
        }
        b'D' => Some(VaultRequest::Delete { key: String::from_utf8_lossy(&data[1..]).to_string() }),
        b'L' => Some(VaultRequest::List),
        b'X' => Some(VaultRequest::Lock),
        b'U' => Some(VaultRequest::Unlock { passphrase: String::from_utf8_lossy(&data[1..]).to_string() }),
        _ => None,
    }
}

// ── Vault Daemon ──────────────────────────────────────────────────────────────

pub struct VaultDaemon {
    store:      BTreeMap<String, Vec<u8>>,
    locked:     bool,
    store_dir:  PathBuf,
    master_key: [u8; 32],
}

impl VaultDaemon {
    pub fn new(dir: &str) -> Self {
        let _ = fs::create_dir_all(dir);
        Self {
            store: BTreeMap::new(), locked: true,
            store_dir: PathBuf::from(dir), master_key: [0u8; 32],
        }
    }

    fn xor_key(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
        data.iter().enumerate().map(|(i, b)| b ^ key[i % 32]).collect()
    }

    fn derive_key(passphrase: &str) -> [u8; 32] {
        let mut k = [0u8; 32];
        for (i, b) in passphrase.bytes().enumerate() { k[i % 32] ^= b.wrapping_mul(7).wrapping_add(i as u8); }
        for round in 0..10_000u32 {
            for i in 0..32 { k[i] = k[i].rotate_left(3).wrapping_add((round & 0xFF) as u8); }
        }
        k
    }

    pub fn unlock(&mut self, passphrase: &str) -> bool {
        self.master_key = Self::derive_key(passphrase);
        self.locked = false;
        self.load_all();
        true
    }

    pub fn lock(&mut self) {
        self.store.clear();
        self.master_key = [0u8; 32];
        self.locked = true;
    }

    fn load_all(&mut self) {
        if let Ok(entries) = fs::read_dir(&self.store_dir) {
            for e in entries.flatten() {
                let path = e.path();
                if path.extension().and_then(|x| x.to_str()) != Some("sec") { continue; }
                let key = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                if let Ok(enc) = fs::read(&path) {
                    let dec = Self::xor_key(&enc, &self.master_key);
                    self.store.insert(key, dec);
                }
            }
        }
    }

    fn persist(&self, key: &str, value: &[u8]) {
        let enc  = Self::xor_key(value, &self.master_key);
        let safe = key.chars().map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' }).collect::<String>();
        let _ = fs::write(self.store_dir.join(format!("{}.sec", safe)), enc);
    }

    fn handle(&mut self, req: VaultRequest) -> VaultResponse {
        if self.locked && !matches!(req, VaultRequest::Unlock{..}) {
            return VaultResponse::Locked;
        }
        match req {
            VaultRequest::Get { key }        => self.store.get(&key)
                .map(|v| VaultResponse::Value(v.clone()))
                .unwrap_or(VaultResponse::Error(format!("key '{}' not found", key))),
            VaultRequest::Set { key, value } => {
                self.persist(&key, &value);
                self.store.insert(key, value);
                VaultResponse::Ok
            }
            VaultRequest::Delete { key }     => {
                self.store.remove(&key);
                let safe = key.chars().map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' }).collect::<String>();
                let _ = fs::remove_file(self.store_dir.join(format!("{}.sec", safe)));
                VaultResponse::Ok
            }
            VaultRequest::List               => VaultResponse::Keys(self.store.keys().cloned().collect()),
            VaultRequest::Lock               => { self.lock(); VaultResponse::Ok }
            VaultRequest::Unlock { passphrase } => {
                if self.unlock(&passphrase) { VaultResponse::Ok }
                else { VaultResponse::Error("unlock failed".to_owned()) }
            }
        }
    }

    fn handle_client(&mut self, mut stream: UnixStream) {
        let mut len_buf = [0u8; 4];
        if stream.read_exact(&mut len_buf).is_err() { return; }
        let len = u32::from_le_bytes(len_buf) as usize;
        if len > MAX_MSG { return; }
        let mut buf = vec![0u8; len];
        if stream.read_exact(&mut buf).is_err() { return; }
        if let Some(req) = decode_request(&buf) {
            let resp = self.handle(req);
            let enc  = encode_response(&resp);
            let rlen = (enc.len() as u32).to_le_bytes();
            let _ = stream.write_all(&rlen);
            let _ = stream.write_all(&enc);
        }
    }

    pub fn run(&mut self) {
        let socket_path = SOCKET_PATH;
        let _ = fs::remove_file(socket_path);
        let _ = fs::create_dir_all("/run/sigma");
        let listener = match UnixListener::bind(socket_path) {
            Ok(l) => l,
            Err(e) => { eprintln!("[sigmad-vault] bind failed: {}", e); return; }
        };
        eprintln!("[sigmad-vault] listening on {}", socket_path);
        for stream in listener.incoming() {
            match stream {
                Ok(s) => self.handle_client(s),
                Err(_) => break,
            }
        }
    }
}
