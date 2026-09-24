/// Minimal SigmaOS Shell (Phase 1)
use std::string::String;

pub struct SigmaShell {
    cwd: String,
}

impl SigmaShell {
    pub fn new() -> Self {
        Self {
            cwd: String::from("/"),
        }
    }

    pub fn execute_command(&mut self, input: &str) -> String {
        let mut parts = input.trim().split_whitespace();
        let cmd = parts.next().unwrap_or("");

        match cmd {
            "echo" => {
                let rest: Vec<&str> = parts.collect();
                rest.join(" ")
            }
            "pwd" => self.cwd.clone(),
            "ls" => String::from(".\n..\nbin\ndev\netc\nusr"),
            "" => String::new(),
            _ => format!("{}: command not found", cmd),
        }
    }
}
