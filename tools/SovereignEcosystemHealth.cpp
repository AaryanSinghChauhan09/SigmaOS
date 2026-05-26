/**
 * SovereignEcosystemHealth.cpp
 * Feature #100 – Ecosystem Health Dashboard
 * =====================================================================
 * Absorbs: Ubuntu Advantage metrics, openSUSE Metrics Dashboard,
 *          Fedora Infra status, community.opensuse.org health board.
 * Mission: Single-pane view of SigmaOS repo, CI, package, and
 *          community health — all zero-dependency, terminal-renderable.
 * Branch:  tools-dev, docs-update
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Ecosystem {

static constexpr sigma_u32 MAX_METRICS     = 64;
static constexpr sigma_u32 MAX_ALERTS      = 32;

// ── Health signal ───────────────────────────────────────────────────
enum class HealthStatus : sigma_u8 {
    HEALTHY  = 0,
    DEGRADED = 1,
    CRITICAL = 2,
    UNKNOWN  = 3
};

struct Metric {
    char          name[48];
    char          unit[16];
    sigma_i64     value;
    sigma_i64     warning_threshold;
    sigma_i64     critical_threshold;
    HealthStatus  status;
};

struct EcosystemAlert {
    char         component[48];
    char         message[128];
    HealthStatus severity;
};

// ── Dashboard ───────────────────────────────────────────────────────
class SovereignEcosystemHealth {
public:
    static SovereignEcosystemHealth& getInstance() {
        static SovereignEcosystemHealth inst;
        return inst;
    }

    void init() {
        m_metric_count = 0;
        m_alert_count  = 0;
        // Pre-register core metrics
        addMetric("open_code_scan_alerts", "alerts", 0,  5, 20);
        addMetric("branch_divergence_commits", "commits", 0, 10, 50);
        addMetric("test_pass_rate_pct", "%", 100, 90, 70);
        addMetric("ci_build_duration_s", "s", 0, 120, 300);
        addMetric("wiki_pages_count", "pages", 0, 5, 1);
        addMetric("open_prs_count", "PRs", 0, 20, 50);
        addMetric("open_issues_count", "issues", 0, 30, 100);
        addMetric("community_packages", "pkgs", 0, 10, 1);
        addMetric("fuzz_corpus_size", "cases", 0, 100, 10);
        addMetric("kic_violations_last_scan", "violations", 0, 0, 1);
        sigma_log("[ECO-HEALTH] Ecosystem Health Dashboard initialised.");
        sigma_log("[ECO-HEALTH] 10 core metrics registered. Call update() to set live values.");
    }

    bool addMetric(const char* name, const char* unit,
                   sigma_i64 initial, sigma_i64 warn, sigma_i64 crit) {
        if (m_metric_count >= MAX_METRICS) return false;
        Metric& m = m_metrics[m_metric_count++];
        safe_copy(m.name, name, 48);
        safe_copy(m.unit, unit, 16);
        m.value              = initial;
        m.warning_threshold  = warn;
        m.critical_threshold = crit;
        m.status             = HealthStatus::UNKNOWN;
        return true;
    }

    bool updateMetric(const char* name, sigma_i64 value) {
        for (sigma_u32 i = 0; i < m_metric_count; i++) {
            if (key_eq(m_metrics[i].name, name)) {
                m_metrics[i].value = value;
                m_metrics[i].status = evaluate(m_metrics[i]);
                return true;
            }
        }
        return false;
    }

    void raiseAlert(const char* component, const char* message, HealthStatus sev) {
        if (m_alert_count >= MAX_ALERTS) {
            // Ring overwrite
            m_alert_count = MAX_ALERTS - 1;
        }
        EcosystemAlert& a = m_alerts[m_alert_count++];
        safe_copy(a.component, component, 48);
        safe_copy(a.message,   message,   128);
        a.severity = sev;
        sigma_log_info("[ECO-HEALTH] ALERT [%s]: %s\n", component, message);
    }

    void render() {
        sigma_log("\n╔══════════════════════════════════════════════════╗");
        sigma_log("║       SIGMAOS ECOSYSTEM HEALTH DASHBOARD         ║");
        sigma_log("╠══════════════════════════════════════════════════╣");

        for (sigma_u32 i = 0; i < m_metric_count; i++) {
            const Metric& m = m_metrics[i];
            const char* badge =
                (m.status == HealthStatus::HEALTHY)  ? "✓ HEALTHY " :
                (m.status == HealthStatus::DEGRADED)  ? "⚠ DEGRADED" :
                (m.status == HealthStatus::CRITICAL)  ? "✗ CRITICAL" : "? UNKNOWN ";
            sigma_log_info("║  [%s]  %-30s %5lld %s\n",
                           badge, m.name,
                           (long long)m.value, m.unit);
        }

        sigma_log("╠══════════════════════════════════════════════════╣");
        if (m_alert_count == 0) {
            sigma_log("║  No active alerts.                               ║");
        } else {
            for (sigma_u32 i = 0; i < m_alert_count; i++) {
                const char* sev =
                    (m_alerts[i].severity == HealthStatus::CRITICAL) ? "CRIT" :
                    (m_alerts[i].severity == HealthStatus::DEGRADED) ? "WARN" : "INFO";
                sigma_log_info("║  [%s] %s: %s\n",
                               sev, m_alerts[i].component, m_alerts[i].message);
            }
        }
        sigma_log("╚══════════════════════════════════════════════════╝\n");
    }

    HealthStatus overallStatus() {
        HealthStatus worst = HealthStatus::HEALTHY;
        for (sigma_u32 i = 0; i < m_metric_count; i++) {
            if (m_metrics[i].status > worst) worst = m_metrics[i].status;
        }
        return worst;
    }

private:
    Metric          m_metrics[MAX_METRICS];
    EcosystemAlert  m_alerts[MAX_ALERTS];
    sigma_u32       m_metric_count = 0;
    sigma_u32       m_alert_count  = 0;

    SovereignEcosystemHealth() = default;

    static void safe_copy(char* dst, const char* src, sigma_u32 max) {
        sigma_u32 i = 0;
        while (i + 1 < max && src[i]) { dst[i] = src[i]; i++; }
        dst[i] = '\0';
    }

    static bool key_eq(const char* a, const char* b) {
        sigma_u32 i = 0;
        while (a[i] && b[i] && a[i] == b[i]) i++;
        return a[i] == '\0' && b[i] == '\0';
    }

    static HealthStatus evaluate(const Metric& m) {
        // For "good = high" metrics like test_pass_rate_pct
        bool invert = (m.warning_threshold > m.critical_threshold);
        if (invert) {
            if (m.value >= m.warning_threshold)  return HealthStatus::HEALTHY;
            if (m.value >= m.critical_threshold) return HealthStatus::DEGRADED;
            return HealthStatus::CRITICAL;
        }
        // For "good = low" metrics like open alerts
        if (m.value <= m.warning_threshold)  return HealthStatus::HEALTHY;
        if (m.value <= m.critical_threshold) return HealthStatus::DEGRADED;
        return HealthStatus::CRITICAL;
    }
};

} // namespace Ecosystem
} // namespace SigmaOS

// ── C API ──────────────────────────────────────────────────────────
extern "C" {

void eco_health_init() {
    SigmaOS::Ecosystem::SovereignEcosystemHealth::getInstance().init();
}

void eco_health_update(const char* metric, sigma_i64 value) {
    SigmaOS::Ecosystem::SovereignEcosystemHealth::getInstance().updateMetric(metric, value);
}

void eco_health_render() {
    SigmaOS::Ecosystem::SovereignEcosystemHealth::getInstance().render();
}

sigma_u8 eco_health_overall() {
    return static_cast<sigma_u8>(
        SigmaOS::Ecosystem::SovereignEcosystemHealth::getInstance().overallStatus());
}

} // extern "C"
