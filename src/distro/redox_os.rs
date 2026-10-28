// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Redox OS microkernel IPC bridge (scheme-based)
// Principles: MICROKERNEL, URL-SCHEME IPC, MODULARITY, SEPARATION OF MECHANISM & POLICY
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[derive(Debug, Clone)]
pub struct SchemeUrl {
    pub scheme: String,
    pub path: String,
}
impl SchemeUrl {
    pub fn parse(url: &str) -> Result<Self, &'static str> {
        if let Some(pos) = url.find(':') {
            Ok(Self {
                scheme: url[..pos].into(),
                path: url[pos + 1..].trim_start_matches('/').into(),
            })
        } else {
            Err("invalid scheme URL")
        }
    }
    pub fn as_string(&self) -> String {
        alloc::format!("{}:{}", self.scheme, self.path)
    }
}
pub trait RedoxScheme {
    fn open(&mut self, path: &str, flags: u32) -> i64;
    fn read(&mut self, id: i64, buf: &mut [u8]) -> i64;
    fn write(&mut self, id: i64, buf: &[u8]) -> i64;
    fn seek(&mut self, id: i64, pos: i64, whence: u8) -> i64;
    fn close(&mut self, id: i64) -> i64;
}
#[derive(Debug)]
pub struct MemScheme {
    pub data: BTreeMap<i64, Vec<u8>>,
    pub next_id: i64,
}
impl MemScheme {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            next_id: 1,
        }
    }
}
impl RedoxScheme for MemScheme {
    fn open(&mut self, _p: &str, _f: u32) -> i64 {
        let id = self.next_id;
        self.next_id += 1;
        self.data.insert(id, Vec::new());
        id
    }
    fn read(&mut self, id: i64, buf: &mut [u8]) -> i64 {
        if let Some(d) = self.data.get(&id) {
            let n = buf.len().min(d.len());
            buf[..n].copy_from_slice(&d[..n]);
            n as i64
        } else {
            -1
        }
    }
    fn write(&mut self, id: i64, buf: &[u8]) -> i64 {
        if let Some(d) = self.data.get_mut(&id) {
            d.extend_from_slice(buf);
            buf.len() as i64
        } else {
            -1
        }
    }
    fn seek(&mut self, _id: i64, pos: i64, _w: u8) -> i64 {
        pos
    }
    fn close(&mut self, id: i64) -> i64 {
        self.data.remove(&id);
        0
    }
}
pub struct IonShell {
    pub vars: BTreeMap<String, String>,
}
impl IonShell {
    pub fn new() -> Self {
        Self {
            vars: BTreeMap::new(),
        }
    }
    pub fn set_var(&mut self, k: &str, v: &str) {
        self.vars.insert(k.into(), v.into());
    }
    pub fn expand_variables(&self, s: &str) -> String {
        let mut out = s.to_string();
        for (k, v) in &self.vars {
            out = out.replace(&alloc::format!("${}", k), v);
        }
        out
    }
    pub fn parse_command(&self, cmd: &str) -> Vec<String> {
        cmd.split_whitespace()
            .map(|s| self.expand_variables(s))
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scheme_url_parse() {
        let u = SchemeUrl::parse("file:/home/user").unwrap();
        assert_eq!(u.scheme, "file");
        assert_eq!(u.path, "home/user");
    }
    #[test]
    fn test_mem_scheme() {
        let mut s = MemScheme::new();
        let id = s.open("test", 0);
        s.write(id, b"hello");
        let mut buf = [0u8; 5];
        s.read(id, &mut buf);
        assert_eq!(&buf, b"hello");
    }
    #[test]
    fn test_ion_shell() {
        let mut sh = IonShell::new();
        sh.set_var("HOME", "/home/sigma");
        let s = sh.expand_variables("cd $HOME");
        assert_eq!(s, "cd /home/sigma");
    }
}
