use std::collections::HashMap;

/// Standardized Man page entry.
#[derive(Debug, Clone)]
pub struct ManPage {
    pub name: String,
    pub section: u32,
    pub synopsis: String,
    pub description: String,
    pub options: HashMap<String, String>,
}

impl ManPage {
    pub fn new(name: &str, section: u32, synopsis: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            section,
            synopsis: synopsis.to_string(),
            description: description.to_string(),
            options: HashMap::new(),
        }
    }

    pub fn add_option(&mut self, flag: &str, desc: &str) {
        self.options.insert(flag.to_string(), desc.to_string());
    }

    pub fn format_page(&self) -> String {
        let mut output = format!("{}({})\n\nNAME\n    {}\n\nSYNOPSIS\n    {}\n\nDESCRIPTION\n    {}\n",
            self.name.to_uppercase(), self.section, self.name, self.synopsis, self.description);

        if !self.options.is_empty() {
            output.push_str("\nOPTIONS\n");
            for (flag, desc) in &self.options {
                output.push_str(&format!("    {:<12} {}\n", flag, desc));
            }
        }
        output
    }
}

/// A HOWTO Guide representing community tutorials.
#[derive(Debug, Clone)]
pub struct HowToGuide {
    pub title: String,
    pub author: String,
    pub steps: Vec<String>,
    pub tags: Vec<String>,
}

impl HowToGuide {
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            steps: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: &str) {
        self.steps.push(step.to_string());
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.to_string());
    }
}

/// Collaborative wiki pages (SigmaWiki).
#[derive(Debug, Clone)]
pub struct WikiPage {
    pub title: String,
    pub content: String,
    pub revision: u32,
    pub tags: Vec<String>,
}

impl WikiPage {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            revision: 1,
            tags: Vec::new(),
        }
    }

    pub fn update_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
        self.revision += 1;
    }
}

/// Community chat and forum channels (IRC/Matrix/Forums).
#[derive(Debug, Clone)]
pub struct ForumPost {
    pub author: String,
    pub content: String,
    pub timestamp_secs: u64,
}

#[derive(Debug, Clone)]
pub struct ForumChannel {
    pub name: String,
    pub topic: String,
    pub posts: Vec<ForumPost>,
}

impl ForumChannel {
    pub fn new(name: &str, topic: &str) -> Self {
        Self {
            name: name.to_string(),
            topic: topic.to_string(),
            posts: Vec::new(),
        }
    }

    pub fn post_message(&mut self, author: &str, content: &str, timestamp: u64) {
        self.posts.push(ForumPost {
            author: author.to_string(),
            content: content.to_string(),
            timestamp_secs: timestamp,
        });
    }
}

/// The overall help and community knowledge manager.
#[derive(Debug, Clone)]
pub struct HelpSystem {
    pub man_pages: HashMap<String, ManPage>,
    pub guides: Vec<HowToGuide>,
    pub wiki_pages: HashMap<String, WikiPage>,
    pub forum_channels: HashMap<String, ForumChannel>,
}

impl HelpSystem {
    pub fn new() -> Self {
        Self {
            man_pages: HashMap::new(),
            guides: Vec::new(),
            wiki_pages: HashMap::new(),
            forum_channels: HashMap::new(),
        }
    }

    pub fn add_man_page(&mut self, page: ManPage) {
        self.man_pages.insert(page.name.clone(), page);
    }

    pub fn add_guide(&mut self, guide: HowToGuide) {
        self.guides.push(guide);
    }

    pub fn add_wiki_page(&mut self, page: WikiPage) {
        self.wiki_pages.insert(page.title.clone(), page);
    }

    pub fn add_forum_channel(&mut self, channel: ForumChannel) {
        self.forum_channels.insert(channel.name.clone(), channel);
    }

    pub fn search_man(&self, query: &str) -> Option<&ManPage> {
        self.man_pages.get(query)
    }

    pub fn search_wiki(&self, keyword: &str) -> Vec<&WikiPage> {
        self.wiki_pages
            .values()
            .filter(|p| p.title.to_lowercase().contains(&keyword.to_lowercase())
                || p.content.to_lowercase().contains(&keyword.to_lowercase()))
            .collect()
    }
}

impl Default for HelpSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_man_page_formatting() {
        let mut page = ManPage::new("sigma-exec", 1, "Execute safe command", "Executes an isolated command under active capability gates.");
        page.add_option("-p", "Specify active capability policy");

        let formatted = page.format_page();
        assert!(formatted.contains("SIGMA-EXEC(1)"));
        assert!(formatted.contains("SYNOPSIS"));
        assert!(formatted.contains("-p"));
    }

    #[test]
    fn test_howto_guide() {
        let mut guide = HowToGuide::new("How to build SigmaOS", "Jules");
        guide.add_step("Install dependencies");
        guide.add_step("Run make clean && make all");
        guide.add_tag("build");

        assert_eq!(guide.steps.len(), 2);
        assert_eq!(guide.tags[0], "build");
    }

    #[test]
    fn test_wiki_revisions() {
        let mut wiki = WikiPage::new("Sovereign IPC", "Sovereign IPC relies on atomic sharding.");
        assert_eq!(wiki.revision, 1);

        wiki.update_content("Sovereign IPC relies on ultra low latency lockless ring buffers.");
        assert_eq!(wiki.revision, 2);
    }

    #[test]
    fn test_forum_matrix_posts() {
        let mut channel = ForumChannel::new("#sigmaos-dev", "Discussion about SigmaOS kernel development");
        channel.post_message("Jules", "Hey team! PQC signatures are working perfectly.", 1718100000);

        assert_eq!(channel.posts.len(), 1);
        assert_eq!(channel.posts[0].author, "Jules");
        assert_eq!(channel.posts[0].content, "Hey team! PQC signatures are working perfectly.");
    }

    #[test]
    fn test_help_system_search() {
        let mut system = HelpSystem::new();
        let page = ManPage::new("sigma-sh", 1, "Sovereign Shell", "Interactive userland command interpreter.");
        system.add_man_page(page);

        let wiki = WikiPage::new("PQC Kyber-1024", "Kyber is used for post-quantum key encapsulation.");
        system.add_wiki_page(wiki);

        assert!(system.search_man("sigma-sh").is_some());
        assert_eq!(system.search_wiki("Kyber").len(), 1);
    }
}
