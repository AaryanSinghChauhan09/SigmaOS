/// SigmaOS Command Palette Launcher (Phase 4)
/// Inspired by Omarchy's keyboard-first command palette UX.

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct LauncherEntry {
    pub name: String,
    pub exec_path: String,
    pub icon: String,
    pub category: String,
    pub keywords: Vec<String>,
}

pub struct CommandPalette {
    pub entries: Vec<LauncherEntry>,
}

impl CommandPalette {
    pub fn new() -> Self { Self { entries: Vec::new() } }

    pub fn register(&mut self, entry: LauncherEntry) { self.entries.push(entry); }

    pub fn fuzzy_search(&self, query: &str) -> Vec<&LauncherEntry> {
        let q = query.to_lowercase();
        let mut results: Vec<(&LauncherEntry, usize)> = self.entries.iter().filter_map(|e| {
            let name_lower = e.name.to_lowercase();
            if name_lower.contains(&q) { Some((e, name_lower.find(&q).unwrap_or(usize::MAX))) }
            else if e.keywords.iter().any(|k| k.to_lowercase().contains(&q)) { Some((e, 1000)) }
            else { None }
        }).collect();
        results.sort_by_key(|(_, score)| *score);
        results.into_iter().map(|(e, _)| e).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_search() {
        let mut palette = CommandPalette::new();
        palette.register(LauncherEntry { name: "Terminal".into(), exec_path: "/usr/bin/sigma-term".into(), icon: "terminal".into(), category: "System".into(), keywords: vec!["console".into(), "shell".into()] });
        palette.register(LauncherEntry { name: "File Manager".into(), exec_path: "/usr/bin/sigma-files".into(), icon: "files".into(), category: "System".into(), keywords: vec!["explorer".into()] });
        let results = palette.fuzzy_search("term");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Terminal");
        let results2 = palette.fuzzy_search("shell");
        assert_eq!(results2.len(), 1);
    }
}
