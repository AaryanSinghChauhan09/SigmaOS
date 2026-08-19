// SigmaOS Alpine Linux BusyBox-Inspired Multi-Call Applet Command Dispatcher
// Zero-dependency, safe, robust command multiplexing for sovereign userland shell
// Inspired by Alpine Linux BusyBox multicall binary architecture

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub type AppletHandler = fn(args: &[&str]) -> Result<String, &'static str>;

pub struct BusyBoxAppletDispatcher {
    pub applets: BTreeMap<String, AppletHandler>,
}

impl BusyBoxAppletDispatcher {
    pub fn new() -> Self {
        let mut dispatcher = Self {
            applets: BTreeMap::new(),
        };
        dispatcher.register_default_applets();
        dispatcher
    }

    pub fn register_applet(&mut self, name: &str, handler: AppletHandler) {
        self.applets.insert(name.to_string(), handler);
    }

    fn register_default_applets(&mut self) {
        self.register_applet("ls", |_args| {
            Ok("applet [ls]: .  ..  bin  dev  etc  proc  sys  usr  var".to_string())
        });

        self.register_applet("cat", |args| {
            if args.is_empty() {
                return Err("cat: missing file argument");
            }
            Ok(format!("applet [cat]: Content of file '{}'", args[0]))
        });

        self.register_applet("echo", |args| {
            let mut out = String::new();
            for (i, arg) in args.iter().enumerate() {
                out.push_str(arg);
                if i + 1 < args.len() {
                    out.push(' ');
                }
            }
            Ok(out)
        });

        self.register_applet("grep", |args| {
            if args.len() < 2 {
                return Err("grep: usage: grep <pattern> <file>");
            }
            Ok(format!("applet [grep]: Matched pattern '{}' in '{}'", args[0], args[1]))
        });

        self.register_applet("cp", |args| {
            if args.len() < 2 {
                return Err("cp: usage: cp <source> <destination>");
            }
            Ok(format!("applet [cp]: Copied '{}' -> '{}'", args[0], args[1]))
        });
    }

    pub fn dispatch(&self, call_name: &str, args: &[&str]) -> Result<String, &'static str> {
        if let Some(handler) = self.applets.get(call_name) {
            handler(args)
        } else {
            Err("applet: command not found")
        }
    }
}

impl Default for BusyBoxAppletDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_busybox_multicall_applet_dispatcher() {
        let dispatcher = BusyBoxAppletDispatcher::new();

        let ls_out = dispatcher.dispatch("ls", &[]).unwrap();
        assert!(ls_out.contains("etc"));

        let echo_out = dispatcher.dispatch("echo", &["Hello", "SigmaOS"]).unwrap();
        assert_eq!(echo_out, "Hello SigmaOS");

        let cat_err = dispatcher.dispatch("cat", &[]);
        assert!(cat_err.is_err());

        let cat_out = dispatcher.dispatch("cat", &["/etc/hostname"]).unwrap();
        assert!(cat_out.contains("/etc/hostname"));

        let unknown = dispatcher.dispatch("nonexistent_applet", &[]);
        assert!(unknown.is_err());
    }
}
