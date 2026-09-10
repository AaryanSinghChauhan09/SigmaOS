// Sovereign Tech News & Technology Media Redirection Engine
// Inspiring content aggregation, RSS/Atom feed parsing, and smart URL redirection
// across 28 top Linux & Tech publications for SigmaOS browser and desktop news widgets.
// 100% Safe Rust `#![no_std]` compliant with zero external dependencies.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TechPublicationCategory {
    LinuxAndOpenSource,
    HardwareAndGadgets,
    AiAndDataScience,
    WindowsAndCrossPlatform,
    EnterpriseAndCloud,
}

#[derive(Debug, Clone)]
pub struct TechPublicationEntry {
    pub name: String,
    pub domain: String,
    pub feed_url: String,
    pub redirect_canonical_url: String,
    pub category: TechPublicationCategory,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct NewsArticleItem {
    pub title: String,
    pub publication: String,
    pub url: String,
    pub summary: String,
    pub timestamp_epoch: u64,
}

pub struct SovereignTechNewsRedirectionEngine {
    pub publications: BTreeMap<String, TechPublicationEntry>,
    pub cached_articles: Vec<NewsArticleItem>,
}

impl SovereignTechNewsRedirectionEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            publications: BTreeMap::new(),
            cached_articles: Vec::new(),
        };
        engine.bootstrap_default_publications();
        engine
    }

    fn bootstrap_default_publications(&mut self) {
        let entries = vec![
            ("itsfoss", "ItsFOSS", "itsfoss.com", "https://itsfoss.com/feed/", "https://itsfoss.com", TechPublicationCategory::LinuxAndOpenSource, "Linux tutorials, distro news and open-source updates"),
            ("9to5linux", "9to5Linux", "9to5linux.com", "https://9to5linux.com/feed", "https://9to5linux.com", TechPublicationCategory::LinuxAndOpenSource, "Linux release news and desktop distro updates"),
            ("geekygadgets", "Geeky Gadgets", "geeky-gadgets.com", "https://www.geeky-gadgets.com/feed/", "https://www.geeky-gadgets.com", TechPublicationCategory::HardwareAndGadgets, "Gadget news, hardware reviews and tech innovations"),
            ("linuxdotcom", "Linux.com", "linux.com", "https://www.linux.com/feed/", "https://www.linux.com", TechPublicationCategory::LinuxAndOpenSource, "Official Linux news, developer guides and industry news"),
            ("kdnuggets", "KDnuggets", "kdnuggets.com", "https://www.kdnuggets.com/feed", "https://www.kdnuggets.com", TechPublicationCategory::AiAndDataScience, "AI, Machine Learning, Data Science & Analytics tutorials"),
            ("hwbusters", "HW Busters", "hwbusters.com", "https://hwbusters.com/feed/", "https://hwbusters.com", TechPublicationCategory::HardwareAndGadgets, "Power supply, GPU, CPU and hardware benchmarking analysis"),
            ("itdaily", "ITDaily", "itdaily.com", "https://www.itdaily.com/feed/", "https://www.itdaily.com", TechPublicationCategory::EnterpriseAndCloud, "Enterprise IT, cloud computing and infrastructure insights"),
            ("howtogeek", "How-To Geek", "howtogeek.com", "https://www.howtogeek.com/feed/", "https://www.howtogeek.com", TechPublicationCategory::WindowsAndCrossPlatform, "Tech explainer guides, software tips and OS tutorials"),
            ("linuxorg", "Linux.org", "linux.org", "https://www.linux.org/forums/-/index.rss", "https://www.linux.org", TechPublicationCategory::LinuxAndOpenSource, "Linux community discussions and distro forums"),
            ("infoworld", "InfoWorld", "infoworld.com", "https://www.infoworld.com/feed/", "https://www.infoworld.com", TechPublicationCategory::EnterpriseAndCloud, "Software development, architecture and enterprise tech"),
            ("linuxfoundation", "Linux Foundation", "linuxfoundation.org", "https://www.linuxfoundation.org/feed", "https://www.linuxfoundation.org", TechPublicationCategory::LinuxAndOpenSource, "Open source ecosystem governance and standards"),
            ("makeuseof", "MakeUseOf", "makeuseof.com", "https://www.makeuseof.com/feed/", "https://www.makeuseof.com", TechPublicationCategory::WindowsAndCrossPlatform, "Consumer tech guides, Linux tricks and software reviews"),
            ("pcworld", "PCWorld", "pcworld.com", "https://www.pcworld.com/feed/", "https://www.pcworld.com", TechPublicationCategory::HardwareAndGadgets, "PC hardware, GPU benchmarks and desktop software"),
            ("marktechpost", "MarkTechPost", "marktechpost.com", "https://www.marktechpost.com/feed/", "https://www.marktechpost.com", TechPublicationCategory::AiAndDataScience, "Artificial Intelligence research and LLM model breakthroughs"),
            ("windowslatest", "Windows Latest", "windowslatest.com", "https://www.windowslatest.com/feed/", "https://www.windowslatest.com", TechPublicationCategory::WindowsAndCrossPlatform, "Windows OS updates, features and platform news"),
            ("techspot", "TechSpot", "techspot.com", "https://www.techspot.com/backend/rss.xml", "https://www.techspot.com", TechPublicationCategory::HardwareAndGadgets, "Tech news, hardware reviews and gaming performance"),
            ("thenewstack", "The New Stack", "thenewstack.io", "https://thenewstack.io/feed/", "https://thenewstack.io", TechPublicationCategory::EnterpriseAndCloud, "Cloud-native, Kubernetes, microservices and devops"),
            ("techpowerup", "TechPowerUp", "techpowerup.com", "https://www.techpowerup.com/rss/news", "https://www.techpowerup.com", TechPublicationCategory::HardwareAndGadgets, "GPU databases, hardware news and firmware reviews"),
            ("windowscentral", "Windows Central", "windowscentral.com", "https://www.windowscentral.com/rss.xml", "https://www.windowscentral.com", TechPublicationCategory::WindowsAndCrossPlatform, "Windows ecosystem, laptops, surface and PC hardware"),
            ("phoronix", "Phoronix", "phoronix.com", "https://www.phoronix.com/phoronix-rss.php", "https://www.phoronix.com", TechPublicationCategory::LinuxAndOpenSource, "Linux hardware benchmarks, kernel patches and graphics drivers"),
            ("techcrunch", "TechCrunch", "techcrunch.com", "https://techcrunch.com/feed/", "https://techcrunch.com", TechPublicationCategory::EnterpriseAndCloud, "Tech startup news, venture capital and industry trends"),
            ("xdadevelopers", "XDA Developers", "xda-developers.com", "https://www.xda-developers.com/feed/", "https://www.xda-developers.com", TechPublicationCategory::WindowsAndCrossPlatform, "Android, mobile tech, custom ROMs and PC hardware"),
            ("zdnet", "ZDNET", "zdnet.com", "https://www.zdnet.com/news/rss.xml", "https://www.zdnet.com", TechPublicationCategory::EnterpriseAndCloud, "Enterprise technology news, security and business IT"),
            ("opensourceforu", "Open Source For You", "opensourceforu.com", "https://www.opensourceforu.com/feed/", "https://www.opensourceforu.com", TechPublicationCategory::LinuxAndOpenSource, "Open source development, Linux kernel articles and tools"),
            ("pcmag", "PCMag", "pcmag.com", "https://www.pcmag.com/rss.xml", "https://www.pcmag.com", TechPublicationCategory::HardwareAndGadgets, "Lab-tested hardware reviews and technology buyer guides"),
            ("linuxteck", "LinuxTeck", "linuxteck.com", "https://www.linuxteck.com/feed/", "https://www.linuxteck.com", TechPublicationCategory::LinuxAndOpenSource, "Linux sysadmin tutorials, DevOps and security guides"),
            ("appuals", "Appuals", "appuals.com", "https://appuals.com/feed/", "https://appuals.com", TechPublicationCategory::WindowsAndCrossPlatform, "Software troubleshooting, OS error fixes and guides"),
            ("distrowatch", "DistroWatch", "distrowatch.com", "https://distrowatch.com/news/dw.xml", "https://distrowatch.com", TechPublicationCategory::LinuxAndOpenSource, "Linux and BSD distribution release tracking and rankings"),
        ];

        for (id, name, domain, feed_url, canonical_url, category, desc) in entries {
            self.publications.insert(
                id.to_string(),
                TechPublicationEntry {
                    name: name.to_string(),
                    domain: domain.to_string(),
                    feed_url: feed_url.to_string(),
                    redirect_canonical_url: canonical_url.to_string(),
                    category,
                    description: desc.to_string(),
                },
            );
        }
    }

    pub fn redirect_url(&self, input_url_or_shortcut: &str) -> Option<String> {
        let trimmed = input_url_or_shortcut.trim();

        // 1. Direct publication shortcut lookup (e.g. "phoronix" -> "https://www.phoronix.com")
        if let Some(pub_entry) = self.publications.get(trimmed.to_lowercase().as_str()) {
            return Some(pub_entry.redirect_canonical_url.clone());
        }

        // 2. Domain matching lookup (e.g. "http://phoronix.com/something" -> "https://www.phoronix.com")
        for pub_entry in self.publications.values() {
            if trimmed.contains(&pub_entry.domain) {
                return Some(pub_entry.redirect_canonical_url.clone());
            }
        }

        None
    }

    pub fn get_publications_by_category(
        &self,
        category: TechPublicationCategory,
    ) -> Vec<TechPublicationEntry> {
        self.publications
            .values()
            .filter(|p| p.category == category)
            .cloned()
            .collect()
    }

    pub fn parse_mock_feed(&mut self, pub_id: &str, mock_xml: &str) -> usize {
        let pub_name = match self.publications.get(pub_id) {
            Some(p) => p.name.clone(),
            None => "Unknown Publication".to_string(),
        };

        let mut items_added = 0;
        // Simple XML tag scanner for <item>/<entry> titles
        for block in mock_xml.split("<item>") {
            if let Some(title_start) = block.find("<title>") {
                if let Some(title_end) = block[title_start..].find("</title>") {
                    let title = block[title_start + 7..title_start + title_end]
                        .trim()
                        .to_string();
                    if !title.is_empty() {
                        self.cached_articles.push(NewsArticleItem {
                            title: title.clone(),
                            publication: pub_name.clone(),
                            url: format!("https://sigma.os/news/{}", items_added + 1),
                            summary: format!("Aggregated tech news: {}", title),
                            timestamp_epoch: 1700000000 + items_added as u64,
                        });
                        items_added += 1;
                    }
                }
            }
        }

        items_added
    }

    pub fn search_articles(&self, query: &str) -> Vec<NewsArticleItem> {
        let q = query.to_lowercase();
        self.cached_articles
            .iter()
            .filter(|a| a.title.to_lowercase().contains(&q) || a.summary.to_lowercase().contains(&q))
            .cloned()
            .collect()
    }
}

impl Default for SovereignTechNewsRedirectionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tech_publication_redirects() {
        let engine = SovereignTechNewsRedirectionEngine::new();

        // Direct key shortcuts
        assert_eq!(
            engine.redirect_url("phoronix"),
            Some("https://www.phoronix.com".to_string())
        );
        assert_eq!(
            engine.redirect_url("itsfoss"),
            Some("https://itsfoss.com".to_string())
        );
        assert_eq!(
            engine.redirect_url("distrowatch"),
            Some("https://distrowatch.com".to_string())
        );

        // Domain matching
        assert_eq!(
            engine.redirect_url("https://www.marktechpost.com/2026/08/ai-paper"),
            Some("https://www.marktechpost.com".to_string())
        );
    }

    #[test]
    fn test_category_filtering() {
        let engine = SovereignTechNewsRedirectionEngine::new();

        let linux_pubs = engine.get_publications_by_category(TechPublicationCategory::LinuxAndOpenSource);
        assert!(linux_pubs.iter().any(|p| p.name == "ItsFOSS"));
        assert!(linux_pubs.iter().any(|p| p.name == "DistroWatch"));
        assert!(linux_pubs.iter().any(|p| p.name == "Phoronix"));

        let ai_pubs = engine.get_publications_by_category(TechPublicationCategory::AiAndDataScience);
        assert!(ai_pubs.iter().any(|p| p.name == "KDnuggets"));
        assert!(ai_pubs.iter().any(|p| p.name == "MarkTechPost"));
    }

    #[test]
    fn test_feed_parser_and_search() {
        let mut engine = SovereignTechNewsRedirectionEngine::new();
        let mock_rss = "<channel><item><title>Linux Kernel 6.12 Benchmarks</title></item><item><title>NVIDIA Drivers Updated</title></item></channel>";

        let count = engine.parse_mock_feed("phoronix", mock_rss);
        assert_eq!(count, 2);

        let search_res = engine.search_articles("Kernel");
        assert_eq!(search_res.len(), 1);
        assert_eq!(search_res[0].title, "Linux Kernel 6.12 Benchmarks");
    }
}
