// SigmaOS Arch Wiki-Style Knowledge Base Engine
// Inspired by Arch Linux Wiki, Gentoo Wiki, and FreeBSD Handbook
// Provides offline-first, searchable, indexed system documentation and troubleshooting guides.

use std::collections::{BTreeMap, HashMap};
use std::string::String;
use std::vec::Vec;

/// Wiki article categories mirroring Arch Wiki namespaces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WikiCategory {
    KernelAndHardware,
    PackageManagement,
    NetworkingAndSecurity,
    DesktopAndUX,
    DevelopmentAndSdk,
    SystemAdministration,
    TroubleshootingAndFixes,
}

/// Structured wiki article record
#[derive(Debug, Clone)]
pub struct ArchWikiArticle {
    pub article_id: String,
    pub title: String,
    pub category: WikiCategory,
    pub summary: String,
    pub markdown_body: String,
    pub tags: Vec<String>,
    pub related_article_ids: Vec<String>,
    pub views_count: u64,
}

/// Offline-first Arch Wiki Knowledge Base Engine
#[derive(Debug, Clone)]
pub struct ArchWikiKnowledgeBaseEngine {
    pub articles: HashMap<String, ArchWikiArticle>,
    pub category_index: HashMap<WikiCategory, Vec<String>>,
    pub tag_index: HashMap<String, Vec<String>>,
}

impl ArchWikiKnowledgeBaseEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            articles: HashMap::new(),
            category_index: HashMap::new(),
            tag_index: HashMap::new(),
        };
        engine.seed_default_handbook_articles();
        engine
    }

    /// Seeds essential Arch Wiki & Gentoo Handbook style system articles
    pub fn seed_default_handbook_articles(&mut self) {
        self.register_article(ArchWikiArticle {
            article_id: "sigmaos-installation-guide".to_string(),
            title: "SigmaOS Installation & Partitioning Guide".to_string(),
            category: WikiCategory::SystemAdministration,
            summary: "Comprehensive guide for EFI/GPT, Btrfs subvolumes, and Limine bootloader setup.".to_string(),
            markdown_body: "# Installation Guide\n1. Partition with GPT\n2. Create Btrfs @root and @home\n3. Run sigma-install.".to_string(),
            tags: vec!["install".to_string(), "btrfs".to_string(), "limine".to_string()],
            related_article_ids: vec!["sigma-pkg-usage".to_string()],
            views_count: 1024,
        });

        self.register_article(ArchWikiArticle {
            article_id: "sigma-pkg-usage".to_string(),
            title: "SigmaPkg Package Manager Reference".to_string(),
            category: WikiCategory::PackageManagement,
            summary: "Guide to universal package formats (.deb, .rpm, .pkg.tar.zst, .apk, .moss, .hpkg).".to_string(),
            markdown_body: "# SigmaPkg Reference\nUse `sigma-pkg install <pkg>` for universal cross-distro installs.".to_string(),
            tags: vec!["package".to_string(), "sigpkg".to_string(), "universal".to_string()],
            related_article_ids: vec!["sigmaos-installation-guide".to_string()],
            views_count: 2048,
        });

        self.register_article(ArchWikiArticle {
            article_id: "zenith-desktop-tweaks".to_string(),
            title: "Zenith Desktop Customization & Performance".to_string(),
            category: WikiCategory::DesktopAndUX,
            summary: "Wayland layer-shell desklets, HiDPI scaling, and BORE scheduler latency tuning.".to_string(),
            markdown_body: "# Zenith Desktop\nConfigure Zenith Wayland compositor settings in ~/.config/zenith.toml.".to_string(),
            tags: vec!["desktop".to_string(), "zenith".to_string(), "wayland".to_string()],
            related_article_ids: vec![],
            views_count: 512,
        });
    }

    /// Registers a new knowledge base article and updates search indices
    pub fn register_article(&mut self, article: ArchWikiArticle) {
        let id = article.article_id.clone();
        let cat = article.category;
        for tag in &article.tags {
            self.tag_index
                .entry(tag.to_lowercase())
                .or_insert_with(Vec::new)
                .push(id.clone());
        }

        self.category_index
            .entry(cat)
            .or_insert_with(Vec::new)
            .push(id.clone());

        self.articles.insert(id, article);
    }

    /// Searches articles by title, summary, or tag query
    pub fn search(&self, query: &str) -> Vec<&ArchWikiArticle> {
        let q_lower = query.to_lowercase();
        let mut results: Vec<(&ArchWikiArticle, usize)> = Vec::new();

        for article in self.articles.values() {
            let mut score = 0;
            if article.title.to_lowercase().contains(&q_lower) {
                score += 10;
            }
            if article.summary.to_lowercase().contains(&q_lower) {
                score += 5;
            }
            if article.tags.iter().any(|t| t.to_lowercase() == q_lower) {
                score += 8;
            }
            if score > 0 {
                results.push((article, score));
            }
        }

        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.into_iter().map(|(art, _)| art).collect()
    }

    /// Retrieves articles filtered by category
    pub fn list_by_category(&self, category: WikiCategory) -> Vec<&ArchWikiArticle> {
        if let Some(ids) = self.category_index.get(&category) {
            ids.iter().filter_map(|id| self.articles.get(id)).collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for ArchWikiKnowledgeBaseEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_wiki_knowledgebase_search() {
        let engine = ArchWikiKnowledgeBaseEngine::new();
        let results = engine.search("package");
        assert!(!results.is_empty());
        assert_eq!(results[0].article_id, "sigma-pkg-usage");
    }

    #[test]
    fn test_arch_wiki_category_filtering() {
        let engine = ArchWikiKnowledgeBaseEngine::new();
        let desktop_articles = engine.list_by_category(WikiCategory::DesktopAndUX);
        assert_eq!(desktop_articles.len(), 1);
        assert_eq!(desktop_articles[0].article_id, "zenith-desktop-tweaks");
    }
}
