#ifndef SIGMA_CORE_H
#define SIGMA_CORE_H

#ifdef __cplusplus
extern "C" {
#endif

// APCB Signatures
void* apcb_create(int pid, const char* intent);
void apcb_handle_crash(void* ptr, const char* traceback);
void apcb_destroy(void* ptr);

// UI Signatures
void ui_init();
void ui_render_frame();
void ui_set_morph_profile(const char* profile);
void ui_toggle_shader(const char* effect, int enabled);

// Vector Memory Signatures
void mem_store(const char* intent, const char* vector_json);
void mem_query(const char* intent_filter);
void mem_prune(int days_old);

// State Ledger Signatures
void ledger_append(const char* transition_hash);
void ledger_audit();

// Scheduler Signatures
void* scheduler_init();
void scheduler_add_goal(void* ptr, const char* goal);
void scheduler_process(void* ptr);

// Performance Signatures
void perf_balance();
void perf_cache_adaptive();
void perf_isolate(int pid);

// Subsystem Signatures
void subsystem_load(const char* name);
void subsystem_unload(const char* name);
int subsystem_is_active(const char* name);

// Security Signatures
void sec_audit();
void sec_encrypt_file(const char* filename);

// Networking Signatures
void net_secure_connect();
void net_audit();

// Multimedia Signatures
void media_load_codec(const char* codec);
void media_list_codecs();

// Component Management Signatures
void comp_split(const char* component_name);
void comp_audit_suites(const char* suite_path);
void comp_optimize(const char* component_name);
int comp_get_total_shards();

// Automation Signatures
void* auto_init();
void auto_run_all(void* ptr);
void auto_trigger_rollback();
void auto_watchdog_start(const char* shard_name);
void auto_watchdog_status();
void auto_patch_nightly();

#ifdef __cplusplus
}
#endif

#endif // SIGMA_CORE_H
