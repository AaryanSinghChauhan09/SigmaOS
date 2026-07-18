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
        let mut output = format!(
            "{}({})\n\nNAME\n    {}\n\nSYNOPSIS\n    {}\n\nDESCRIPTION\n    {}\n",
            self.name.to_uppercase(),
            self.section,
            self.name,
            self.synopsis,
            self.description
        );

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

/// Bug Bounty vulnerability report states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BountyStatus {
    Submitted,
    Auditing,
    Resolved,
    Rejected,
}

/// Represents an individual Bug Bounty vulnerability report
#[derive(Debug, Clone)]
pub struct BugBountyReport {
    pub report_id: u32,
    pub reporter: String,
    pub vulnerability_title: String,
    pub severity: String, // e.g. "Critical", "High", "Medium"
    pub status: BountyStatus,
    pub reward_amount_usd: u32,
}

/// Standardized Bug Bounty Program tracking contributor security incentives
#[derive(Debug, Clone)]
pub struct BugBountyProgram {
    pub program_name: String,
    pub reports: Vec<BugBountyReport>,
    pub total_payout_usd: u32,
}

impl BugBountyProgram {
    pub fn new(name: &str) -> Self {
        Self {
            program_name: name.to_string(),
            reports: Vec::new(),
            total_payout_usd: 0,
        }
    }

    pub fn submit_report(&mut self, report_id: u32, reporter: &str, title: &str, severity: &str) {
        self.reports.push(BugBountyReport {
            report_id,
            reporter: reporter.to_string(),
            vulnerability_title: title.to_string(),
            severity: severity.to_string(),
            status: BountyStatus::Submitted,
            reward_amount_usd: 0,
        });
    }

    pub fn audit_and_reward(
        &mut self,
        id: u32,
        approved: bool,
        reward: u32,
    ) -> Result<(), &'static str> {
        for report in &mut self.reports {
            if report.report_id == id {
                if approved {
                    report.status = BountyStatus::Resolved;
                    report.reward_amount_usd = reward;
                    self.total_payout_usd += reward;
                } else {
                    report.status = BountyStatus::Rejected;
                }
                return Ok(());
            }
        }
        Err("Report ID not found")
    }
}

/// Represents a talk or presentation at a conference
#[derive(Debug, Clone)]
pub struct ConferenceTalk {
    pub speaker: String,
    pub title: String,
    pub duration_mins: u32,
}

/// Outlines conferences and outreach meetups (e.g. DebConf, FOSDEM style)
#[derive(Debug, Clone)]
pub struct CommunityConference {
    pub name: String,
    pub location: String,
    pub schedules: Vec<ConferenceTalk>,
}

impl CommunityConference {
    pub fn new(name: &str, location: &str) -> Self {
        Self {
            name: name.to_string(),
            location: location.to_string(),
            schedules: Vec::new(),
        }
    }

    pub fn schedule_talk(&mut self, speaker: &str, title: &str, duration: u32) {
        self.schedules.push(ConferenceTalk {
            speaker: speaker.to_string(),
            title: title.to_string(),
            duration_mins: duration,
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
            .filter(|p| {
                p.title.to_lowercase().contains(&keyword.to_lowercase())
                    || p.content.to_lowercase().contains(&keyword.to_lowercase())
            })
            .collect()
    }

    /// Retrieves a manual localized using translation dictionary (simulation)
    pub fn translate_man_summary(
        &self,
        query: &str,
        locale_dictionary: &HashMap<String, String>,
    ) -> String {
        if let Some(page) = self.man_pages.get(query) {
            let key = format!("man_{}_summary", query);
            locale_dictionary
                .get(&key)
                .cloned()
                .unwrap_or(page.synopsis.clone())
        } else {
            "Manual not found".to_string()
        }
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
        let mut page = ManPage::new(
            "sigma-exec",
            1,
            "Execute safe command",
            "Executes an isolated command under active capability gates.",
        );
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
        let mut channel = ForumChannel::new(
            "#sigmaos-dev",
            "Discussion about SigmaOS kernel development",
        );
        channel.post_message(
            "Jules",
            "Hey team! PQC signatures are working perfectly.",
            1718100000,
        );

        assert_eq!(channel.posts.len(), 1);
        assert_eq!(channel.posts[0].author, "Jules");
        assert_eq!(
            channel.posts[0].content,
            "Hey team! PQC signatures are working perfectly."
        );
    }

    #[test]
    fn test_help_system_search() {
        let mut system = HelpSystem::new();
        let page = ManPage::new(
            "sigma-sh",
            1,
            "Sovereign Shell",
            "Interactive userland command interpreter.",
        );
        system.add_man_page(page);

        let wiki = WikiPage::new(
            "PQC Kyber-1024",
            "Kyber is used for post-quantum key encapsulation.",
        );
        system.add_wiki_page(wiki);

        assert!(system.search_man("sigma-sh").is_some());
        assert_eq!(system.search_wiki("Kyber").len(), 1);
    }

    #[test]
    fn test_bug_bounty_program() {
        let mut bounty = BugBountyProgram::new("SigmaOS Hardening Bounty");
        bounty.submit_report(101, "Alice", "PQC key validation bypass", "Critical");

        assert_eq!(bounty.reports.len(), 1);
        assert_eq!(bounty.reports[0].status, BountyStatus::Submitted);

        assert!(bounty.audit_and_reward(101, true, 5000).is_ok());
        assert_eq!(bounty.reports[0].status, BountyStatus::Resolved);
        assert_eq!(bounty.reports[0].reward_amount_usd, 5000);
        assert_eq!(bounty.total_payout_usd, 5000);
    }

    #[test]
    fn test_community_conferences() {
        let mut conf = CommunityConference::new("SigmaConf 2024", "New Delhi");
        conf.schedule_talk("Jules", "Unifying Distro Ecosystems with Rust", 45);

        assert_eq!(conf.schedules.len(), 1);
        assert_eq!(conf.schedules[0].speaker, "Jules");
        assert_eq!(
            conf.schedules[0].title,
            "Unifying Distro Ecosystems with Rust"
        );
    }

    #[test]
    fn test_help_localized_manuals() {
        let mut system = HelpSystem::new();
        let page = ManPage::new(
            "sigma-pkg",
            1,
            "Sigma Package Manager",
            "Registers, resolves, and verifies universal packages.",
        );
        system.add_man_page(page);

        let mut dict = HashMap::new();
        dict.insert(
            "man_sigma-pkg_summary".to_string(),
            "Gestionnaire de paquets universels de SigmaOS".to_string(),
        );

        let summary = system.translate_man_summary("sigma-pkg", &dict);
        assert_eq!(summary, "Gestionnaire de paquets universels de SigmaOS");
    }
}
