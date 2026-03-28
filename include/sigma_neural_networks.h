/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Neural Networks & Deep Learning Implementation
 * ========================================================
 * Complete neural network architectures:
 * - Multi-Layer Perceptron (MLP)
 * - Convolutional Neural Networks (CNN)
 * - Recurrent Neural Networks (RNN, LSTM, GRU)
 * - Transformers
 * - Autoencoders
 * - Generative Adversarial Networks (GAN)
 */

#ifndef SIGMA_NEURAL_NETWORKS_H
#define SIGMA_NEURAL_NETWORKS_H

#include "sigma_ml_algorithms.h"

// ==================== NEURAL NETWORK LAYERS ====================

typedef enum {
    SIGMA_LAYER_DENSE,
    SIGMA_LAYER_CONV2D,
    SIGMA_LAYER_CONV3D,
    SIGMA_LAYER_MAXPOOL2D,
    SIGMA_LAYER_AVGPOOL2D,
    SIGMA_LAYER_DROPOUT,
    SIGMA_LAYER_BATCHNORM,
    SIGMA_LAYER_LSTM,
    SIGMA_LAYER_GRU,
    SIGMA_LAYER_ATTENTION,
    SIGMA_LAYER_FLATTEN,
    SIGMA_LAYER_RESHAPE,
    SIGMA_LAYER_ACTIVATION
} SigmaLayerType;

typedef struct SigmaLayer {
    SigmaLayerType type;
    uint32_t input_size;
    uint32_t output_size;
    
    // Weights and biases
    SigmaMatrix* weights;
    SigmaVector* biases;
    
    // Gradients
    SigmaMatrix* weight_gradients;
    SigmaVector* bias_gradients;
    
    // Momentum (for Adam)
    SigmaMatrix* m_weights;
    SigmaMatrix* v_weights;
    SigmaVector* m_biases;
    SigmaVector* v_biases;
    
    // Activations and cache
    SigmaMatrix* activations;
    SigmaMatrix* pre_activations;
    
    // For convolution layers
    uint32_t kernel_size;
    uint32_t filters;
    uint32_t stride;
    uint32_t padding;
    
    // For pooling layers
    uint32_t pool_size;
    
    // For dropout
    double dropout_rate;
    SigmaMatrix* dropout_mask;
    
    // For LSTM/GRU
    uint32_t hidden_units;
    SigmaMatrix* hidden_state;
    SigmaMatrix* cell_state;
    
    // For attention
    uint32_t num_heads;
    double attention_dropout;
    
    // Activation function
    double (*activation)(double);
    double (*activation_derivative)(double);
    
    struct SigmaLayer* next;
    struct SigmaLayer* prev;
} SigmaLayer;

// ==================== MULTI-LAYER PERCEPTRON ====================

typedef struct {
    SigmaLayer* layers;
    uint32_t n_layers;
    double learning_rate;
    double momentum;
    double l2_lambda;
    uint32_t batch_size;
    uint32_t epochs;
    char optimizer[16]; // "sgd", "adam", "rmsprop"
    double beta1;
    double beta2;
    double epsilon;
    char loss_function[32];
    double train_loss;
    double val_loss;
    double train_accuracy;
    double val_accuracy;
    SigmaHyperParams params;
} SigmaMLP;

SigmaMLP* sigma_mlp_create(SigmaHyperParams* params);
void sigma_mlp_add_layer(SigmaMLP* model, SigmaLayerType type, uint32_t units, 
                         const char* activation, double dropout);
void sigma_mlp_compile(SigmaMLP* model, const char* optimizer, const char* loss);
void sigma_mlp_fit(SigmaMLP* model, SigmaDataset* train_data, SigmaDataset* val_data);
SigmaVector* sigma_mlp_predict(SigmaMLP* model, SigmaMatrix* X);
SigmaMatrix* sigma_mlp_predict_proba(SigmaMLP* model, SigmaMatrix* X);
void sigma_mlp_evaluate(SigmaMLP* model, SigmaDataset* data);
void sigma_mlp_save(SigmaMLP* model, const char* filename);
SigmaMLP* sigma_mlp_load(const char* filename);
void sigma_mlp_destroy(SigmaMLP* model);

// ==================== CONVOLUTIONAL NEURAL NETWORK ====================

typedef struct {
    SigmaLayer* layers;
    uint32_t n_layers;
    uint32_t input_shape[3]; // height, width, channels
    uint32_t num_classes;
    char architecture[64]; // "lenet", "alexnet", "vgg", "resnet", "custom"
    
    // Training metrics
    double train_loss;
    double val_loss;
    double train_accuracy;
    double val_accuracy;
    
    SigmaHyperParams params;
} SigmaCNN;

SigmaCNN* sigma_cnn_create(uint32_t* input_shape, uint32_t num_classes, 
                           const char* architecture, SigmaHyperParams* params);
void sigma_cnn_add_conv2d(SigmaCNN* model, uint32_t filters, uint32_t kernel_size, 
                          uint32_t stride, uint32_t padding, const char* activation);
void sigma_cnn_add_maxpool2d(SigmaCNN* model, uint32_t pool_size, uint32_t stride);
void sigma_cnn_add_dense(SigmaCNN* model, uint32_t units, const char* activation, double dropout);
void sigma_cnn_add_batchnorm(SigmaCNN* model);
void sigma_cnn_add_dropout(SigmaCNN* model, double rate);
void sigma_cnn_add_flatten(SigmaCNN* model);
void sigma_cnn_compile(SigmaCNN* model, const char* optimizer, const char* loss);
void sigma_cnn_fit(SigmaCNN* model, SigmaDataset* train_data, SigmaDataset* val_data, 
                   uint32_t epochs, uint32_t batch_size);
SigmaVector* sigma_cnn_predict(SigmaCNN* model, SigmaMatrix* X);
void sigma_cnn_evaluate(SigmaCNN* model, SigmaDataset* data);
void sigma_cnn_save(SigmaCNN* model, const char* filename);
SigmaCNN* sigma_cnn_load(const char* filename);
void sigma_cnn_destroy(SigmaCNN* model);

// Predefined architectures
SigmaCNN* sigma_cnn_lenet5(uint32_t num_classes, SigmaHyperParams* params);
SigmaCNN* sigma_cnn_alexnet(uint32_t num_classes, SigmaHyperParams* params);
SigmaCNN* sigma_cnn_vgg16(uint32_t num_classes, SigmaHyperParams* params);
SigmaCNN* sigma_cnn_resnet18(uint32_t num_classes, SigmaHyperParams* params);

// ==================== RECURRENT NEURAL NETWORKS ====================

typedef struct {
    SigmaLayer* layers;
    uint32_t n_layers;
    uint32_t sequence_length;
    uint32_t input_dim;
    uint32_t hidden_units;
    uint32_t num_classes;
    bool return_sequences;
    bool bidirectional;
    char cell_type[8]; // "rnn", "lstm", "gru"
    
    double train_loss;
    double val_loss;
    double train_accuracy;
    double val_accuracy;
    
    SigmaHyperParams params;
} SigmaRNN;

SigmaRNN* sigma_rnn_create(uint32_t sequence_length, uint32_t input_dim,
                             const char* cell_type, uint32_t hidden_units,
                             bool bidirectional, SigmaHyperParams* params);
void sigma_rnn_add_layer(SigmaRNN* model, const char* cell_type, 
                         uint32_t hidden_units, double dropout, bool return_seq);
void sigma_rnn_add_dense(SigmaRNN* model, uint32_t units, const char* activation);
void sigma_rnn_compile(SigmaRNN* model, const char* optimizer, const char* loss);
void sigma_rnn_fit(SigmaRNN* model, SigmaDataset* train_data, SigmaDataset* val_data);
SigmaMatrix* sigma_rnn_predict(SigmaRNN* model, SigmaMatrix* X);
void sigma_rnn_evaluate(SigmaRNN* model, SigmaDataset* data);
void sigma_rnn_save(SigmaRNN* model, const char* filename);
void sigma_rnn_destroy(SigmaRNN* model);

// LSTM Cell operations
void sigma_lstm_forward(SigmaMatrix* input, SigmaMatrix* h_prev, SigmaMatrix* c_prev,
                        SigmaMatrix* W_f, SigmaMatrix* W_i, SigmaMatrix* W_c, SigmaMatrix* W_o,
                        SigmaMatrix* U_f, SigmaMatrix* U_i, SigmaMatrix* U_c, SigmaMatrix* U_o,
                        SigmaVector* b_f, SigmaVector* b_i, SigmaVector* b_c, SigmaVector* b_o,
                        SigmaMatrix* h_next, SigmaMatrix* c_next);

// GRU Cell operations
void sigma_gru_forward(SigmaMatrix* input, SigmaMatrix* h_prev,
                       SigmaMatrix* W_z, SigmaMatrix* W_r, SigmaMatrix* W_h,
                       SigmaMatrix* U_z, SigmaMatrix* U_r, SigmaMatrix* U_h,
                       SigmaVector* b_z, SigmaVector* b_r, SigmaVector* b_h,
                       SigmaMatrix* h_next);

// ==================== TRANSFORMER ====================

typedef struct {
    uint32_t num_heads;
    uint32_t d_model;
    uint32_t d_k;
    uint32_t d_v;
    double dropout;
    
    // Query, Key, Value weights
    SigmaMatrix* W_q;
    SigmaMatrix* W_k;
    SigmaMatrix* W_v;
    SigmaMatrix* W_o;
    
    // Gradients
    SigmaMatrix* dW_q;
    SigmaMatrix* dW_k;
    SigmaMatrix* dW_v;
    SigmaMatrix* dW_o;
} SigmaAttentionHead;

typedef struct {
    SigmaAttentionHead* heads;
    uint32_t num_heads;
    uint32_t d_model;
    double dropout;
    
    SigmaMatrix* attention_weights; // For visualization
} SigmaMultiHeadAttention;

typedef struct {
    // Multi-head attention
    SigmaMultiHeadAttention* mha;
    
    // Feed-forward network
    SigmaMatrix* W1;
    SigmaVector* b1;
    SigmaMatrix* W2;
    SigmaVector* b2;
    
    // Layer normalization
    SigmaVector* ln1_gamma;
    SigmaVector* ln1_beta;
    SigmaVector* ln2_gamma;
    SigmaVector* ln2_beta;
    
    // Dropout
    double dropout_rate;
} SigmaTransformerEncoderLayer;

typedef struct {
    SigmaTransformerEncoderLayer* layers;
    uint32_t n_layers;
    uint32_t d_model;
    uint32_t num_heads;
    uint32_t d_ff;
    uint32_t max_seq_length;
    uint32_t vocab_size;
    double dropout;
    
    // Embeddings
    SigmaMatrix* token_embedding;
    SigmaMatrix* position_embedding;
    
    // Output layer
    SigmaMatrix* output_weights;
    SigmaVector* output_bias;
    
    double train_loss;
    double val_loss;
    double train_accuracy;
    double val_accuracy;
    
    SigmaHyperParams params;
} SigmaTransformer;

SigmaTransformer* sigma_transformer_create(uint32_t vocab_size, uint32_t d_model,
                                          uint32_t num_heads, uint32_t n_layers,
                                          uint32_t d_ff, uint32_t max_seq_length,
                                          SigmaHyperParams* params);
void sigma_transformer_compile(SigmaTransformer* model, const char* optimizer);
void sigma_transformer_fit(SigmaTransformer* model, SigmaDataset* train_data, 
                           SigmaDataset* val_data, uint32_t epochs);
SigmaMatrix* sigma_transformer_encode(SigmaTransformer* model, SigmaMatrix* input_ids);
SigmaMatrix* sigma_transformer_predict(SigmaTransformer* model, SigmaMatrix* input_ids);
void sigma_transformer_save(SigmaTransformer* model, const char* filename);
void sigma_transformer_destroy(SigmaTransformer* model);

// ==================== AUTOENCODER ====================

typedef struct {
    SigmaLayer* encoder_layers;
    SigmaLayer* decoder_layers;
    uint32_t n_encoder_layers;
    uint32_t n_decoder_layers;
    uint32_t input_dim;
    uint32_t encoding_dim;
    
    double reconstruction_loss;
    SigmaHyperParams params;
} SigmaAutoencoder;

SigmaAutoencoder* sigma_autoencoder_create(uint32_t input_dim, uint32_t encoding_dim,
                                           SigmaHyperParams* params);
void sigma_autoencoder_add_encoder_layer(SigmaAutoencoder* model, uint32_t units, 
                                         const char* activation);
void sigma_autoencoder_add_decoder_layer(SigmaAutoencoder* model, uint32_t units,
                                         const char* activation);
void sigma_autoencoder_compile(SigmaAutoencoder* model, const char* optimizer);
void sigma_autoencoder_fit(SigmaAutoencoder* model, SigmaMatrix* X, uint32_t epochs);
SigmaMatrix* sigma_autoencoder_encode(SigmaAutoencoder* model, SigmaMatrix* X);
SigmaMatrix* sigma_autoencoder_decode(SigmaAutoencoder* model, SigmaMatrix* encoded);
SigmaMatrix* sigma_autoencoder_reconstruct(SigmaAutoencoder* model, SigmaMatrix* X);
void sigma_autoencoder_destroy(SigmaAutoencoder* model);

// Variational Autoencoder
typedef struct {
    SigmaAutoencoder* base;
    SigmaMatrix* mu_weights;
    SigmaVector* mu_bias;
    SigmaMatrix* log_var_weights;
    SigmaVector* log_var_bias;
    double kl_weight;
} SigmaVAE;

SigmaVAE* sigma_vae_create(uint32_t input_dim, uint32_t latent_dim, SigmaHyperParams* params);
void sigma_vae_fit(SigmaVAE* model, SigmaMatrix* X, uint32_t epochs);
SigmaMatrix* sigma_vae_sample(SigmaVAE* model, SigmaMatrix* mu, SigmaMatrix* log_var);
SigmaMatrix* sigma_vae_generate(SigmaVAE* model, uint32_t n_samples);
void sigma_vae_destroy(SigmaVAE* model);

// ==================== GAN (GENERATIVE ADVERSARIAL NETWORK) ====================

typedef struct {
    SigmaMLP* generator;
    SigmaMLP* discriminator;
    uint32_t latent_dim;
    uint32_t output_dim;
    char type[16]; // "vanilla", "dcgan", "cgan", "wgan"
    
    double g_loss;
    double d_loss;
    double g_accuracy;
    double d_accuracy;
    
    SigmaHyperParams params;
} SigmaGAN;

SigmaGAN* sigma_gan_create(uint32_t latent_dim, uint32_t output_dim,
                             const char* type, SigmaHyperParams* params);
void sigma_gan_compile(SigmaGAN* model, const char* g_optimizer, 
                       const char* d_optimizer, const char* loss);
void sigma_gan_fit(SigmaGAN* model, SigmaMatrix* real_data, uint32_t epochs, 
                    uint32_t batch_size);
SigmaMatrix* sigma_gan_generate(SigmaGAN* model, uint32_t n_samples);
void sigma_gan_save(SigmaGAN* model, const char* filename);
void sigma_gan_destroy(SigmaGAN* model);

// DCGAN (Deep Convolutional GAN)
typedef struct {
    SigmaCNN* generator;
    SigmaCNN* discriminator;
    uint32_t latent_dim;
    uint32_t* image_shape;
    
    double g_loss;
    double d_loss;
    
    SigmaHyperParams params;
} SigmaDCGAN;

SigmaDCGAN* sigma_dcgan_create(uint32_t latent_dim, uint32_t* image_shape,
                               SigmaHyperParams* params);
void sigma_dcgan_fit(SigmaDCGAN* model, SigmaMatrix* images, uint32_t epochs);
SigmaMatrix* sigma_dcgan_generate(SigmaDCGAN* model, uint32_t n_samples);
void sigma_dcgan_save(SigmaDCGAN* model, const char* filename);
void sigma_dcgan_destroy(SigmaDCGAN* model);

// ==================== TRAINING UTILITIES ====================

// Callbacks
typedef struct {
    char type[32]; // "early_stopping", "model_checkpoint", "learning_rate_scheduler"
    uint32_t patience;
    double min_delta;
    uint32_t cooldown;
    double factor;
    double min_lr;
    char monitor[16]; // "loss", "val_loss", "accuracy", "val_accuracy"
    char mode[8]; // "min", "max"
    uint32_t best_epoch;
    double best_value;
    bool stop_training;
} SigmaCallback;

SigmaCallback* sigma_callback_early_stopping(uint32_t patience, const char* monitor);
SigmaCallback* sigma_callback_checkpoint(const char* filepath, const char* monitor);
SigmaCallback* sigma_callback_lr_scheduler(double factor, uint32_t patience);
void sigma_callback_check(SigmaCallback* callback, uint32_t epoch, 
                          double train_loss, double val_loss,
                          double train_acc, double val_acc);
void sigma_callback_destroy(SigmaCallback* callback);

// Data augmentation
SigmaMatrix* sigma_augment_flip_horizontal(SigmaMatrix* images);
SigmaMatrix* sigma_augment_flip_vertical(SigmaMatrix* images);
SigmaMatrix* sigma_augment_rotate(SigmaMatrix* images, double angle);
SigmaMatrix* sigma_augment_zoom(SigmaMatrix* images, double zoom_factor);
SigmaMatrix* sigma_augment_shift(SigmaMatrix* images, double shift_x, double shift_y);
SigmaMatrix* sigma_augment_brightness(SigmaMatrix* images, double factor);
SigmaMatrix* sigma_augment_contrast(SigmaMatrix* images, double factor);
SigmaMatrix* sigma_augment_noise(SigmaMatrix* images, double noise_factor);
SigmaMatrix* sigma_augment_cutout(SigmaMatrix* images, uint32_t n_holes, uint32_t length);

// Batch generators
SigmaDataset* sigma_batch_generator(SigmaDataset* data, uint32_t batch_size, bool shuffle);
void sigma_shuffle_dataset(SigmaDataset* data);
void sigma_normalize_dataset(SigmaDataset* data);
void sigma_standardize_dataset(SigmaDataset* data);

// Cross-validation
typedef struct {
    SigmaDataset** train_folds;
    SigmaDataset** val_folds;
    uint32_t n_folds;
    double* scores;
} SigmaCrossValidation;

SigmaCrossValidation* sigma_cross_validation_split(SigmaDataset* data, uint32_t n_folds);
void sigma_cross_validation_destroy(SigmaCrossValidation* cv);

// ==================== MODEL OPTIMIZATION ====================

// Quantization
void sigma_quantize_weights(SigmaMatrix* weights, uint32_t bits);
void sigma_quantize_model(SigmaMLP* model, uint32_t bits);

// Pruning
void sigma_prune_weights(SigmaMatrix* weights, double sparsity);
void sigma_prune_model(SigmaMLP* model, double sparsity);

// Knowledge Distillation
typedef struct {
    SigmaMLP* teacher;
    SigmaMLP* student;
    double temperature;
    double alpha;
} SigmaDistillation;

SigmaDistillation* sigma_distillation_create(SigmaMLP* teacher, SigmaMLP* student,
                                             double temperature, double alpha);
void sigma_distillation_train(SigmaDistillation* dist, SigmaDataset* data, uint32_t epochs);
void sigma_distillation_destroy(SigmaDistillation* dist);

// ==================== DEPLOYMENT ====================

// Model export
void sigma_export_onnx(SigmaMLP* model, const char* filename);
void sigma_export_tensorrt(SigmaCNN* model, const char* filename);
void sigma_export_tflite(SigmaMLP* model, const char* filename);
void sigma_export_torchscript(SigmaMLP* model, const char* filename);

// Model serving
void sigma_serve_model_http(SigmaMLP* model, uint32_t port);
void sigma_serve_model_grpc(SigmaMLP* model, uint32_t port);

#endif // SIGMA_NEURAL_NETWORKS_H

