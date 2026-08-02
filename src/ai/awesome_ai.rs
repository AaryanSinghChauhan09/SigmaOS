#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Awesome-Code-AI Local Registry Module
//
// Formally implements compilable, production-ready Rust structures for the absorbed Awesome-Code-AI curated list:
// 1. AwesomeCodeAiRegistry (Local registry of AI development, completion, refactoring, and search tools)

pub struct AwesomeToolInfo {
    pub name: String,
    pub category: String,
    pub description: String,
    pub url: String,
}

pub struct AwesomeCodeAiRegistry {
    pub tools: Vec<AwesomeToolInfo>,
}

impl AwesomeCodeAiRegistry {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let tools = vec![
            AwesomeToolInfo {
                name: "Claude Code".to_string(),
                category: "Code assistants and search".to_string(),
                description: "Agentic coding tool that lives in your terminal.".to_string(),
                url: "https://www.anthropic.com/claude-code".to_string(),
            },
            AwesomeToolInfo {
                name: "Sourcegraph Cody".to_string(),
                category: "Code assistants and search".to_string(),
                description: "Uses your codebase as context for completions, edits, and search.".to_string(),
                url: "https://about.sourcegraph.com/cody".to_string(),
            },
            AwesomeToolInfo {
                name: "GitHub Copilot".to_string(),
                category: "Code completion tools".to_string(),
                description: "Pair programmer that offers autocomplete-style suggestions as you code.".to_string(),
                url: "https://github.com/features/copilot".to_string(),
            },
            AwesomeToolInfo {
                name: "Continue".to_string(),
                category: "Code completion tools".to_string(),
                description: "Open Source autopilot for VS Code and JetBrains that connects to any LLM.".to_string(),
                url: "https://continue.dev/".to_string(),
            },
            AwesomeToolInfo {
                name: "CodiumAI".to_string(),
                category: "Code completion tools".to_string(),
                description: "Analyzes code and generates meaningful tests.".to_string(),
                url: "https://www.codium.ai/".to_string(),
            },
            AwesomeToolInfo {
                name: "Aider".to_string(),
                category: "Code assistants and search".to_string(),
                description: "Pair programming tool in your terminal that works with local git repos.".to_string(),
                url: "https://aider.chat".to_string(),
            },
        ];

        Self { tools }
    }

    pub fn query_by_category(&self, category: &str) -> Vec<&AwesomeToolInfo> {
        self.tools
            .iter()
            .filter(|t| t.category.eq_ignore_ascii_case(category))
            .collect()
    }

    pub fn search_by_name(&self, query: &str) -> Vec<&AwesomeToolInfo> {
        self.tools
            .iter()
            .filter(|t| t.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}

impl Default for AwesomeCodeAiRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_awesome_ai_registry() {
        let registry = AwesomeCodeAiRegistry::new();
        assert!(registry.tools.len() >= 6);

        let completion_tools = registry.query_by_category("Code completion tools");
        assert_eq!(completion_tools.len(), 3);

        let results = registry.search_by_name("Cody");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Sourcegraph Cody");
    }
}
