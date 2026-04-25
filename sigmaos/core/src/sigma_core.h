#ifndef SIGMA_CORE_H
#define SIGMA_CORE_H

#ifdef __cplusplus
extern "C" {
#endif

// APCB Signatures
void* apcb_create(int pid, const char* intent);
void apcb_handle_crash(void* ptr, const char* traceback);
void apcb_destroy(void* ptr);

// Scheduler Signatures
void* scheduler_init();
void scheduler_add_goal(void* ptr, const char* goal);
void scheduler_process(void* ptr);

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

// Automation Signatures
void* auto_init();
void auto_run_all(void* ptr);
void auto_trigger_rollback();

#ifdef __cplusplus
}
#endif

#endif // SIGMA_CORE_H
