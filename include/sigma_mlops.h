/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS MLOps Platform Implementation
 * =======================================
 * Complete MLOps lifecycle management:
 * - Experiment tracking
 * - Model registry
 * - Data versioning
 * - Pipeline orchestration
 * - Model monitoring
 * - A/B testing
 * - Feature store
 */

#ifndef SIGMA_MLOPS_H
#define SIGMA_MLOPS_H

#include "sigma_ml_algorithms.h"
#include "sigma_neural_networks.h"
#include <time.h>

// ==================== EXPERIMENT TRACKING ====================

typedef struct {
    char experiment_id[64];
    char name[256];
    char description[1024];
    time_t start_time;
    time_t end_time;
    char status[16]; // "running", "completed", "failed"
    
    // Hyperparameters
    char params_json[10000];
    
    // Metrics
    double train_loss;
    double val_loss;
    double test_loss;
    double train_accuracy;
    double val_accuracy;
    double test_accuracy;
    double f1_score;
    double precision;
    double recall;
    double auc_roc;
    
    // Artifacts
    char model_path[1024];
    char dataset_version[64];
    char code_version[64];
    char git_commit[64];
    
    // Tags
    char tags[10][64];
    uint32_t n_tags;
} SigmaExperiment;

typedef struct {
    SigmaExperiment* experiments;
    uint32_t capacity;
    uint32_t count;
    char tracking_uri[1024];
    char experiment_name[256];
} SigmaExperimentTracker;

SigmaExperimentTracker* sigma_mlflow_create(const char* tracking_uri, const char* experiment_name);
SigmaExperiment* sigma_mlflow_start_run(SigmaExperimentTracker* tracker, const char* run_name);
void sigma_mlflow_log_param(SigmaExperiment* run, const char* key, const char* value);
void sigma_mlflow_log_params(SigmaExperiment* run, const char* params_json);
void sigma_mlflow_log_metric(SigmaExperiment* run, const char* key, double value, uint32_t step);
void sigma_mlflow_log_metrics(SigmaExperiment* run, double train_loss, double val_loss, 
                              double train_acc, double val_acc, uint32_t epoch);
void sigma_mlflow_log_artifact(SigmaExperiment* run, const char* local_path, const char* artifact_path);
void sigma_mlflow_log_model(SigmaExperiment* run, void* model, const char* model_type);
void sigma_mlflow_set_tag(SigmaExperiment* run, const char* key, const char* value);
void sigma_mlflow_end_run(SigmaExperiment* run, const char* status);
SigmaExperiment** sigma_mlflow_search_runs(SigmaExperimentTracker* tracker, 
                                           const char* filter_string, 
                                           uint32_t* n_results);
void sigma_mlflow_destroy(SigmaExperimentTracker* tracker);

// ==================== MODEL REGISTRY ====================

typedef enum {
    SIGMA_MODEL_STAGING,
    SIGMA_MODEL_PRODUCTION,
    SIGMA_MODEL_ARCHIVED
} SigmaModelStage;

typedef struct {
    char model_name[256];
    char version[32];
    SigmaModelStage stage;
    char description[1024];
    char run_id[64];
    char source_path[1024];
    time_t creation_time;
    char user_id[256];
    char tags[10][128];
    uint32_t n_tags;
    
    // Model details
    char model_type[64]; // "sklearn", "tensorflow", "pytorch", "xgboost"
    char framework_version[32];
    char signature_json[5000];
    char input_schema[5000];
    char output_schema[5000];
} SigmaModelVersion;

typedef struct {
    char model_name[256];
    char description[1024];
    SigmaModelVersion* versions;
    uint32_t n_versions;
    uint32_t latest_version;
    char tags[10][128];
    uint32_t n_tags;
} SigmaRegisteredModel;

typedef struct {
    SigmaRegisteredModel* models;
    uint32_t capacity;
    uint32_t count;
    char registry_uri[1024];
} SigmaModelRegistry;

SigmaModelRegistry* sigma_model_registry_create(const char* registry_uri);
SigmaRegisteredModel* sigma_register_model(SigmaModelRegistry* registry, 
                                           const char* model_name,
                                           const char* run_id,
                                           const char* source_path);
SigmaModelVersion* sigma_create_model_version(SigmaModelRegistry* registry,
                                              const char* model_name,
                                              const char* source_path,
                                              const char* run_id);
void sigma_transition_model_stage(SigmaModelRegistry* registry,
                                  const char* model_name,
                                  const char* version,
                                  SigmaModelStage new_stage);
SigmaModelVersion* sigma_get_latest_model_version(SigmaModelRegistry* registry,
                                                  const char* model_name,
                                                  const char* stage);
void sigma_model_registry_destroy(SigmaModelRegistry* registry);

// ==================== DATA VERSIONING (DVC) ====================

typedef struct {
    char file_path[1024];
    char md5_hash[33];
    uint64_t size_bytes;
    time_t modification_time;
    char remote_url[1024];
} SigmaDataFile;

typedef struct {
    char version_id[64];
    char commit_message[1024];
    time_t timestamp;
    char author[256];
    SigmaDataFile* files;
    uint32_t n_files;
    char parent_version[64];
    char tags[10][64];
    uint32_t n_tags;
} SigmaDataVersion;

typedef struct {
    char repo_path[1024];
    char remote_storage[1024];
    SigmaDataVersion* versions;
    uint32_t capacity;
    uint32_t count;
    SigmaDataVersion* current_version;
} SigmaDataVersionControl;

SigmaDataVersionControl* sigma_dvc_init(const char* repo_path);
void sigma_dvc_add(SigmaDataVersionControl* dvc, const char* file_path);
void sigma_dvc_commit(SigmaDataVersionControl* dvc, const char* message);
void sigma_dvc_push(SigmaDataVersionControl* dvc);
void sigma_dvc_pull(SigmaDataVersionControl* dvc);
void sigma_dvc_checkout(SigmaDataVersionControl* dvc, const char* version_id);
SigmaDataVersion* sigma_dvc_log(SigmaDataVersionControl* dvc, uint32_t* n_versions);
void sigma_dvc_destroy(SigmaDataVersionControl* dvc);

// ==================== PIPELINE ORCHESTRATION ====================

typedef enum {
    SIGMA_STEP_DATA_INGESTION,
    SIGMA_STEP_DATA_PREPROCESSING,
    SIGMA_STEP_FEATURE_ENGINEERING,
    SIGMA_STEP_MODEL_TRAINING,
    SIGMA_STEP_MODEL_VALIDATION,
    SIGMA_STEP_MODEL_DEPLOYMENT,
    SIGMA_STEP_MONITORING
} SigmaPipelineStepType;

typedef struct SigmaPipelineStep {
    char step_id[64];
    char name[256];
    SigmaPipelineStepType type;
    char command[2048];
    char parameters_json[5000];
    char inputs[10][256];
    char outputs[10][256];
    uint32_t n_inputs;
    uint32_t n_outputs;
    
    // Execution info
    char status[16]; // "pending", "running", "completed", "failed"
    time_t start_time;
    time_t end_time;
    char logs[10000];
    int exit_code;
    
    // Dependencies
    struct SigmaPipelineStep** dependencies;
    uint32_t n_dependencies;
    
    struct SigmaPipelineStep* next;
} SigmaPipelineStep;

typedef struct {
    char pipeline_id[64];
    char name[256];
    char description[1024];
    SigmaPipelineStep* steps;
    uint32_t n_steps;
    char schedule[64]; // cron expression
    bool is_active;
    
    // Execution tracking
    uint32_t total_runs;
    uint32_t successful_runs;
    uint32_t failed_runs;
    double avg_duration;
} SigmaMLPipeline;

typedef struct {
    SigmaMLPipeline* pipelines;
    uint32_t capacity;
    uint32_t count;
    char orchestrator_uri[1024];
} SigmaPipelineOrchestrator;

SigmaPipelineOrchestrator* sigma_pipeline_orchestrator_create(const char* uri);
SigmaMLPipeline* sigma_pipeline_create(SigmaPipelineOrchestrator* orch,
                                       const char* name,
                                       const char* description);
SigmaPipelineStep* sigma_pipeline_add_step(SigmaMLPipeline* pipeline,
                                           const char* name,
                                           SigmaPipelineStepType type,
                                           const char* command);
void sigma_pipeline_add_dependency(SigmaPipelineStep* step, SigmaPipelineStep* dependency);
void sigma_pipeline_schedule(SigmaMLPipeline* pipeline, const char* cron_expression);
void sigma_pipeline_run(SigmaMLPipeline* pipeline);
void sigma_pipeline_run_async(SigmaMLPipeline* pipeline);
char* sigma_pipeline_get_logs(SigmaMLPipeline* pipeline, const char* step_id);
void sigma_pipeline_orchestrator_destroy(SigmaPipelineOrchestrator* orch);

// ==================== MODEL MONITORING ====================

typedef struct {
    time_t timestamp;
    double prediction_value;
    double actual_value;
    double prediction_latency_ms;
    char input_hash[64];
    char output_hash[64];
    bool is_anomaly;
    double confidence_score;
} SigmaPredictionLog;

typedef struct {
    char metric_name[64];
    double value;
    time_t timestamp;
    char tags[5][64];
    uint32_t n_tags;
} SigmaModelMetric;

typedef struct {
    char model_name[256];
    char model_version[32];
    char endpoint[256];
    time_t deployment_time;
    
    // Performance metrics
    double requests_per_second;
    double avg_latency_ms;
    double p50_latency_ms;
    double p95_latency_ms;
    double p99_latency_ms;
    double error_rate;
    uint64_t total_requests;
    uint64_t total_errors;
    
    // Drift detection
    double data_drift_score;
    double concept_drift_score;
    bool drift_detected;
    char drift_type[32];
    
    // Alerts
    char alerts[10][256];
    uint32_t n_alerts;
} SigmaModelDeployment;

typedef struct {
    SigmaModelDeployment* deployments;
    uint32_t capacity;
    uint32_t count;
    SigmaPredictionLog* recent_predictions;
    uint32_t prediction_capacity;
    uint32_t prediction_count;
    
    // Monitoring config
    double drift_threshold;
    double latency_threshold_ms;
    double error_rate_threshold;
    uint32_t monitoring_window_minutes;
} SigmaModelMonitor;

SigmaModelMonitor* sigma_model_monitor_create(const char* model_name,
                                              const char* model_version,
                                              const char* endpoint);
void sigma_monitor_log_prediction(SigmaModelMonitor* monitor, 
                                SigmaPredictionLog* prediction);
void sigma_monitor_record_metric(SigmaModelMonitor* monitor,
                                 const char* metric_name,
                                 double value);
void sigma_monitor_check_drift(SigmaModelMonitor* monitor);
void sigma_monitor_alert(SigmaModelMonitor* monitor, 
                         const char* alert_type,
                         const char* message);
SigmaModelMetric** sigma_monitor_get_metrics(SigmaModelMonitor* monitor,
                                             const char* metric_name,
                                             time_t start_time,
                                             time_t end_time,
                                             uint32_t* n_metrics);
void sigma_model_monitor_destroy(SigmaModelMonitor* monitor);

// ==================== A/B TESTING ====================

typedef struct {
    char test_id[64];
    char test_name[256];
    char model_a_name[256];
    char model_a_version[32];
    char model_b_name[256];
    char model_b_version[32];
    double traffic_split_a;
    double traffic_split_b;
    
    // Success metrics
    char primary_metric[64];
    double metric_threshold;
    uint32_t min_sample_size;
    double confidence_level;
    
    // Status
    char status[16]; // "running", "completed", "stopped"
    time_t start_time;
    time_t end_time;
    
    // Results
    double model_a_metric;
    double model_b_metric;
    double p_value;
    double lift_percentage;
    char winner[256];
} SigmaABTest;

typedef struct {
    SigmaABTest* tests;
    uint32_t capacity;
    uint32_t count;
} SigmaABTestManager;

SigmaABTestManager* sigma_ab_test_manager_create(void);
SigmaABTest* sigma_ab_test_create(SigmaABTestManager* manager,
                                  const char* test_name,
                                  const char* model_a_name,
                                  const char* model_a_version,
                                  const char* model_b_name,
                                  const char* model_b_version,
                                  double traffic_split);
void sigma_ab_test_set_metric(SigmaABTest* test, const char* metric_name, 
                              double threshold, uint32_t min_samples);
void sigma_ab_test_assign_variant(SigmaABTest* test, const char* user_id, 
                                  char* variant_out);
void sigma_ab_test_record_outcome(SigmaABTest* test, const char* variant, 
                                  double metric_value);
void sigma_ab_test_stop(SigmaABTest* test);
SigmaABTest* sigma_ab_test_get_results(SigmaABTestManager* manager, const char* test_id);
void sigma_ab_test_manager_destroy(SigmaABTestManager* manager);

// ==================== FEATURE STORE ====================

typedef struct {
    char feature_name[256];
    char feature_type[32]; // "numeric", "categorical", "embedding", "time"
    char description[1024];
    char data_type[16]; // "float", "int", "string", "array"
    char entity_key[64];
    uint32_t dimension;
    
    // Statistics
    double mean;
    double std;
    double min;
    double max;
    uint32_t null_count;
    
    // Versioning
    char version[32];
    time_t created_at;
    time_t updated_at;
} SigmaFeatureDefinition;

typedef struct {
    char entity_id[256];
    char entity_type[64];
    SigmaVector* features;
    char feature_names[50][256];
    uint32_t n_features;
    time_t timestamp;
    char event_id[64];
} SigmaFeatureVector;

typedef struct {
    char store_name[256];
    SigmaFeatureDefinition* definitions;
    uint32_t n_definitions;
    
    // Online store (low latency)
    SigmaFeatureVector* online_features;
    uint32_t online_capacity;
    uint32_t online_count;
    
    // Offline store (historical)
    char offline_storage_path[1024];
    
    // Materialization
    time_t last_materialization;
    uint32_t materialization_interval_hours;
} SigmaFeatureStore;

SigmaFeatureStore* sigma_feature_store_create(const char* store_name);
void sigma_feature_store_register_feature(SigmaFeatureStore* store,
                                          SigmaFeatureDefinition* feature);
void sigma_feature_store_ingest(SigmaFeatureStore* store,
                                const char* entity_id,
                                const char* entity_type,
                                const char* feature_name,
                                double value);
SigmaFeatureVector* sigma_feature_store_get_online_features(SigmaFeatureStore* store,
                                                            const char* entity_id,
                                                            char** feature_names,
                                                            uint32_t n_features);
SigmaMatrix* sigma_feature_store_get_offline_features(SigmaFeatureStore* store,
                                                      const char* entity_type,
                                                      time_t start_time,
                                                      time_t end_time);
void sigma_feature_store_materialize(SigmaFeatureStore* store);
void sigma_feature_store_destroy(SigmaFeatureStore* store);

// ==================== MODEL SERVING ====================

typedef struct {
    char endpoint_name[256];
    char model_name[256];
    char model_version[32];
    char endpoint_uri[1024];
    
    // Scaling
    uint32_t min_replicas;
    uint32_t max_replicas;
    uint32_t current_replicas;
    double target_cpu_utilization;
    
    // Performance
    double requests_per_second;
    double avg_latency_ms;
    double error_rate;
} SigmaModelEndpoint;

typedef struct {
    SigmaModelEndpoint* endpoints;
    uint32_t capacity;
    uint32_t count;
    char serving_platform[32]; // "kubernetes", "sagemaker", "vertex", "custom"
} SigmaModelServing;

SigmaModelServing* sigma_model_serving_create(const char* platform);
SigmaModelEndpoint* sigma_model_serving_deploy(SigmaModelServing* serving,
                                              const char* endpoint_name,
                                              const char* model_name,
                                              const char* model_version,
                                              uint32_t min_replicas,
                                              uint32_t max_replicas);
void sigma_model_serving_scale(SigmaModelEndpoint* endpoint, uint32_t n_replicas);
void sigma_model_serving_undeploy(SigmaModelServing* serving, const char* endpoint_name);
SigmaVector* sigma_model_serving_predict(SigmaModelEndpoint* endpoint, SigmaMatrix* input);
void sigma_model_serving_destroy(SigmaModelServing* serving);

// ==================== BATCH INFERENCE ====================

typedef struct {
    char job_id[64];
    char job_name[256];
    char model_name[256];
    char model_version[32];
    char input_path[1024];
    char output_path[1024];
    char format[16]; // "csv", "parquet", "json"
    
    // Status
    char status[16]; // "pending", "running", "completed", "failed"
    uint32_t total_records;
    uint32_t processed_records;
    uint32_t failed_records;
    double progress_percentage;
    time_t start_time;
    time_t end_time;
    
    // Compute
    uint32_t n_workers;
    char compute_type[16]; // "cpu", "gpu"
} SigmaBatchInferenceJob;

typedef struct {
    SigmaBatchInferenceJob* jobs;
    uint32_t capacity;
    uint32_t count;
} SigmaBatchInferenceManager;

SigmaBatchInferenceManager* sigma_batch_inference_create(void);
SigmaBatchInferenceJob* sigma_batch_inference_submit(SigmaBatchInferenceManager* manager,
                                                    const char* job_name,
                                                    const char* model_name,
                                                    const char* model_version,
                                                    const char* input_path,
                                                    const char* output_path);
void sigma_batch_inference_monitor(SigmaBatchInferenceJob* job);
char* sigma_batch_inference_get_logs(SigmaBatchInferenceJob* job);
void sigma_batch_inference_manager_destroy(SigmaBatchInferenceManager* manager);

// ==================== ML COMMANDS ====================

void sigma_mlops_experiment_create(const char* name, const char* description);
void sigma_mlops_experiment_log(const char* run_id, const char* metric, double value);
void sigma_mlops_model_register(const char* model_name, const char* version, const char* stage);
void sigma_mlops_model_deploy(const char* model_name, const char* version, const char* endpoint);
void sigma_mlops_pipeline_run(const char* pipeline_name);
void sigma_mlops_feature_register(const char* name, const char* type, const char* entity);
void sigma_mlops_monitor_enable(const char* model_name, const char* version);
void sigma_mlops_ab_test_create(const char* test_name, const char* model_a, const char* model_b);

#endif // SIGMA_MLOPS_H

