/*
 * SigmaOS Advanced AI: Meta Learning, GNN, Federated Learning, etc.
 * ================================================================
 * Cutting-edge AI techniques:
 * - Meta Learning (MAML, ProtoNets)
 * - Graph Neural Networks (GCN, GAT, GraphSAGE)
 * - Federated Learning
 * - Neural Architecture Search (NAS)
 * - Continual Learning
 * - Adversarial Training
 * - Self-Supervised Learning
 * - Contrastive Learning
 */

#ifndef SIGMA_ADVANCED_AI_H
#define SIGMA_ADVANCED_AI_H

#include "sigma_neural_networks.h"
#include "sigma_ml_algorithms.h"

// ==================== META LEARNING ====================

// MAML (Model-Agnostic Meta-Learning)
typedef struct {
    // Base model (the model to be meta-learned)
    SigmaMLP* base_model;
    
    // Meta-learning parameters
    double meta_lr; // Outer loop learning rate
    double inner_lr; // Inner loop learning rate
    uint32_t n_inner_steps; // Number of gradient steps in inner loop
    
    // First-order approximation (FOMAML)
    bool first_order;
    
    // Task distribution
    uint32_t n_tasks_per_batch; // Number of tasks per meta-batch
    uint32_t k_shot; // K-shot learning (k support examples per task)
    uint32_t n_query; // Number of query examples per task
    
    // Task parameters
    uint32_t input_dim;
    uint32_t output_dim;
    
    // Metrics
    double meta_loss;
    double* task_losses;
} SigmaMAML;

SigmaMAML* sigma_maml_create(SigmaMLP* base_model,
                            double meta_lr,
                            double inner_lr,
                            uint32_t n_inner_steps,
                            bool first_order);

void sigma_maml_outer_update(SigmaMAML* maml,
                            SigmaMatrix** support_x, // List of task support sets
                            SigmaVector** support_y,
                            SigmaMatrix** query_x,   // List of task query sets
                            SigmaVector** query_y,
                            uint32_t n_tasks);

void sigma_maml_inner_update(SigmaMAML* maml,
                            SigmaMatrix* support_x,
                            SigmaVector* support_y,
                            SigmaMLP* adapted_model);

void sigma_maml_train(SigmaMAML* maml,
                     void** tasks, // Task sampling function
                     uint32_t n_iterations);

void sigma_maml_evaluate(SigmaMAML* maml,
                        void** test_tasks,
                        uint32_t n_test_tasks,
                        double* avg_accuracy);

void sigma_maml_save(SigmaMAML* maml, const char* path);
void sigma_maml_load(SigmaMAML* maml, const char* path);
void sigma_maml_destroy(SigmaMAML* maml);

// Prototypical Networks
typedef struct {
    // Embedding network
    SigmaCNN* encoder; // For images
    SigmaMLP* embedder; // For other data
    
    // Prototype parameters
    uint32_t embedding_dim;
    uint32_t n_classes_per_task;
    uint32_t k_shot;
    uint32_t n_query;
    
    // Distance metric
    char distance[16]; // "euclidean", "cosine"
    
    // Prototypes [n_classes, embedding_dim]
    SigmaMatrix** prototypes; // Per task
    
    // Metrics
    double accuracy;
} SigmaPrototypicalNetworks;

SigmaPrototypicalNetworks* sigma_proto_net_create(SigmaCNN* encoder,
                                                uint32_t embedding_dim,
                                                uint32_t n_classes,
                                                uint32_t k_shot,
                                                const char* distance);

void sigma_proto_net_compute_prototypes(SigmaPrototypicalNetworks* proto,
                                       SigmaMatrix* support_x,
                                       SigmaVector* support_y,
                                       SigmaMatrix* prototypes);

double sigma_proto_net_predict(SigmaPrototypicalNetworks* proto,
                              double* query_embedding,
                              SigmaMatrix* prototypes,
                              uint32_t* predicted_class);

void sigma_proto_net_train(SigmaPrototypicalNetworks* proto,
                          void** episodes, // List of few-shot episodes
                          uint32_t n_episodes,
                          uint32_t epochs);

void sigma_proto_net_destroy(SigmaPrototypicalNetworks* proto);

// Relation Networks
typedef struct {
    // Embedding module
    SigmaCNN* encoder;
    
    // Relation module (learns to compare embeddings)
    SigmaMLP* relation_module;
    
    uint32_t embedding_dim;
    uint32_t n_classes_per_task;
    uint32_t k_shot;
} SigmaRelationNetworks;

SigmaRelationNetworks* sigma_relation_net_create(SigmaCNN* encoder,
                                               uint32_t embedding_dim,
                                               uint32_t n_classes,
                                               uint32_t k_shot);

void sigma_relation_net_train(SigmaRelationNetworks* relation,
                             void** episodes,
                             uint32_t n_episodes,
                             uint32_t epochs);

void sigma_relation_net_destroy(SigmaRelationNetworks* relation);

// ==================== GRAPH NEURAL NETWORKS ====================

// Graph structure
typedef struct {
    uint32_t n_nodes;
    uint32_t n_edges;
    
    // Adjacency (can be sparse)
    double** adjacency_matrix; // [n_nodes, n_nodes]
    uint32_t** edge_index;     // [2, n_edges] - COO format
    double* edge_weights;      // [n_edges]
    
    // Node features
    SigmaMatrix* node_features; // [n_nodes, feature_dim]
    
    // Edge features (optional)
    SigmaMatrix* edge_features; // [n_edges, edge_feature_dim]
    
    // Node labels (for supervised tasks)
    SigmaVector* labels;
    
    // Graph-level label (for graph classification)
    double graph_label;
} SigmaGraphData;

// Graph Convolutional Network (GCN)
typedef struct {
    uint32_t n_layers;
    uint32_t* hidden_dims;
    uint32_t output_dim;
    
    // Layer parameters
    SigmaMatrix** weights; // [n_layers]
    SigmaVector** biases;  // [n_layers]
    
    // Activation
    char activation[16]; // "relu", "tanh", etc.
    
    // Dropout
    double dropout_rate;
    
    // Normalization
    bool use_layer_norm;
    
    // Task type
    char task_type[16]; // "node", "edge", "graph"
} SigmaGCN;

SigmaGCN* sigma_gcn_create(uint32_t input_dim,
                          uint32_t* hidden_dims,
                          uint32_t n_layers,
                          uint32_t output_dim,
                          const char* task_type);

SigmaMatrix* sigma_gcn_forward(SigmaGCN* gcn,
                              SigmaGraphData* graph,
                              SigmaMatrix* node_features);

void sigma_gcn_propagate(SigmaGCN* gcn,
                        SigmaMatrix* adjacency_normalized,
                        SigmaMatrix* node_features,
                        uint32_t layer_idx,
                        SigmaMatrix* output);

void sigma_gcn_train(SigmaGCN* gcn,
                    SigmaGraphData** graphs,
                    uint32_t n_graphs,
                    uint32_t epochs);

void sigma_gcn_destroy(SigmaGCN* gcn);

// Graph Attention Network (GAT)
typedef struct {
    uint32_t n_layers;
    uint32_t* hidden_dims;
    uint32_t n_heads; // Multi-head attention
    uint32_t output_dim;
    
    // Attention parameters
    SigmaMatrix*** attention_weights; // [n_layers, n_heads]
    SigmaVector*** attention_biases;
    
    // LeakyReLU negative slope
    double leaky_relu_slope;
    
    // Dropout
    double dropout_rate;
    double attn_dropout_rate;
    
    // Output averaging (for multi-head)
    bool average_last_layer;
    
    char task_type[16];
} SigmaGAT;

SigmaGAT* sigma_gat_create(uint32_t input_dim,
                          uint32_t* hidden_dims,
                          uint32_t n_layers,
                          uint32_t n_heads,
                          uint32_t output_dim,
                          const char* task_type);

SigmaMatrix* sigma_gat_forward(SigmaGAT* gat,
                              SigmaGraphData* graph,
                              SigmaMatrix* node_features);

void sigma_gat_compute_attention(SigmaGAT* gat,
                                uint32_t layer,
                                uint32_t head,
                                SigmaMatrix* node_features,
                                uint32_t** edge_index,
                                double* attention_coeffs);

void sigma_gat_train(SigmaGAT* gat,
                    SigmaGraphData** graphs,
                    uint32_t n_graphs,
                    uint32_t epochs);

void sigma_gat_destroy(SigmaGAT* gat);

// GraphSAGE
typedef struct {
    uint32_t n_layers;
    uint32_t* hidden_dims;
    uint32_t output_dim;
    
    // Aggregator type
    char aggregator[16]; // "mean", "sum", "max", "lstm"
    
    // LSTM aggregator (if used)
    SigmaRNN* lstm_aggregator;
    
    // Sampling
    uint32_t n_sample_neighbors; // Number of neighbors to sample
    bool use_sampling;
    
    // Layer weights
    SigmaMatrix** weights;
    SigmaVector** biases;
    
    char task_type[16];
} SigmaGraphSAGE;

SigmaGraphSAGE* sigma_graphsage_create(uint32_t input_dim,
                                      uint32_t* hidden_dims,
                                      uint32_t n_layers,
                                      uint32_t output_dim,
                                      const char* aggregator,
                                      const char* task_type);

void sigma_graphsage_sample_neighbors(SigmaGraphSAGE* sage,
                                     SigmaGraphData* graph,
                                     uint32_t node_id,
                                     uint32_t* sampled_neighbors,
                                     uint32_t* n_sampled);

SigmaMatrix* sigma_graphsage_aggregate(SigmaGraphSAGE* sage,
                                      SigmaMatrix* neighbor_features,
                                      uint32_t n_neighbors);

SigmaMatrix* sigma_graphsage_forward(SigmaGraphSAGE* sage,
                                  SigmaGraphData* graph,
                                  SigmaMatrix* node_features);

void sigma_graphsage_train(SigmaGraphSAGE* sage,
                          SigmaGraphData** graphs,
                          uint32_t n_graphs,
                          uint32_t epochs);

void sigma_graphsage_destroy(SigmaGraphSAGE* sage);

// Graph Isomorphism Network (GIN)
typedef struct {
    uint32_t n_layers;
    uint32_t* hidden_dims;
    uint32_t output_dim;
    
    // MLPs for each layer
    SigmaMLP** mlps;
    
    // Epsilon parameter (learnable or fixed)
    double* epsilon;
    bool trainable_epsilon;
    
    char task_type[16];
} SigmaGIN;

SigmaGIN* sigma_gin_create(uint32_t input_dim,
                          uint32_t* hidden_dims,
                          uint32_t n_layers,
                          uint32_t output_dim,
                          const char* task_type);

SigmaMatrix* sigma_gin_forward(SigmaGIN* gin,
                              SigmaGraphData* graph,
                              SigmaMatrix* node_features);

void sigma_gin_train(SigmaGIN* gin,
                    SigmaGraphData** graphs,
                    uint32_t n_graphs,
                    uint32_t epochs);

void sigma_gin_destroy(SigmaGIN* gin);

// Message Passing Neural Network (MPNN)
typedef struct {
    uint32_t n_message_passing_steps;
    uint32_t hidden_dim;
    uint32_t output_dim;
    
    // Message function
    SigmaMLP* message_mlp;
    
    // Update function
    SigmaMLP* update_mlp;
    
    // Readout function (for graph-level tasks)
    SigmaMLP* readout_mlp;
    
    char message_func[16]; // "mlp", "edge_nn"
    char update_func[16]; // "gru", "mlp"
    char readout_func[16]; // "sum", "mean", "max", "set2set"
} SigmaMPNN;

SigmaMPNN* sigma_mpnn_create(uint32_t node_feat_dim,
                            uint32_t edge_feat_dim,
                            uint32_t hidden_dim,
                            uint32_t n_steps,
                            uint32_t output_dim);

SigmaMatrix* sigma_mpnn_message_pass(SigmaMPNN* mpnn,
                                    SigmaGraphData* graph,
                                    SigmaMatrix* node_hidden);

SigmaMatrix* sigma_mpnn_forward(SigmaMPNN* mpnn,
                               SigmaGraphData* graph,
                               SigmaMatrix* node_features,
                               SigmaMatrix* edge_features);

void sigma_mpnn_train(SigmaMPNN* mpnn,
                     SigmaGraphData** graphs,
                     uint32_t n_graphs,
                     uint32_t epochs);

void sigma_mpnn_destroy(SigmaMPNN* mpnn);

// Graph Autoencoder
typedef struct {
    // Encoder
    SigmaGCN* encoder;
    
    // Decoder (reconstructs adjacency)
    SigmaMLP* decoder;
    
    // Embedding dimension
    uint32_t embedding_dim;
    
    // Reconstruction loss weight
    double recon_loss_weight;
} SigmaGraphAutoencoder;

SigmaGraphAutoencoder* sigma_graph_ae_create(uint32_t input_dim,
                                            uint32_t embedding_dim);

void sigma_graph_ae_encode(SigmaGraphAutoencoder* gae,
                          SigmaGraphData* graph,
                          SigmaMatrix* embeddings);

void sigma_graph_ae_decode(SigmaGraphAutoencoder* gae,
                          SigmaMatrix* embeddings,
                          SigmaMatrix* reconstructed_adj);

void sigma_graph_ae_train(SigmaGraphAutoencoder* gae,
                         SigmaGraphData** graphs,
                         uint32_t n_graphs,
                         uint32_t epochs);

void sigma_graph_ae_destroy(SigmaGraphAutoencoder* gae);

// Variational Graph Autoencoder (VGAE)
typedef struct {
    SigmaGCN* encoder_mean;
    SigmaGCN* encoder_log_var;
    
    SigmaMLP* decoder;
    
    uint32_t embedding_dim;
    double kl_weight;
} SigmaVGAE;

SigmaVGAE* sigma_vgae_create(uint32_t input_dim,
                            uint32_t embedding_dim);

void sigma_vgae_encode(SigmaVGAE* vgae,
                      SigmaGraphData* graph,
                      SigmaMatrix* mu,
                      SigmaMatrix* log_var);

SigmaMatrix* sigma_vgae_reparameterize(SigmaVGAE* vgae,
                                        SigmaMatrix* mu,
                                        SigmaMatrix* log_var);

void sigma_vgae_train(SigmaVGAE* vgae,
                     SigmaGraphData** graphs,
                     uint32_t n_graphs,
                     uint32_t epochs);

void sigma_vgae_destroy(SigmaVGAE* vgae);

// ==================== FEDERATED LEARNING ====================

typedef struct {
    // Client ID
    char client_id[64];
    
    // Local model (copy of global model)
    void* local_model; // SigmaMLP*, SigmaCNN*, etc.
    char model_type[32];
    
    // Local dataset
    SigmaDataset* local_data;
    
    // Local training config
    uint32_t local_epochs;
    uint32_t batch_size;
    double local_lr;
    
    // Differential privacy (optional)
    bool use_dp;
    double dp_epsilon;
    double dp_delta;
    double dp_clip_norm;
    
    // Secure aggregation
    bool use_secure_agg;
} SigmaFLClient;

typedef struct {
    // Global model
    void* global_model;
    char model_type[32];
    
    // Clients
    SigmaFLClient** clients;
    uint32_t n_clients;
    
    // Aggregation strategy
    char aggregation[16]; // "fedavg", "fedprox", "scaffold", "fednova"
    
    // FedProx parameter
    double mu; // Proximal term weight
    
    // Communication rounds
    uint32_t n_rounds;
    uint32_t current_round;
    
    // Client sampling
    double fraction_clients; // C (fraction of clients per round)
    
    // Model updates
    void** client_updates; // Gradient updates from clients
    double** update_weights; // Weights for aggregation
    
    // Metrics
    double* round_accuracies;
    double* round_losses;
    double communication_cost; // Total bytes transferred
} SigmaFederatedLearning;

SigmaFederatedLearning* sigma_fl_create(void* global_model,
                                       const char* model_type,
                                       const char* aggregation,
                                       uint32_t n_rounds,
                                       double fraction_clients);

void sigma_fl_add_client(SigmaFederatedLearning* fl,
                        const char* client_id,
                        SigmaDataset* local_data,
                        uint32_t local_epochs,
                        uint32_t batch_size);

void sigma_fl_initialize_clients(SigmaFederatedLearning* fl);

void sigma_fl_distribute_model(SigmaFederatedLearning* fl);

void sigma_fl_client_train(SigmaFLClient* client);

void sigma_fl_collect_updates(SigmaFederatedLearning* fl,
                             uint32_t* selected_clients,
                             uint32_t n_selected);

void sigma_fl_aggregate_fedavg(SigmaFederatedLearning* fl,
                              uint32_t* selected_clients,
                              uint32_t n_selected);

void sigma_fl_aggregate_fedprox(SigmaFederatedLearning* fl,
                               uint32_t* selected_clients,
                               uint32_t n_selected,
                               double mu);

void sigma_fl_update_global(SigmaFederatedLearning* fl);

void sigma_fl_train_round(SigmaFederatedLearning* fl);

void sigma_fl_run_full(SigmaFederatedLearning* fl);

double sigma_fl_evaluate_global(SigmaFederatedLearning* fl,
                               SigmaDataset* test_data);

void sigma_fl_save_global(SigmaFederatedLearning* fl, const char* path);
void sigma_fl_load_global(SigmaFederatedLearning* fl, const char* path);
void sigma_fl_destroy(SigmaFederatedLearning* fl);

// ==================== NEURAL ARCHITECTURE SEARCH ====================

// Cell-based NAS (like DARTS, ENAS)
typedef enum {
    SIGMA_OP_NONE,
    SIGMA_OP_CONV_3X3,
    SIGMA_OP_CONV_5X5,
    SIGMA_OP_DIL_CONV_3X3,
    SIGMA_OP_DIL_CONV_5X5,
    SIGMA_OP_MAX_POOL_3X3,
    SIGMA_OP_AVG_POOL_3X3,
    SIGMA_OP_SKIP_CONNECT,
    SIGMA_OP_SEP_CONV_3X3,
    SIGMA_OP_SEP_CONV_5X5,
    SIGMA_OP_DIL_SEP_CONV_3X3,
    SIGMA_OP_DIL_SEP_CONV_5X5
} SigmaNASOperation;

typedef struct {
    // Search space
    uint32_t n_nodes; // Number of intermediate nodes in cell
    uint32_t n_ops;   // Number of primitive operations
    SigmaNASOperation* primitives;
    
    // Architecture parameters (alpha in DARTS)
    double** alphas_normal; // Architecture weights for normal cell
    double** alphas_reduce; // Architecture weights for reduction cell
    
    // Mixed operations (continuous relaxation)
    void*** mixed_ops; // [n_nodes, n_nodes]
    
    // Supernetwork (contains all operations)
    SigmaCNN* supernet;
    
    // Search strategy
    char strategy[16]; // "darts", "enas", "random", " evolutionary"
    
    // Optimization
    double arch_lr; // Learning rate for architecture parameters
    uint32_t n_epochs;
    
    // Best architecture found
    uint32_t** best_normal_arch; // [n_nodes, 2] - (op_id, from_node)
    uint32_t** best_reduce_arch;
    
    // Performance
    double best_val_acc;
} SigmaNAS;

SigmaNAS* sigma_nas_create(uint32_t n_nodes,
                          SigmaNASOperation* primitives,
                          uint32_t n_ops,
                          const char* strategy);

void sigma_nas_build_supernet(SigmaNAS* nas,
                             uint32_t init_channels,
                             uint32_t n_layers,
                             uint32_t num_classes);

void sigma_nas_search_darts(SigmaNAS* nas,
                           SigmaDataset* train_data,
                           SigmaDataset* val_data,
                           uint32_t n_epochs);

void sigma_nas_derive_architecture(SigmaNAS* nas,
                                  uint32_t** normal_arch,
                                  uint32_t** reduce_arch);

void* sigma_nas_build_final_model(SigmaNAS* nas,
                                  uint32_t** normal_arch,
                                  uint32_t** reduce_arch,
                                  uint32_t init_channels,
                                  uint32_t n_layers,
                                  uint32_t num_classes);

void sigma_nas_train_final(SigmaNAS* nas,
                          void* final_model,
                          SigmaDataset* train_data,
                          SigmaDataset* test_data,
                          uint32_t epochs);

void sigma_nas_destroy(SigmaNAS* nas);

// ==================== CONTINUAL LEARNING ====================

// Elastic Weight Consolidation (EWC)
typedef struct {
    // Base model
    void* model;
    char model_type[32];
    
    // Fisher Information matrix (diagonal approximation)
    double** fisher_diagonal; // Per parameter importance
    
    // Optimal parameters from previous tasks
    double** optimal_params;
    
    // Number of tasks learned so far
    uint32_t n_tasks;
    
    // EWC lambda (regularization strength)
    double lambda_ewc;
    
    // Task names
    char** task_names;
} SigmaEWC;

SigmaEWC* sigma_ewc_create(void* model, const char* model_type, double lambda);

void sigma_ewc_compute_fisher(SigmaEWC* ewc,
                             SigmaDataset* task_data,
                             uint32_t task_id);

void sigma_ewc_update_optimal_params(SigmaEWC* ewc, uint32_t task_id);

double sigma_ewc_compute_penalty(SigmaEWC* ewc,
                                double** current_params);

void sigma_ewc_train_task(SigmaEWC* ewc,
                         SigmaDataset* task_data,
                         const char* task_name,
                         uint32_t epochs,
                         uint32_t batch_size);

void sigma_ewc_destroy(SigmaEWC* ewc);

// Progressive Neural Networks
typedef struct {
    // Number of columns (tasks)
    uint32_t n_columns;
    uint32_t max_columns;
    
    // Columns (one per task)
    SigmaMLP** columns;
    char** task_names;
    
    // Lateral connections (adapters from previous columns)
    SigmaMatrix**** lateral_weights; // [to_col][to_layer][from_col]
    
    // Architecture per column
    uint32_t* layer_sizes;
    uint32_t n_layers;
    
    // Which columns to freeze
    bool** frozen_columns;
} SigmaProgressiveNN;

SigmaProgressiveNN* sigma_prog_nn_create(uint32_t* layer_sizes,
                                        uint32_t n_layers,
                                        uint32_t max_tasks);

void sigma_prog_nn_add_column(SigmaProgressiveNN* prog,
                             const char* task_name,
                             bool freeze_previous);

SigmaMatrix* sigma_prog_nn_forward(SigmaProgressiveNN* prog,
                                  uint32_t task_id,
                                  SigmaMatrix* input,
                                  bool use_lateral);

void sigma_prog_nn_train_task(SigmaProgressiveNN* prog,
                             uint32_t task_id,
                             SigmaDataset* task_data,
                             uint32_t epochs);

void sigma_prog_nn_destroy(SigmaProgressiveNN* prog);

// ==================== SELF-SUPERVISED LEARNING ====================

// SimCLR (Simple Contrastive Learning)
typedef struct {
    // Encoder network
    SigmaCNN* encoder;
    
    // Projection head
    SigmaMLP* projection_head;
    
    // Temperature parameter
    double temperature;
    
    // Augmentation parameters
    double color_jitter_strength;
    bool use_blur;
    
    // Training
    uint32_t batch_size; // Needs large batches
    uint32_t n_epochs;
    
    // Metrics
    double contrastive_loss;
    double ntxent_loss; // Normalized temperature-scaled cross entropy
} SigmaSimCLR;

SigmaSimCLR* sigma_simclr_create(SigmaCNN* encoder,
                                uint32_t projection_dim,
                                double temperature);

void sigma_simclr_augment(SigmaSimCLR* simclr,
                         SigmaMatrix* image,
                         SigmaMatrix* augmented1,
                         SigmaMatrix* augmented2);

void sigma_simclr_train(SigmaSimCLR* simclr,
                       SigmaMatrix* images,
                       uint32_t n_images,
                       uint32_t epochs);

double sigma_simclr_nt_xent_loss(SigmaSimCLR* simclr,
                                SigmaMatrix* z_i,
                                SigmaMatrix* z_j,
                                uint32_t batch_size);

void sigma_simclr_destroy(SigmaSimCLR* simclr);

// MoCo (Momentum Contrast)
typedef struct {
    // Query encoder
    SigmaCNN* encoder_q;
    
    // Key encoder (momentum updated)
    SigmaCNN* encoder_k;
    
    // Momentum coefficient
    double momentum;
    
    // Queue of negative samples
    SigmaMatrix* queue;
    uint32_t queue_size;
    uint32_t queue_ptr;
    
    // Projection heads
    SigmaMLP* projection_q;
    SigmaMLP* projection_k;
    
    // Temperature
    double temperature;
    
    // Key encoder update counter
    uint32_t update_counter;
} SigmaMoCo;

SigmaMoCo* sigma_moco_create(SigmaCNN* base_encoder,
                            uint32_t projection_dim,
                            uint32_t queue_size,
                            double momentum,
                            double temperature);

void sigma_moco_update_key_encoder(SigmaMoCo* moco);

void sigma_moco_dequeue_and_enqueue(SigmaMoCo* moco,
                                   SigmaMatrix* keys);

void sigma_moco_train(SigmaMoCo* moco,
                     SigmaMatrix* images,
                     uint32_t n_images,
                     uint32_t epochs);

void sigma_moco_destroy(SigmaMoCo* moco);

// BYOL (Bootstrap Your Own Latent)
typedef struct {
    // Online network (encoder + projector + predictor)
    SigmaCNN* online_encoder;
    SigmaMLP* online_projector;
    SigmaMLP* predictor;
    
    // Target network (encoder + projector)
    SigmaCNN* target_encoder;
    SigmaMLP* target_projector;
    
    // EMA coefficient
    double tau; // Usually 0.996
    
    // Dimensions
    uint32_t projection_dim;
    uint32_t prediction_dim;
} SigmaBYOL;

SigmaBYOL* sigma_byol_create(SigmaCNN* base_encoder,
                            uint32_t projection_dim,
                            uint32_t prediction_dim,
                            double tau);

void sigma_byol_update_target_network(SigmaBYOL* byol);

void sigma_byol_train(SigmaBYOL* byol,
                     SigmaMatrix* images,
                     uint32_t n_images,
                     uint32_t epochs);

void sigma_byol_destroy(SigmaBYOL* byol);

// SwAV (Swapping Assignments)
typedef struct {
    // Encoder
    SigmaCNN* encoder;
    
    // Projection head
    SigmaMLP* projection_head;
    
    // Prototypes (codebook)
    SigmaMatrix* prototypes; // [projection_dim, n_prototypes]
    uint32_t n_prototypes;
    
    // Sinkhorn-Knopp algorithm parameters
    double sinkhorn_epsilon;
    uint32_t sinkhorn_iterations;
    
    // Multi-crop parameters
    uint32_t n_crops;
    uint32_t n_small_crops;
    
    // Temperature
    double temperature;
    
    // Queue for small crops
    SigmaMatrix* queue;
    uint32_t queue_size;
} SigmaSwAV;

SigmaSwAV* sigma_swav_create(SigmaCNN* encoder,
                            uint32_t projection_dim,
                            uint32_t n_prototypes,
                            uint32_t queue_size);

void sigma_swav_sinkhorn_knopp(SigmaSwAV* swav,
                              SigmaMatrix* scores,
                              SigmaMatrix* Q); // Assignment matrix

void sigma_swav_train(SigmaSwAV* swav,
                     SigmaMatrix* images,
                     uint32_t n_images,
                     uint32_t epochs);

void sigma_swav_destroy(SigmaSwAV* swav);

// Barlow Twins
typedef struct {
    // Encoder
    SigmaCNN* encoder;
    
    // Projector
    SigmaMLP* projector;
    
    // Projector output dim
    uint32_t projector_output_dim;
    
    // Lambda parameter (weight for off-diagonal terms)
    double lambda_param;
    
    // Batch normalization in projector
    bool use_batch_norm;
} SigmaBarlowTwins;

SigmaBarlowTwins* sigma_barlow_twins_create(SigmaCNN* encoder,
                                          uint32_t projector_sizes[],
                                          uint32_t n_layers,
                                          double lambda);

void sigma_barlow_twins_loss(SigmaBarlowTwins* bt,
                            SigmaMatrix* z_a,
                            SigmaMatrix* z_b,
                            uint32_t batch_size,
                            double* loss);

void sigma_barlow_twins_train(SigmaBarlowTwins* bt,
                             SigmaMatrix* images,
                             uint32_t n_images,
                             uint32_t epochs);

void sigma_barlow_twins_destroy(SigmaBarlowTwins* bt);

// ==================== ADVERSARIAL TRAINING ====================

// FGSM (Fast Gradient Sign Method)
typedef struct {
    // Epsilon (perturbation magnitude)
    double epsilon;
    
    // Targeted or untargeted
    bool targeted;
    uint32_t target_class;
} SigmaFGSM;

SigmaFGSM* sigma_fgsm_create(double epsilon, bool targeted);

void sigma_fgsm_generate(SigmaFGSM* fgsm,
                      void* model,
                      SigmaMatrix* input,
                      SigmaVector* labels,
                      SigmaMatrix* adversarial_example);

void sigma_fgsm_destroy(SigmaFGSM* fgsm);

// PGD (Projected Gradient Descent)
typedef struct {
    double epsilon;
    double step_size;
    uint32_t num_steps;
    bool targeted;
    uint32_t target_class;
    char norm[8]; // "inf", "2", "1"
} SigmaPGDAttack;

SigmaPGDAttack* sigma_pgd_create(double epsilon,
                                double step_size,
                                uint32_t num_steps,
                                bool targeted,
                                const char* norm);

void sigma_pgd_generate(SigmaPGDAttack* pgd,
                     void* model,
                     SigmaMatrix* input,
                     SigmaVector* labels,
                     SigmaMatrix* adversarial_example);

void sigma_pgd_destroy(SigmaPGDAttack* pgd);

// Adversarial Training
typedef struct {
    // Base model
    void* model;
    char model_type[32];
    
    // Attack method for generating adversarial examples
    char attack_type[16]; // "fgsm", "pgd", "none"
    double epsilon;
    double alpha;
    uint32_t attack_steps;
    
    // Training ratio (clean vs adversarial)
    double adversarial_ratio; // 0.5 = 50% adversarial, 50% clean
    
    // Defense methods
    bool use_randomization; // Random input transformations
    bool use_feature_squeezing;
    bool use_label_smoothing;
} SigmaAdversarialTraining;

SigmaAdversarialTraining* sigma_adv_train_create(void* model,
                                                const char* model_type,
                                                const char* attack_type,
                                                double epsilon);

void sigma_adv_train_epoch(SigmaAdversarialTraining* adv,
                         SigmaDataset* clean_data,
                         uint32_t epoch);

void sigma_adv_train_full(SigmaAdversarialTraining* adv,
                         SigmaDataset* train_data,
                         SigmaDataset* test_data,
                         uint32_t epochs);

double sigma_adv_train_evaluate_robustness(SigmaAdversarialTraining* adv,
                                          SigmaDataset* test_data,
                                          char** attack_methods,
                                          uint32_t n_attacks);

void sigma_adv_train_destroy(SigmaAdversarialTraining* adv);

#endif // SIGMA_ADVANCED_AI_H
