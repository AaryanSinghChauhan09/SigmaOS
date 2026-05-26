/**
 * ===========================================================================
 * Σ SIGMAOS: SOVEREIGN COMMUNITY PORTAL (S-COMMUNITY) v1.0
 * ===========================================================================
 * Mission: Slackware/EndeavourOS-grade community infrastructure.
 *          Contributor registry, documentation index, build script manager
 *          (SlackBuild-style), badge/gamification system, and health metrics.
 *
 * Inspired by: Slackware SlackBuilds / EndeavourOS Community / Arch Wiki
 * ZERO-DEPENDENCY: Self-hosted community infrastructure integrated at kernel level.
 * ===========================================================================
 */

#include "../../../include/sigma_log.h"
#include "../../../include/sigma_kernel_types.h"

/* ---- Internal Constants ---- */
#define COMMUNITY_MAX_CONTRIBUTORS  1024
#define COMMUNITY_MAX_DOCS           512
#define COMMUNITY_MAX_BUILD_SCRIPTS  256

namespace SigmaOS {
namespace Kernel {
namespace Community {

/* =========================================================================
 * CONTRIBUTOR REGISTRY — Track and authenticate community members
 * ========================================================================= */
struct Contributor {
    sigma_u32 id;
    char      username[64];
    char      email[128];
    sigma_u32 commits;
    sigma_u32 reviews;
    sigma_u32 docs_written;
    sigma_u32 badge_flags;    /* Bitmask of earned badges */
    bool      core_team;
    bool      verified;
};

/* Badge flag definitions */
#define BADGE_FIRST_COMMIT      BIT(0)
#define BADGE_KERNEL_HACKER     BIT(1)
#define BADGE_DOC_WRITER        BIT(2)
#define BADGE_BUG_HUNTER        BIT(3)
#define BADGE_SECURITY_AUDITOR  BIT(4)
#define BADGE_COMMUNITY_MENTOR  BIT(5)
#define BADGE_PERFORMANCE_TUNER BIT(6)
#define BADGE_DRIVER_DEV        BIT(7)

static Contributor s_contributors[COMMUNITY_MAX_CONTRIBUTORS];
static sigma_u32   s_contributor_count = 0;

/* =========================================================================
 * DOCUMENTATION INDEX — Arch-wiki-style doc management
 * ========================================================================= */
struct DocEntry {
    sigma_u32 id;
    char      title[128];
    char      category[64];
    sigma_u32 revision;
    sigma_u32 views;
    sigma_u32 author_id;
    bool      published;
};

static DocEntry  s_docs[COMMUNITY_MAX_DOCS];
static sigma_u32 s_doc_count = 0;

/* =========================================================================
 * BUILD SCRIPT REGISTRY — SlackBuild-inspired reproducible scripts
 * ========================================================================= */
struct BuildScript {
    sigma_u32 id;
    char      package_name[64];
    char      version[32];
    char      maintainer[64];
    sigma_u32 download_count;
    bool      verified;
    bool      sovereign_approved;
};

static BuildScript s_build_scripts[COMMUNITY_MAX_BUILD_SCRIPTS];
static sigma_u32   s_build_script_count = 0;

/* =========================================================================
 * COMMUNITY PORTAL ENGINE — Initialization and management
 * ========================================================================= */
class SovereignCommunityPortal {
public:
    static SovereignCommunityPortal& getInstance() {
        static SovereignCommunityPortal instance;
        return instance;
    }

    void init() {
        sigma_log("[COMMUNITY]: ═══════════════════════════════════════════════\n");
        sigma_log("[COMMUNITY]: Σ SOVEREIGN COMMUNITY PORTAL v1.0 — Initializing...\n");
        sigma_log("[COMMUNITY]: ═══════════════════════════════════════════════\n");

        /* Register core documentation pages */
        registerDoc("Getting Started with SigmaOS", "Onboarding", 1);
        registerDoc("Kernel Architecture Overview", "Architecture", 1);
        registerDoc("Contributing to SigmaOS — Developer Guide", "Contributing", 1);
        registerDoc("Zenith Desktop Environment — User Guide", "Desktop", 1);
        registerDoc("OmniPkg Package Manager — Reference", "Packages", 1);
        registerDoc("Security Architecture — Zero-Trust Model", "Security", 1);
        registerDoc("Building SigmaOS from Source", "Development", 1);
        registerDoc("SovereignSandbox — Container Isolation Guide", "Security", 1);
        registerDoc("Hardware Abstraction Layer — Driver Development", "Drivers", 1);
        registerDoc("SlackBuild-Style Package Creation", "Packages", 1);

        /* Register seed build scripts */
        registerBuildScript("sigma-core", "1.0.0", "SigmaOS Core Team");
        registerBuildScript("zenith-desktop", "1.0.0", "SigmaOS Core Team");
        registerBuildScript("omnipkg", "1.0.0", "SigmaOS Core Team");
        registerBuildScript("sovereign-sandbox", "1.0.0", "SigmaOS Core Team");
        registerBuildScript("sigma-crypto", "1.0.0", "SigmaOS Core Team");

        sigma_log("[COMMUNITY]: %d documentation pages indexed.\n", s_doc_count);
        sigma_log("[COMMUNITY]: %d build scripts registered.\n", s_build_script_count);
        sigma_log("[COMMUNITY]: Community Portal READY.\n");
    }

    sigma_u32 registerContributor(const char* username, const char* email) {
        if (s_contributor_count >= COMMUNITY_MAX_CONTRIBUTORS) {
            sigma_log_warn("[COMMUNITY]: WARNING — Contributor registry full.\n");
            return 0;
        }

        Contributor* c = &s_contributors[s_contributor_count];
        c->id = s_contributor_count + 1;
        sigma_strncpy(c->username, username, 64);
        sigma_strncpy(c->email, email, 128);
        c->commits = 0;
        c->reviews = 0;
        c->docs_written = 0;
        c->badge_flags = 0;
        c->core_team = false;
        c->verified = true;

        s_contributor_count++;
        sigma_log("[COMMUNITY]: Contributor registered — #%d '%s' (%s)\n",
                  c->id, username, email);
        return c->id;
    }

    void awardBadge(sigma_u32 contributor_id, sigma_u32 badge) {
        if (contributor_id == 0 || contributor_id > s_contributor_count) return;
        Contributor* c = &s_contributors[contributor_id - 1];
        c->badge_flags |= badge;

        const char* badge_name = "Unknown";
        if (badge == BADGE_FIRST_COMMIT) badge_name = "First Commit";
        else if (badge == BADGE_KERNEL_HACKER) badge_name = "Kernel Hacker";
        else if (badge == BADGE_DOC_WRITER) badge_name = "Doc Writer";
        else if (badge == BADGE_BUG_HUNTER) badge_name = "Bug Hunter";
        else if (badge == BADGE_SECURITY_AUDITOR) badge_name = "Security Auditor";
        else if (badge == BADGE_COMMUNITY_MENTOR) badge_name = "Community Mentor";

        sigma_log("[COMMUNITY]: 🏆 Badge '%s' awarded to contributor '%s'.\n",
                  badge_name, c->username);
    }

    void reportHealth() {
        sigma_log("\n--- Σ SOVEREIGN COMMUNITY HEALTH ---\n");
        sigma_log("| Contributors     : %d\n", s_contributor_count);
        sigma_log("| Documentation    : %d pages\n", s_doc_count);
        sigma_log("| Build Scripts    : %d\n", s_build_script_count);
        sigma_log("| Core Team        : ");
        sigma_u32 core = 0;
        for (sigma_u32 i = 0; i < s_contributor_count; i++) {
            if (s_contributors[i].core_team) core++;
        }
        sigma_log("%d\n", core);
        sigma_log("------------------------------------\n");
    }

private:
    SovereignCommunityPortal() = default;

    void registerDoc(const char* title, const char* category, sigma_u32 revision) {
        if (s_doc_count >= COMMUNITY_MAX_DOCS) return;
        DocEntry* d = &s_docs[s_doc_count];
        d->id = s_doc_count + 1;
        sigma_strncpy(d->title, title, 128);
        sigma_strncpy(d->category, category, 64);
        d->revision = revision;
        d->views = 0;
        d->author_id = 0;
        d->published = true;
        s_doc_count++;
    }

    void registerBuildScript(const char* name, const char* version, const char* maintainer) {
        if (s_build_script_count >= COMMUNITY_MAX_BUILD_SCRIPTS) return;
        BuildScript* b = &s_build_scripts[s_build_script_count];
        b->id = s_build_script_count + 1;
        sigma_strncpy(b->package_name, name, 64);
        sigma_strncpy(b->version, version, 32);
        sigma_strncpy(b->maintainer, maintainer, 64);
        b->download_count = 0;
        b->verified = true;
        b->sovereign_approved = true;
        s_build_script_count++;
    }
};

} // namespace Community
} // namespace Kernel
} // namespace SigmaOS

/* =========================================================================
 * C WRAPPERS
 * ========================================================================= */
extern "C" void community_init() {
    SigmaOS::Kernel::Community::SovereignCommunityPortal::getInstance().init();
}

extern "C" void community_report_health() {
    SigmaOS::Kernel::Community::SovereignCommunityPortal::getInstance().reportHealth();
}
