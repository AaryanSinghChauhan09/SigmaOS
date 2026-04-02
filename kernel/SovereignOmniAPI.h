// ==============================================================================
// SIGMAOS SOVEREIGN ARCHITECTURE
// CORE API: Sovereign Omni-API (AUTO-GENERATED)
// DEPENDENCIES: NONE (-nostdlib -ffreestanding)
// ==============================================================================

#ifndef SOVEREIGN_OMNI_API_H
#define SOVEREIGN_OMNI_API_H

#include "sigma_kernel_types.h"

// -> Source Shard: aether_abs.c
extern void aether_absorb_cloud(AetherAbsorber* a);
extern void aether_absorb_lattice(AetherAbsorber* a);
extern void aether_absorb_ai(AetherAbsorber* a);
extern void aether_deploy_unity(void);

// -> Source Shard: aether_orch.c
extern void aether_init_core(void);
extern void aether_register_trigger(u32 trigger_id, u64 target_shard_id);
extern void aether_pulse_trigger(u32 trigger_id);
extern void aether_audit(void);

// -> Source Shard: app_manager.c
extern void app_manager_init(void);
extern k_status register_app(const char* name);
extern void app_switch_state(u32 idx, AppState state);
extern void app_personalize(u32 idx, u32 theme);

// -> Source Shard: audio_engine_shard.c
extern void __attribute__((noinline)) mix_audio_streams(int16_t* stream_a, int16_t* stream_b, int16_t* out, uint32_t len);
extern void apply_reverb_filter(int16_t* buffer, uint32_t size, uint32_t delay_samples, float decay);

// -> Source Shard: audit_master.c
extern void audit_master_init(void);
extern k_status audit_now(void);

// -> Source Shard: automation_shard.c
extern void automation_shard_init(void);
extern void automation_on_tick(u64 current_tick);
extern void automation_audit(void);

// -> Source Shard: bnss_shard.c
extern void bnss_init(void);
extern k_status bnss_arrest_audit(void);
extern k_status bnss_remand_audit(u32 days);
extern k_status bsa_certificate_gen(void* shard_ptr);
extern k_status bnss_search_audit(void);

// -> Source Shard: build_system_shard.c
extern void __attribute__((noinline)) resolve_build_graph(build_node_t* roots, uint32_t count);
extern void dispatch_parallel_compilation(void);
extern void sign_build_artifact(void* binary_data, uint32_t size);

// -> Source Shard: camera_shard.c
extern void camera_init(void);
extern k_status camera_capture_frame(void* external_buffer);
extern k_status camera_apply_filter(void* frame_ptr, const char* filter_name);
extern k_status camera_scratch_trigger(u32 event_id);
extern k_status camera_forensic_session_start(const char* evidence_tag);
extern k_status camera_forensic_session_stop(void);
extern void camera_list_filters(void);
extern void camera_process_events(void);

// -> Source Shard: cgroup_shard.c
extern void cgroup_init(void);
extern k_status cgroup_create(const char* name, u32 weight, u64 mem_limit);
extern bool_t cgroup_limit_check(u32 cg_id, u64 mem_req);

// -> Source Shard: checklist_shard.c
extern void checklist_init(void);
extern k_status checklist_query_domain(LawDomain domain, u32* out_count);
extern k_status checklist_audit_deadline(u32 filing_date, u32 section_limit);
extern k_status checklist_generate_report(void);
extern u32 checklist_total_items(void);

// -> Source Shard: console.c
extern void serial_init(void);
extern void serial_putc(char c);
extern void kprintf(const char* fmt, ...);
extern                 else if (*fmt == 'd') { i64 v = va_arg(ap, i64); if(v<0){kprint_char('-');v=-v;} kprint_u64((u64)v,10); }
extern void console_init(void);

// -> Source Shard: const_shard.c
extern void const_init(void);
extern k_status law_query_section(const char* act, u32 section);
extern k_status const_audit_rights(void);

// -> Source Shard: cs_research_shard.c
extern void cs_research_init(void);
extern k_status research_index_paper(const char* title, const char* doi);
extern k_status research_audit_complexity(void* shard_ptr);

// -> Source Shard: dist_shard.c
extern void dist_shard_init(void);
extern k_status dist_register_node(u32 remote_id, u32 cpus, u64 mem);
extern k_status dist_delegate_task(u64 task_id, const void* data, u32 len);
extern void dist_audit(void);

// -> Source Shard: distro_forge.c
extern void distro_forge_init(void);
extern k_status forge_generate_distro(const char* name, bool_t include_pqc, bool_t include_ai);

// -> Source Shard: elf_loader.c
extern void elf_selftest(void);

// -> Source Shard: forensics_shard.c
extern void forensics_init(void);
extern k_status forensics_sharded_dump(u64 start_addr, usize size);
extern k_status forensics_carve_artifact(const char* signature);

// -> Source Shard: gpu_compute_shard.c
extern void __attribute__((noinline)) init_gpu_compute_engine(void);
extern void submit_compute_shader(void* shader_bytecode, uint32_t size);
extern void wait_for_gpu_idle(void);
extern void execute_hardware_raytracing_pass(void* scene_tree);

// -> Source Shard: hal.c
extern void hal_discover_hardware(void);
extern SigmaDevice* hal_find_device(u16 vendor, u16 device);

// -> Source Shard: health.c
extern void health_init(void);
extern void health_reset_shard(u32 shard_id);
extern void health_report_error(u32 shard_id);
extern void health_audit_system(void);

// -> Source Shard: hot_replace.c
extern void hot_replace_init(void);
extern k_status hot_replace_register(const char* name, void* original, void* patched);
extern void hot_replace_activate(u32 idx);

// -> Source Shard: identity.c
extern bool_t id_verify_token(u32 pid, const u8* token, u32 len);
extern void id_init(void);
extern void id_audit(void);

// -> Source Shard: idt.c
extern void pic_unmask_irq(u8 irq);
extern void idt_init(void);
extern void idt_register_handler(u32 vec, sigma_irq_handler_t fn);
extern void sigma_interrupt_handler(SigmaInterruptFrame* frame);

// -> Source Shard: io_scheduler.c
extern void io_scheduler_init(void);
extern k_status io_submit(IORequest* req);
extern void io_dispatch_pulse(void);

// -> Source Shard: ipc.c
extern i32 ipc_pipe_create(i32* read_fd, i32* write_fd);
extern i64 ipc_pipe_write(i32 fd, const void* buf, usize n);
extern i64 ipc_pipe_read(i32 fd, void* buf, usize n);
extern i32 ipc_pipe_close(i32 fd);
extern i32 ipc_mq_open(const char* name);
extern i32 ipc_mq_send(i32 mqd, u32 mtype, const void* data, u32 len);
extern i64 ipc_mq_recv(i32 mqd, u32* mtype_out, void* buf, u32 buflen);
extern i32 ipc_shm_get(u32 key, usize size);
extern void* ipc_shm_attach(i32 shmid);
extern i32 ipc_shm_detach(i32 shmid);
extern i32 ipc_futex_wait(volatile u32* uaddr, u32 val);
extern i32 ipc_futex_wake(volatile u32* uaddr, u32 n);
extern void ipc_init(void);

// -> Source Shard: keyboard_master.c
extern void keyboard_master_init(void);
extern void keyboard_on_event(u32 mod, u32 key);

// -> Source Shard: ksm_shard.c
extern void ksm_init(void);
extern k_status ksm_scan_and_merge(void);

// -> Source Shard: lattice_sync.c
extern void lattice_sync_init(void);
extern k_status lattice_sync_send_shard(u32 shard_id, const void* buffer, u32 len);
extern k_status lattice_sync_process_packet(LatticePacket* pkt);

// -> Source Shard: legal_shard.c
extern void legal_init(void);
extern k_status legal_bnss_proc_audit(u32 step_id);
extern k_status legal_ipc_search(const char* section);
extern k_status legal_citation_audit(const char* cite);

// -> Source Shard: linux_shim.c
extern void linux_shim_init(void);
extern k_status linux_register_driver(LinuxDriverShim* drv);

// -> Source Shard: master_checklist_db.c
extern void education_master_init(void);
extern k_status education_query(const char* edu_query_str);

// -> Source Shard: ml_core.c
extern void ml_init(void);
extern k_status ml_matrix_multiply(MatrixShard* A, MatrixShard* B, MatrixShard* C);
extern k_status ml_train_shard(void* tensor_data, u32 epochs);

// -> Source Shard: mod_loader.c
extern void mod_loader_init(void);
extern k_status ins_shard(const char* name, void* elf_data, usize size);
extern k_status rm_shard(const char* name);

// -> Source Shard: molt_shard.c
extern void molt_init(void);
extern k_status molt_spawn_agent(AgentTaskType type);
extern void molt_sync_agents(void);

// -> Source Shard: namespace_shard.c
extern void namespace_init(void);
extern k_status namespace_create(const char* name, NamespaceType type);
extern bool_t namespace_visible(u32 ns_id, u32 target_id);

// -> Source Shard: ncert_shard.c
extern void ncert_init(void);
extern k_status ncert_physics_sim(const char* experiment);
extern k_status ncert_chem_orbit(u32 atomic_num);

// -> Source Shard: net_firewall.c
extern void firewall_init(void);
extern k_status firewall_add_rule(u32 src, u16 sport, u16 dport, u8 proto, RuleAction act);
extern RuleAction firewall_process_packet(u32 src, u16 sport, u16 dport, u8 proto);

// -> Source Shard: net.c
extern i32 net_socket(u8 proto);
extern i32 net_connect(i32 sockfd, u32 dst_ip, u16 dst_port);
extern i64 net_send(i32 sockfd, const void* buf, usize len);
extern i32 net_close(i32 sockfd);
extern void net_init(void);
extern void net_audit(void);

// -> Source Shard: omni_shell.c
extern void start_shell_zenith(void);
extern int main(void);

// -> Source Shard: oom_killer.c
extern void oom_killer_init(void);
extern k_status oom_execute_industrial_sweep(void);

// -> Source Shard: panic_shard.c
extern void sigma_panic(const char* msg, u64 rip, u64 rsp);
extern void sigma_assert(bool_t condition, const char* msg);

// -> Source Shard: personalizer.c
extern void personalizer_init(void);
extern void register_identity(const char* tag, u32 color);
extern u32 get_current_theme_color(void);

// -> Source Shard: pit.c
extern void pit_init(void);
extern void pit_irq_handler(SigmaInterruptFrame* frame);
extern u64 timer_get_ticks(void) { return g_timer.ticks; }
extern u64 timer_get_ms(void)    { return g_timer.ms;    }
extern u64 timer_get_ns(void);
extern void timer_sleep_ms(u64 ms);
extern void timer_init(void);

// -> Source Shard: plugins.c
extern void sigma_plugin_list();

// -> Source Shard: posix_bridge.c
extern i64 posix_syscall_dispatch(u64 num, u64 a1, u64 a2, u64 a3, u64 a4, u64 a5);

// -> Source Shard: pqc_core.c
extern void pqc_init(void);
extern void pqc_generate_shard_key(u32 shard_id);
extern void pqc_encrypt_buffer(u32 shard_id, void* buffer, u32 len);

// -> Source Shard: procfs.c
extern void procfs_init(void);
extern i64 proc_read_stat(void* buffer, u32 len);
extern i64 proc_read_meminfo(void* buffer, u32 len);

// -> Source Shard: quantum_rcu.c
extern void rcu_read_lock(void);
extern void rcu_read_unlock(void);
extern void rcu_on_quiescent_state(u32 cpu_id);
extern void rcu_init_core(void);

// -> Source Shard: registry.c
extern void registry_init(void);
extern k_status registry_set(const char* key, const char* val);
extern const char* registry_get(const char* key);

// -> Source Shard: scheduler_ai.c
extern u8 sched_predict_priority(u64 last_dur, u8 current_prio);
extern u64 ema_predict(u64 last, u64 current);
extern void sched_update_predictor(SigmaTaskPredictor* p, u64 duration);
extern void sched_predict_audit(void);

// -> Source Shard: scheduler.c
extern void sigma_scheduler_init();
extern sigma_err_t sigma_task_create(virt_addr_t entry, sigma_u32 priority);
extern void sigma_schedule();
extern SIGMA_NORETURN void sigma_panic(const char* message);
extern     while(1);

// -> Source Shard: sci_compute_shard.c
extern void __attribute__((noinline)) execute_cooley_tukey_fft(complex_t* data, uint32_t n);
extern void sovereign_vector_dot(double* a, double* b, double* result, uint32_t len);
extern     for(uint32_t i = 0; i < len; i++);

// -> Source Shard: screen_recorder.c
extern void screen_recorder_init(void);
extern k_status screen_recorder_start(u32 w, u32 h, u32 bpp);
extern void screen_recorder_stop(void);
extern void screen_recorder_on_refresh(const void* lfb_ptr);

// -> Source Shard: shard_core.c
extern void shard_init_core(void);
extern u64 shard_create(const char* name, ShardType type, u64 base, u64 limit);
extern void shard_isolate_and_switch(u64 next_shard_id);
extern void shard_amnesic_destroy(u64 shard_id);

// -> Source Shard: shard_explorer.c
extern void shard_explorer_init(void);
extern void explorer_render_tree(void);
extern void explorer_navigate(const char* target);

// -> Source Shard: sigma_kernel.c
extern void sigma_kernel_main(void* mb2_info, u32 mb2_magic);

// -> Source Shard: sigma_std.c
extern void sigma_set_tsc_freq_mhz(u64 mhz);
extern u64 os_get_timestamp_ns(void);
extern u64 os_get_timestamp_ms(void);
extern void* sigma_memset32(void* s, u32 val, usize count);
extern void sigma_bzero(void* s, usize n);
extern int sigma_memcmp(const void* a, const void* b, usize n);
extern void sigma_strcpy_safe(char* dst, const char* src, usize max);
extern int sigma_strcmp(const char* s1, const char* s2);
extern int sigma_strcasecmp(const char* s1, const char* s2);
extern const char* sigma_strstr(const char* haystack, const char* needle);
extern usize sigma_itoa(i64 val, char* buf, usize buflen);
extern usize sigma_utohex(u64 val, char* buf, usize buflen);
extern i64 sigma_atoi64(const char* s);
extern u32 sigma_fnv1a_32(const u8* data, usize len);
extern u64 sigma_fnv1a_64(const u8* data, usize len);
extern u32 sigma_djb2(const char* s);
extern void sigma_io_wait(void);
extern u16 port_inw(u16 port);
extern void port_outw_fn(u16 port, u16 val);
extern void port_outl(u16 port, u32 val);
extern u32 port_inl(u16 port);
extern u64 sigma_rdmsr(u32 msr);
extern void sigma_wrmsr(u32 msr, u64 val);
extern CPUIDResult sigma_cpuid(u32 leaf);
extern void k_print_raw(const char* s);
extern void rb_init(RingBuffer* rb, u8* storage, u32 cap);
extern bool_t rb_push(RingBuffer* rb, u8 val);
extern bool_t rb_pop(RingBuffer* rb, u8* out);
extern u64 sigma_rand64(void);
extern u32 sigma_rand32(void);

// -> Source Shard: SigmaAI.c
extern void sigma_ai_reason(const char* prompt);
extern void sigma_ai_init();

// -> Source Shard: SigmaEmbeddedKernel.c
extern void sigma_embedded_main();
extern     while(1);
extern float sigma_embedded_pow(float b, int e);

// -> Source Shard: SigmaProfessionalKernels.c
extern void Sovereign_DisplayPrinciples(void);
extern void Sovereign_AuditSystemIntegrity(void);

// -> Source Shard: signal.c
extern void signal_init(void);
extern k_status signal_deliver(u32 tid, u32 signum);
extern void signal_register_handler(u32 tid, u32 signum, u64 handler);

// -> Source Shard: sound_core.c
extern void sound_core_init(void);
extern void sound_play_alert(u32 type);
extern void sound_master_voice(const char* msg);

// -> Source Shard: sovereign_auto.c
extern void sauto_init(void);
extern k_status sauto_parse_dsl(const char* dsl);
extern void sauto_register_workflow(u32 event, u32 action);
extern void sauto_trigger_event(u32 event_id);
extern void sauto_audit(void);

// -> Source Shard: sovereign_bpf.c
extern u64 sovereign_bpf_exec(u32 prog_id, u64 ctx);
extern void sovereign_bpf_init(void);
extern void sovereign_bpf_load(u64 id, SBPFInst* instructions, u32 count);

// -> Source Shard: sovereign_ring.c
extern void sring_init(void);
extern void sring_process_submissions(void);

// -> Source Shard: SovereignAetherAbsorption.c
extern void SovereignAetherAbsorber_init(SovereignAetherAbsorber* self);
extern void SovereignAetherAbsorber_DeploySovereignUnity(SovereignAetherAbsorber* self);

// -> Source Shard: SovereignAetherOrchestrator.c
extern void SovereignAetherOrchestrator_init(SovereignAetherOrchestrator* self);
extern void SovereignAetherOrchestrator_RouteMission(SovereignAetherOrchestrator* self, const char* mission);
extern void SovereignAetherOrchestrator_DeepThinkMode(SovereignAetherOrchestrator* self);

// -> Source Shard: SovereignAetherSentinel.c
extern void SovereignAetherSentinel_init(SovereignAetherSentinel* self);
extern void SovereignAetherSentinel_HandleTrap(SovereignAetherSentinel* self, sigma_u64 trap_id, sigma_u64 rip);
extern void SovereignAetherSentinel_ResolveLastError(SovereignAetherSentinel* self, const char* shard_id, sigma_u64 error_code);
extern void SovereignAetherSentinel_DeepSanitize(SovereignAetherSentinel* self);
extern void SovereignAetherSentinel_AuditIntegrity(SovereignAetherSentinel* self);

// -> Source Shard: SovereignAetherShardLoader.c
extern void SovereignSOD_MapShard(SovereignSOD* self, const char* shard_id);
extern void SovereignSOD_UnmapShard(SovereignSOD* self, const char* shard_id);
extern void SovereignSOD_ExecuteMultimedia(SovereignSOD* self);
extern void SovereignSOD_ExecuteSecurityAudit(SovereignSOD* self);
extern void SovereignSOD_ExecuteGaming(SovereignSOD* self);
extern void SovereignSOD_ExecutePrivacy(SovereignSOD* self);
extern int main(int argc, char** argv);

// -> Source Shard: SovereignAIKernelZenith.c
extern void start_aikernel_zenith(void);
extern int main(void);

// -> Source Shard: SovereignAmnesicShard.c
extern void SovereignAmnesicShard_init(SovereignAmnesicShard* self);
extern void SovereignAmnesicShard_StartAmnesicSession(SovereignAmnesicShard* self);
extern void SovereignAmnesicShard_SecureSiliconExit(SovereignAmnesicShard* self);
extern void SovereignAmnesicShard_PerformSiliconWipe(SovereignAmnesicShard* self);
extern void SovereignAmnesicShard_KillMetadataShards(SovereignAmnesicShard* self);

// -> Source Shard: SovereignCoreUtils.c
extern void sigma_hexdump(const void* ptr, sigma_size_t len);
extern void sigma_strcpy(char* dest, const char* src, sigma_size_t maxlen);
extern void sigma_u64_to_str(sigma_u64 val, char* buf, sigma_size_t buflen);
extern int main(void);

// -> Source Shard: SovereignDiagnosticsZenith.c
extern void start_diagnostic_zenith(void);
extern int main(void);

// -> Source Shard: SovereignFileSystemZenith.c
extern void start_vfs_zenith(void);
extern int main(void);

// -> Source Shard: SovereignForensicMatrix.c
extern int main(void);

// -> Source Shard: SovereignHTTPServer.c
extern void sovereign_http_start(int port);
extern void core_module_handler(const char* verb, const char* path);
extern int main_http() { // Renamed or entry point

// -> Source Shard: SovereignHypervisorZenith.c
extern void start_hypervisor_zenith(void);
extern int main(void);

// -> Source Shard: SovereignKernelPrinciples.c
extern void Sovereign_DisplayPrinciples(void);
extern void Sovereign_AuditSystemIntegrity(void);

// -> Source Shard: SovereignKernelZenith.c
extern void sigma_hw_wipe_page(sigma_u64 addr);
extern sigma_i64 sovereign_syscall_dispatch(SovereignSyscall call, sigma_u64 arg1, sigma_u64 arg2);
extern void start_kernel_zenith(void);
extern int main(void);

// -> Source Shard: SovereignLatticePQC.c
extern void start_security_zenith(void);
extern int main(void);

// -> Source Shard: SovereignMachIPC.c
extern mach_port_t sovereign_mach_port_allocate();
extern void sovereign_mach_msg_send(mach_msg_header_t* header, void* data, mach_msg_size_t size);
extern void sovereign_mach_msg_receive(mach_port_t port, mach_msg_header_t* header, void* buffer, mach_msg_size_t buffer_size);

// -> Source Shard: SovereignMemoryRAII.c
extern void SovereignMemoryRAII_TestHarness(void);

// -> Source Shard: SovereignMemoryZenith.c
extern void start_memory_zenith(void);
extern int main(void);

// -> Source Shard: SovereignML.c
extern int main(void);

// -> Source Shard: SovereignNamespace.c
extern void SovereignNamespace_mount(SovereignNamespace* self, const char* remote_shard, const char* local_view);
extern void SovereignNamespace_init(SovereignNamespace* self, const char* view_id);
extern void SovereignNamespace_execute_sharded_mission(SovereignNamespace* self);

// -> Source Shard: SovereignNetMesh.c
extern void start_net_zenith(void);
extern int main(void);

// -> Source Shard: SovereignOffensiveShard.c
extern void SovereignOffensive_CrushLinux(void);
extern void SovereignOffensive_CrushWindows(void);
extern void SovereignOffensive_NeutronAudit(void);

// -> Source Shard: SovereignOmniShard.c
extern void SovereignScheduler_init(SovereignScheduler* s);
extern void SovereignScheduler_MultilevelFeedbackQueue(SovereignScheduler* s);
extern void SovereignScheduler_RealTimeDeadlineSchedule(SovereignScheduler* s);
extern void SovereignScheduler_audit(const SovereignScheduler* s);
extern void SovereignCloud_init(SovereignCloudOrchestrator* c);
extern void SovereignCloud_ElasticShardScale(SovereignCloudOrchestrator* c, int nodeCount);
extern void SovereignCloud_VirtualVPCIsolation(SovereignCloudOrchestrator* c, const char* tenantId);
extern void SovereignCloud_audit(const SovereignCloudOrchestrator* c);
extern void SovereignUI_init(SovereignUIEngine* u);
extern void SovereignUI_RenderSovereignDOM(SovereignUIEngine* u, const char* markup);
extern void SovereignUI_ApplyZenithCSS(SovereignUIEngine* u, const char* styling);
extern void SovereignUI_audit(const SovereignUIEngine* u);
extern void SovereignNet_init(SovereignNetZenith* n);
extern void SovereignNet_ZeroTrustHandshake(SovereignNetZenith* n);
extern void SovereignNet_RecursiveDNSNode(SovereignNetZenith* n, const char* domain);
extern void SovereignNet_audit(const SovereignNetZenith* n);

// -> Source Shard: SovereignPersonalizerZenith.c
extern void start_personalizer_demo(void);
extern int main(void);

// -> Source Shard: SovereignProcessManager.c
extern sigma_status SovereignProcess_Spawn(const char* image_shard);
extern void SovereignProcess_Kill(sigma_u32 pid);
extern void SovereignProcess_IsolateNamespace(const char* ns_hash);
extern void SovereignProcess_CompetitorCrush(const char* os_name);
extern void SovereignProcess_Audit(void);
extern void Sovereign_PM_Main(void);

// -> Source Shard: SovereignQuantumKernel.c
extern void SovereignQuantumKernel_init(SovereignQuantumKernel* self);
extern void SovereignQuantumKernel_InitializeQuantumSync(SovereignQuantumKernel* self);
extern void SovereignQuantumKernel_ExecuteKyberTaskSlice(SovereignQuantumKernel* self);
extern void SovereignQuantumKernel_VerifySiliconIntegrity(SovereignQuantumKernel* self);

// -> Source Shard: SovereignQuantumShard.c
extern void SovereignQuantum_LatticeInit(void);
extern sigma_status SovereignQuantum_GenerateKey(void* pk, void* sk);
extern void SovereignQuantum_AuditSecurity(void);

// -> Source Shard: SovereignSearch.c
extern int main(void);

// -> Source Shard: SovereignStyleZenith.c
extern void SovereignStyleZenith_init(SovereignStyleZenith* self);
extern void SovereignStyleZenith_ApplyFluentAquaFusion(SovereignStyleZenith* self);
extern void SovereignStyleZenith_ApplyCenturionGoldPalette(SovereignStyleZenith* self);
extern void SovereignStyleZenith_ShardDynamicDesign(SovereignStyleZenith* self, const char* designerId);
extern void SovereignStyleZenith_Personalize(SovereignStyleZenith* self, const char* user_pref);

// -> Source Shard: SovereignSuperCalculator.c
extern void start_calc_zenith(void);
extern int main(void);

// -> Source Shard: SovereignUnityShard.c
extern void SovereignUnity_init(SovereignUnityShard* self);
extern void SovereignUnity_SovereignAudit(SovereignUnityShard* self);
extern void SovereignUnity_OptimizeSilicon(SovereignUnityShard* self);
extern int main();

// -> Source Shard: SovereignVoiceShard.c
extern int main(void);

// -> Source Shard: syscall.c
extern void syscall_handler(SigmaInterruptFrame* frame);
extern void syscall_init(void);
extern u64 sched_current_tid(void);

// -> Source Shard: test_framework_shard.c
extern void __attribute__((noinline)) assert_eq_memory(void* a, void* b, uint32_t size, const char* name);
extern     for(uint32_t i = 0; i < size; i++);

// -> Source Shard: thp_shard.c
extern void thp_init(void);
extern k_status thp_merge_shards(u64 start_pfn);

// -> Source Shard: user_manager.c
extern void user_manager_init(void);
extern u32 user_get_current_uid(void);
extern bool_t user_is_sovereign(u32 uid);
extern void user_switch_identity(u32 uid);

// -> Source Shard: vfs.c
extern void vfs_init(void);
extern i32 vfs_open(const char* path, u32 flags, u32 mode);
extern i64 vfs_read(i32 fd, void* buf, usize count);
extern i64 vfs_write(i32 fd, const void* buf, usize count);
extern i32 vfs_close(i32 fd);
extern i32 vfs_mkdir(const char* path, u32 mode);
extern i32 vfs_stat(const char* path, VFileStat* st);
extern void vfs_audit(void);

// -> Source Shard: voice_zenith.c
extern void voice_init(void);
extern void voice_speak_alert(const char* msg);
extern k_status voice_configure(u32 rate, u16 pitch, u8 vol);

// -> Source Shard: web_bridge.c
extern void web_bridge_init(void);
extern void web_send_packet(const char* msg);
extern void web_sync_vga(void);
extern k_status web_process_request(u32 req_id);

// -> Source Shard: zen_editor.c
extern void zen_editor_init(void);
extern void zen_editor_highlight(u32 line_idx);
extern void zen_editor_open_shard(const char* path);
extern void editor_personalize_theme(u32 bg_color, u32 fg_color);

// -> Source Shard: zram_shard.c
extern void zram_init(void);
extern k_status zram_compress_shard(u64 pfn);

#endif // SOVEREIGN_OMNI_API_H

