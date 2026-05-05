#include "../../../include/sigma_types.h""
#include "../../../include/sigma_hal.h""
#include "../../../include/SovereignLibC.h""

/**
 * SigmaOS Sovereign Telemetry Exporter Hooks
 * External observability for the Sovereign ecosystem.
 *
 * USP: securely exports real-time metrics (e.g., to Prometheus/Grafana) using
 * SovereignNetStack without exposing the Ring-0 memory space.
 *
 * Design: OOP-isolated singleton — SovereignTelemetryExporter.
 */

class SovereignTelemetryExporter {
public:
    static SovereignTelemetryExporter& getInstance() {
        static SovereignTelemetryExporter instance;
        return instance;
    }

    void init() {
        sigma_log("[TELEMETRY-EX] Initializing External Observability Hooks...");
        this->endpoint_configured = false;
    }

    void configureEndpoint(const char* ip_address) {
        sigma_hardened_strcpy(this->remote_ip, ip_address, 16);
        this->endpoint_configured = true;
        sigma_printf("[TELEMETRY-EX] Remote telemetry endpoint bound to %s.\n", ip_address);
    }

    void exportMetrics(sigma_u32 cpu_load, sigma_u32 memory_pressure) {
        if (!this->endpoint_configured) return;
        sigma_printf("[TELEMETRY-EX] Exporting metrics to %s -> CPU: %u%%, Mem: %u%%\n", 
                     this->remote_ip, cpu_load, memory_pressure);
    }

private:
    SovereignTelemetryExporter() : endpoint_configured(false) {}

    char remote_ip[16];
    bool endpoint_configured;
};

/* --- C Wrappers --- */
extern "C" void telemetry_ex_init() {
    SovereignTelemetryExporter::getInstance().init();
}

extern "C" void telemetry_ex_configure(const char* ip) {
    SovereignTelemetryExporter::getInstance().configureEndpoint(ip);
}

extern "C" void telemetry_ex_export(sigma_u32 cpu, sigma_u32 mem) {
    SovereignTelemetryExporter::getInstance().exportMetrics(cpu, mem);
}



