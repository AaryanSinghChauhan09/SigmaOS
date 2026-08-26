// Community Handbook, Reproducible Package Recipes & Blueprint Toolkit for SigmaOS
// Inspired by Arch Wiki, FreeBSD Handbook, Gentoo Portage, Void XBPS-src, and OpenBSD ports.

use std::collections::HashMap;

/// Article categories for the Community Handbook
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticleCategory {
    Installation,
    SystemAdministration,
    Networking,
    SecurityHardening,
    PackageManagement,
    KernelTuning,
    HardwareDrivers,
    Troubleshooting,
}

/// Community Handbook Article (Arch Wiki & FreeBSD Handbook parity)
#[derive(Debug, Clone)]
pub struct HandbookArticle {
    pub id: usize,
    pub title: String,
    pub category: ArticleCategory,
    pub tags: Vec<String>,
    pub content: String,
    pub distro_inspiration: String, // e.g. "Arch Linux Wiki", "FreeBSD Handbook"
}

/// Community Handbook Catalog Engine
pub struct CommunityHandbookCatalog {
    pub articles: HashMap<usize, HandbookArticle>,
    pub next_id: usize,
}

impl CommunityHandbookCatalog {
    pub fn new() -> Self {
        let mut catalog = Self {
            articles: HashMap::new(),
            next_id: 1,
        };
        catalog.seed_handbook_defaults();
        catalog
    }

    pub fn seed_handbook_defaults(&mut self) {
        self.add_article(
            "SigmaOS Installation & Partitioning Guide",
            ArticleCategory::Installation,
            &["install", "partitioning", "btrfs", "zfs"],
            "Guide covering UEFI, S-Boot, Btrfs snapshots, and ZFS pool setup inspired by Arch Wiki.",
            "Arch Linux Wiki",
        );

        self.add_article(
            "FreeBSD GEOM Storage & GELI Disk Encryption",
            ArticleCategory::SystemAdministration,
            &["geom", "geli", "encryption", "raid"],
            "Configuring FreeBSD GEOM transformations, g_mirror, g_stripe, and GELI encryption.",
            "FreeBSD Handbook",
        );

        self.add_article(
            "OpenBSD pledge and unveil Security Sandboxing",
            ArticleCategory::SecurityHardening,
            &["pledge", "unveil", "sandboxing", "security"],
            "Applying OpenBSD pledge system call restrictions and unveil filesystem path restriction policies.",
            "OpenBSD Manual",
        );
    }

    pub fn add_article(&mut self, title: &str, category: ArticleCategory, tags: &[&str], content: &str, inspiration: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let article = HandbookArticle {
            id,
            title: title.to_string(),
            category,
            tags: tags.iter().map(|t| t.to_string()).collect(),
            content: content.to_string(),
            distro_inspiration: inspiration.to_string(),
        };
        self.articles.insert(id, article);
        id
    }

    pub fn search_articles(&self, query: &str) -> Vec<&HandbookArticle> {
        let q = query.to_lowercase();
        self.articles
            .values()
            .filter(|a| a.title.to_lowercase().contains(&q) || a.content.to_lowercase().contains(&q) || a.tags.iter().any(|t| t.to_lowercase().contains(&q)))
            .collect()
    }
}

impl Default for CommunityHandbookCatalog {
    fn default() -> Self {
        Self::new()
    }
}


pub struct HybridFirewallTemplateStore {
    pub templates: HashMap<String, String>,
}

impl HybridFirewallTemplateStore {
    pub fn new() -> Self {
        let mut store = Self { templates: HashMap::new() };
        store.templates.insert("default-mesh-shield".to_string(), "table inet filter { chain input { type filter hook input priority 0; policy drop; } }".to_string());
        store
    }
}

impl Default for HybridFirewallTemplateStore {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VirtualizationBlueprintStore {
    pub blueprints: HashMap<String, String>,
}

impl VirtualizationBlueprintStore {
    pub fn new() -> Self {
        let mut store = Self { blueprints: HashMap::new() };
        store.blueprints.insert("micro-vm-node".to_string(), "virtio-net,virtio-blk,memory=512M,vcpu=2".to_string());
        store
    }
}

impl Default for VirtualizationBlueprintStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Recipe source format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeSourceFormat {
    ArchPkgBuild,
    GentooEbuild,
    VoidXbpsSrc,
    FreeBsdPort,
    SigmaRecipe,
}

/// Reproducible Package Recipe (Arch PKGBUILD, Gentoo ebuild, Void XBPS-src parity)
#[derive(Debug, Clone)]
pub struct PackageRecipe {
    pub name: String,
    pub version: String,
    pub format: RecipeSourceFormat,
    pub source_url: String,
    pub sha256_checksum: String,
    pub build_dependencies: Vec<String>,
    pub run_dependencies: Vec<String>,
    pub use_flags: Vec<String>,
}

/// Reproducible Package Recipe Manager
pub struct ReproduciblePackageRecipeManager {
    pub recipes: HashMap<String, PackageRecipe>,
}

impl ReproduciblePackageRecipeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            recipes: HashMap::new(),
        };
        manager.seed_default_recipes();
        manager
    }

    pub fn seed_default_recipes(&mut self) {
        self.register_recipe(PackageRecipe {
            name: "zenith-desktop".to_string(),
            version: "1.0.0".to_string(),
            format: RecipeSourceFormat::SigmaRecipe,
            source_url: "https://packages.sigmaos.org/src/zenith-1.0.0.tar.gz".to_string(),
            sha256_checksum: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            build_dependencies: vec!["rust".to_string(), "cargo".to_string(), "wayland-protocols".to_string()],
            run_dependencies: vec!["pixman".to_string(), "libxkbcommon".to_string()],
            use_flags: vec!["vulkan".to_string(), "wayland".to_string()],
        });
    }

    pub fn register_recipe(&mut self, recipe: PackageRecipe) {
        self.recipes.insert(recipe.name.clone(), recipe);
    }

    pub fn verify_checksum(&self, name: &str, computed_sha256: &str) -> Result<bool, &'static str> {
        let recipe = self.recipes.get(name).ok_or("Recipe not found")?;
        Ok(recipe.sha256_checksum == computed_sha256)
    }
}

impl Default for ReproduciblePackageRecipeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Security Profile Template Store (Pledge/Unveil, AppArmor/SELinux, Capsicum)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityModelType {
    OpenBsdPledgeUnveil,
    FreeBsdCapsicum,
    LinuxAppArmor,
    LinuxSelinux,
}

pub struct SecurityTemplate {
    pub name: String,
    pub model_type: SecurityModelType,
    pub profile_rules: String,
}

pub struct SecurityProfileTemplateStore {
    pub templates: HashMap<String, SecurityTemplate>,
}

impl SecurityProfileTemplateStore {
    pub fn new() -> Self {
        let mut store = Self {
            templates: HashMap::new(),
        };
        store.register_template("browser_sandboxed", SecurityModelType::OpenBsdPledgeUnveil, "pledge: stdio rpath wpath cpath inet dns tty; unveil: /usr/share r, /home/user/Downloads rwc");
        store.register_template("hardened-webserver", SecurityModelType::LinuxAppArmor, "profile hardened-webserver");
        store
    }

    pub fn register_template(&mut self, name: &str, model_type: SecurityModelType, rules: &str) {
        self.templates.insert(
            name.to_string(),
            SecurityTemplate {
                name: name.to_string(),
                model_type,
                profile_rules: rules.to_string(),
            },
        );
    }
}

impl Default for SecurityProfileTemplateStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_community_handbook_and_recipes() {
        let catalog = CommunityHandbookCatalog::new();
        let articles = catalog.search_articles("FreeBSD");
        assert!(!articles.is_empty());
        assert_eq!(articles[0].distro_inspiration, "FreeBSD Handbook");

        let recipe_mgr = ReproduciblePackageRecipeManager::new();
        assert!(recipe_mgr.verify_checksum("zenith-desktop", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855").unwrap());

        let sec_store = SecurityProfileTemplateStore::new();
        assert!(sec_store.templates.contains_key("browser_sandboxed"));
    }
}
