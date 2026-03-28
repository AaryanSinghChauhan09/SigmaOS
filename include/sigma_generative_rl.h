/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Advanced Generative AI & Reinforcement Learning
 * =========================================================
 * State-of-the-art generative and RL models:
 * - Diffusion Models (DDPM, DDIM, Stable Diffusion style)
 * - Advanced VAEs (VQ-VAE, VQ-VAE-2)
 * - Advanced GANs (StyleGAN, CycleGAN, Pix2Pix)
 * - Advanced RL (SAC, TD3, PPO, A3C, DDPG, TRPO)
 * - Multi-agent RL
 */

#ifndef SIGMA_GENERATIVE_RL_H
#define SIGMA_GENERATIVE_RL_H

#include "sigma_neural_networks.h"

// ==================== DIFFUSION MODELS ====================

typedef struct {
    // Diffusion parameters
    uint32_t num_timesteps; // T (usually 1000)
    double beta_start; // 0.0001
    double beta_end; // 0.02
    double* beta_schedule; // Linear or cosine
    double* alpha; // 1 - beta
    double* alpha_bar; // Cumulative product of alpha
    double* sigma; // Noise schedule
    
    // Model architecture
    char model_type[32]; // "unet", "transformer"
    uint32_t image_size;
    uint32_t channels;
    uint32_t hidden_size;
    
    // U-Net denoising model
    SigmaUNet* denoise_model;
    
    // Conditional generation
    bool conditional;
    uint32_t num_classes;
    uint32_t text_embed_dim; // For text-to-image
    
    // Text encoder (CLIP style) for text-to-image
    SigmaTransformer* text_encoder;
    
    // Classifier-free guidance
    double guidance_scale; // 7.5 default
    double unconditional_prob; // 0.1 for training
    
    // Sampling method
    char sampler[16]; // "ddpm", "ddim", "pndm", "dpm"
    uint32_t num_inference_steps; // Usually 50
    
    // VAE encoder/decoder (for latent diffusion)
    bool use_latent; // Stable diffusion style
    SigmaVAE* vae; // Encode to latent space
    uint32_t latent_size; // 64 for 512x512 images
} SigmaDiffusionModel;

SigmaDiffusionModel* sigma_diffusion_create(uint32_t image_size, 
                                           uint32_t channels,
                                           uint32_t num_timesteps);
void sigma_diffusion_set_beta_schedule(SigmaDiffusionModel* dm,
                                      const char* schedule_type); // "linear", "cosine"
void sigma_diffusion_build_unet(SigmaDiffusionModel* dm,
                               uint32_t base_channels,
                               uint32_t num_res_blocks,
                               uint32_t attention_resolutions[]);
void sigma_diffusion_add_attention(SigmaDiffusionModel* dm,
                                  uint32_t num_heads,
                                  uint32_t head_dim);
void sigma_diffusion_add_text_conditioning(SigmaDiffusionModel* dm,
                                          uint32_t vocab_size,
                                          uint32_t max_length,
                                          uint32_t embed_dim);

// Forward diffusion (training)
void sigma_diffusion_forward_diffusion(SigmaDiffusionModel* dm,
                                      SigmaMatrix* x_0, // Clean image
                                      uint32_t t, // Timestep
                                      SigmaMatrix* x_t, // Noisy image
                                      SigmaMatrix* noise); // Added noise

// Training
void sigma_diffusion_train(SigmaDiffusionModel* dm,
                          SigmaMatrix* images,
                          uint32_t n_images,
                          uint32_t epochs,
                          uint32_t batch_size);
void sigma_diffusion_train_conditional(SigmaDiffusionModel* dm,
                                      SigmaMatrix* images,
                                      char** captions,
                                      uint32_t n_images,
                                      uint32_t epochs);

// Sampling/Generation
void sigma_diffusion_sample_ddpm(SigmaDiffusionModel* dm,
                                SigmaMatrix* generated_image);
void sigma_diffusion_sample_ddim(SigmaDiffusionModel* dm,
                                uint32_t num_inference_steps,
                                SigmaMatrix* generated_image);
void sigma_diffusion_sample_pndm(SigmaDiffusionModel* dm,
                                SigmaMatrix* generated_image);
void sigma_diffusion_sample_plms(SigmaDiffusionModel* dm,
                                SigmaMatrix* generated_image);

// Conditional generation
void sigma_diffusion_generate_text_to_image(SigmaDiffusionModel* dm,
                                           const char* prompt,
                                           uint32_t num_images,
                                           double guidance_scale,
                                           SigmaMatrix** generated_images);
void sigma_diffusion_generate_class_conditional(SigmaDiffusionModel* dm,
                                              uint32_t class_id,
                                              SigmaMatrix* generated_image);
void sigma_diffusion_generate_image_to_image(SigmaDiffusionModel* dm,
                                            SigmaMatrix* input_image,
                                            double strength, // 0.0 to 1.0
                                            const char* prompt,
                                            SigmaMatrix* output_image);
void sigma_diffusion_inpaint(SigmaDiffusionModel* dm,
                            SigmaMatrix* image,
                            SigmaMatrix* mask, // 1 for areas to inpaint
                            const char* prompt,
                            SigmaMatrix* inpainted_image);

// Utilities
void sigma_diffusion_save(SigmaDiffusionModel* dm, const char* path);
void sigma_diffusion_load(SigmaDiffusionModel* dm, const char* path);
void sigma_diffusion_destroy(SigmaDiffusionModel* dm);

// ==================== LATENT DIFFUSION (STABLE DIFFUSION STYLE) ====================

typedef struct {
    SigmaVAE* vae; // Encode images to latent space
    SigmaDiffusionModel* latent_diffusion; // Diffusion in latent space
    SigmaTransformer* text_encoder; // CLIP-like text encoder
    
    // U-Net for denoising in latent space
    uint32_t latent_channels;
    uint32_t latent_height;
    uint32_t latent_width;
    
    // Downsampling factor
    uint32_t downsample_factor; // Usually 8 (512->64)
} SigmaLatentDiffusion;

SigmaLatentDiffusion* sigma_latent_diffusion_create(uint32_t image_size,
                                                    uint32_t latent_dim);
void sigma_latent_diffusion_encode_image(SigmaLatentDiffusion* ld,
                                        SigmaMatrix* image,
                                        SigmaMatrix* latent);
void sigma_latent_diffusion_decode_latent(SigmaLatentDiffusion* ld,
                                         SigmaMatrix* latent,
                                         SigmaMatrix* image);
void sigma_latent_diffusion_generate(SigmaLatentDiffusion* ld,
                                    const char* prompt,
                                    uint32_t num_inference_steps,
                                    double guidance_scale,
                                    SigmaMatrix* generated_image);
void sigma_latent_diffusion_destroy(SigmaLatentDiffusion* ld);

// ==================== VQ-VAE (Vector Quantized VAE) ====================

typedef struct {
    // Encoder/Decoder
    SigmaCNN* encoder;
    SigmaCNN* decoder;
    
    // Codebook
    SigmaMatrix* embeddings; // [num_embeddings, embedding_dim]
    uint32_t num_embeddings; // Codebook size (usually 1024)
    uint32_t embedding_dim; // Usually 256 or 512
    
    // Commitment cost (beta in VQ-VAE paper)
    double commitment_cost;
    
    // Input/output
    uint32_t input_channels;
    uint32_t output_channels;
    uint32_t image_height;
    uint32_t image_width;
    
    // Quantization
    uint32_t* quantized_indices; // Indices into codebook
    
    // Hierarchical (VQ-VAE-2)
    bool hierarchical;
    uint32_t n_levels; // 2 for VQ-VAE-2
    SigmaVQVAE** hierarchical_vqvae; // Top and bottom level
    
    // Priors for generation (PixelCNN style)
    SigmaPixelCNN* top_prior;
    SigmaPixelCNN* bottom_prior;
    
    // Training metrics
    double reconstruction_loss;
    double vq_loss;
    double perplexity; // Codebook usage
} SigmaVQVAE;

typedef struct {
    // PixelCNN architecture
    uint32_t n_filters;
    uint32_t kernel_size;
    uint32_t n_residual_blocks;
    uint32_t n_classes; // For conditional
} SigmaPixelCNN;

SigmaVQVAE* sigma_vqvae_create(uint32_t num_embeddings,
                              uint32_t embedding_dim,
                              uint32_t input_channels,
                              uint32_t hidden_dim);
SigmaVQVAE* sigma_vqvae_2_create(uint32_t top_num_embeddings,
                                uint32_t bottom_num_embeddings,
                                uint32_t embedding_dim);

void sigma_vqvae_encode(SigmaVQVAE* vqvae,
                       SigmaMatrix* images,
                       uint32_t* quantized_indices,
                       SigmaMatrix* quantized_latents);
void sigma_vqvae_decode(SigmaVQVAE* vqvae,
                       SigmaMatrix* quantized_latents,
                       SigmaMatrix* reconstructed_images);
void sigma_vqvae_reconstruct(SigmaVQVAE* vqvae,
                            SigmaMatrix* images,
                            SigmaMatrix* reconstructed_images);

// Training
void sigma_vqvae_train(SigmaVQVAE* vqvae,
                      SigmaMatrix* images,
                      uint32_t n_images,
                      uint32_t epochs,
                      uint32_t batch_size);

// Generation using priors
void sigma_vqvae_train_prior(SigmaVQVAE* vqvae,
                            uint32_t* quantized_indices,
                            uint32_t n_samples,
                            uint32_t epochs);
void sigma_vqvae_generate(SigmaVQVAE* vqvae,
                          uint32_t n_samples,
                          SigmaMatrix* generated_images);
void sigma_vqvae_generate_hierarchical(SigmaVQVAE* vqvae,
                                     uint32_t n_samples,
                                     SigmaMatrix* generated_images);

void sigma_vqvae_save(SigmaVQVAE* vqvae, const char* path);
void sigma_vqvae_load(SigmaVQVAE* vqvae, const char* path);
void sigma_vqvae_destroy(SigmaVQVAE* vqvae);

// ==================== ADVANCED GANs ====================

// StyleGAN
typedef struct {
    // Mapping network (z -> w)
    SigmaMLP* mapping_network;
    uint32_t mapping_layers;
    uint32_t mapping_lr_mul;
    
    // Synthesis network (w -> image)
    uint32_t n_synthesis_layers;
    uint32_t* synthesis_channels;
    
    // Style modulation (AdaIN)
    bool use_style_modulation;
    
    // Progressive growing
    bool use_progressive_growing;
    uint32_t current_resolution; // 4x4 to 1024x1024
    
    // Noise inputs (per layer)
    SigmaMatrix** noise_inputs;
    
    // Style mixing
    double style_mixing_prob;
    
    // Discriminator
    SigmaCNN* discriminator;
    bool discriminator_progressive;
    
    // Truncation trick
    double truncation_psi;
    double truncation_cutoff;
    
    // Path length regularization
    bool use_path_reg;
    double path_reg_weight;
    
    // Resolution
    uint32_t resolution;
    uint32_t n_channels;
    
    // Latent space
    uint32_t latent_dim;
    uint32_t w_dim; // Intermediate latent
    
    double g_loss;
    double d_loss;
} SigmaStyleGAN;

SigmaStyleGAN* sigma_stylegan_create(uint32_t resolution,
                                    uint32_t latent_dim,
                                    uint32_t n_channels);
SigmaStyleGAN* sigma_stylegan2_create(uint32_t resolution,
                                     uint32_t latent_dim,
                                     uint32_t n_channels);

void sigma_stylegan_map_latent(SigmaStyleGAN* stylegan,
                              SigmaMatrix* z, // [batch, latent_dim]
                              SigmaMatrix* w); // [batch, w_dim]
void sigma_stylegan_synthesize(SigmaStyleGAN* stylegan,
                              SigmaMatrix* w,
                              uint32_t target_resolution,
                              SigmaMatrix* generated_images);
void sigma_stylegan_train(SigmaStyleGAN* stylegan,
                         SigmaMatrix* real_images,
                         uint32_t n_images,
                         uint32_t epochs,
                         uint32_t batch_size);

// Style mixing for controlled generation
void sigma_stylegan_mix_styles(SigmaStyleGAN* stylegan,
                              SigmaMatrix* w1,
                              SigmaMatrix* w2,
                              uint32_t crossover_layer,
                              SigmaMatrix* mixed_w);

// Truncation trick for quality vs diversity trade-off
void sigma_stylegan_apply_truncation(SigmaStyleGAN* stylegan,
                                    SigmaMatrix* w,
                                    double psi,
                                    SigmaMatrix* truncated_w);

// Generate with style mixing
void sigma_stylegan_generate_style_mixed(SigmaStyleGAN* stylegan,
                                        SigmaMatrix* z1,
                                        SigmaMatrix* z2,
                                        uint32_t layer,
                                        SigmaMatrix* generated_images);

// Project real image to latent space (encoder)
void sigma_stylegan_project_to_latent(SigmaStyleGAN* stylegan,
                                     SigmaMatrix* image,
                                     uint32_t n_steps,
                                     SigmaMatrix* w);

void sigma_stylegan_save(SigmaStyleGAN* stylegan, const char* path);
void sigma_stylegan_load(SigmaStyleGAN* stylegan, const char* path);
void sigma_stylegan_destroy(SigmaStyleGAN* stylegan);

// CycleGAN (Unpaired Image-to-Image Translation)
typedef struct {
    // Two generators (A->B and B->A)
    SigmaCNN* generator_AB;
    SigmaCNN* generator_BA;
    
    // Two discriminators
    SigmaCNN* discriminator_A;
    SigmaCNN* discriminator_B;
    
    // Cycle consistency loss weight
    double lambda_cycle;
    
    // Identity loss weight
    double lambda_identity;
    
    // Domain names
    char domain_A[64];
    char domain_B[64];
    
    // Image size
    uint32_t image_size;
    uint32_t n_channels;
    
    // Losses
    double g_loss;
    double d_loss;
    double cycle_loss;
    double identity_loss;
} SigmaCycleGAN;

SigmaCycleGAN* sigma_cyclegan_create(uint32_t image_size,
                                    uint32_t n_channels,
                                    const char* domain_A,
                                    const char* domain_B);

void sigma_cyclegan_train(SigmaCycleGAN* cyclegan,
                         SigmaMatrix* images_A,
                         SigmaMatrix* images_B,
                         uint32_t n_epochs,
                         uint32_t batch_size);

void sigma_cyclegan_translate_A_to_B(SigmaCycleGAN* cyclegan,
                                    SigmaMatrix* images_A,
                                    SigmaMatrix* translated_B);
void sigma_cyclegan_translate_B_to_A(SigmaCycleGAN* cyclegan,
                                    SigmaMatrix* images_B,
                                    SigmaMatrix* translated_A);

void sigma_cyclegan_destroy(SigmaCycleGAN* cyclegan);

// Pix2Pix (Paired Image-to-Image Translation)
typedef struct {
    // U-Net generator
    SigmaUNet* generator;
    
    // PatchGAN discriminator
    SigmaCNN* discriminator;
    
    // L1 loss weight
    double lambda_L1;
    
    // Image size
    uint32_t image_size;
    uint32_t input_channels;
    uint32_t output_channels;
    
    // Losses
    double g_loss;
    double d_loss;
    double l1_loss;
} SigmaPix2Pix;

SigmaPix2Pix* sigma_pix2pix_create(uint32_t image_size,
                                  uint32_t input_channels,
                                  uint32_t output_channels);

void sigma_pix2pix_train(SigmaPix2Pix* pix2pix,
                        SigmaMatrix* input_images,
                        SigmaMatrix* target_images,
                        uint32_t n_epochs,
                        uint32_t batch_size);

void sigma_pix2pix_generate(SigmaPix2Pix* pix2pix,
                           SigmaMatrix* input_images,
                           SigmaMatrix* generated_images);

void sigma_pix2pix_destroy(SigmaPix2Pix* pix2pix);

// ==================== ADVANCED REINFORCEMENT LEARNING ====================

// Soft Actor-Critic (SAC)
typedef struct {
    // Actor network (policy)
    SigmaMLP* actor;
    
    // Critic networks (Q-functions)
    SigmaMLP* critic1;
    SigmaMLP* critic2;
    
    // Target networks
    SigmaMLP* target_critic1;
    SigmaMLP* target_critic2;
    
    // Entropy temperature (automatic tuning)
    double alpha;
    double target_entropy;
    double log_alpha;
    bool automatic_entropy_tuning;
    
    // State and action dimensions
    uint32_t state_dim;
    uint32_t action_dim;
    
    // Action bounds (for continuous actions)
    double* action_min;
    double* action_max;
    
    // Hyperparameters
    double gamma;
    double tau;
    double actor_lr;
    double critic_lr;
    double alpha_lr;
    
    // Replay buffer
    double** replay_buffer_states;
    double** replay_buffer_actions;
    double* replay_buffer_rewards;
    double** replay_buffer_next_states;
    bool* replay_buffer_dones;
    uint32_t replay_buffer_capacity;
    uint32_t replay_buffer_size;
    
    // Metrics
    double avg_reward;
    double critic_loss;
    double actor_loss;
    double alpha_loss;
} SigmaSAC;

SigmaSAC* sigma_sac_create(uint32_t state_dim,
                          uint32_t action_dim,
                          double* action_min,
                          double* action_max);

void sigma_sac_select_action(SigmaSAC* sac,
                            double* state,
                            bool deterministic,
                            double* action);
void sigma_sac_store_transition(SigmaSAC* sac,
                               double* state,
                               double* action,
                               double reward,
                               double* next_state,
                               bool done);
void sigma_sac_update(SigmaSAC* sac, uint32_t batch_size);
void sigma_sac_train(SigmaSAC* sac,
                    SigmaRLEnvironment* env,
                    uint32_t n_episodes,
                    uint32_t max_steps);

void sigma_sac_save(SigmaSAC* sac, const char* path);
void sigma_sac_load(SigmaSAC* sac, const char* path);
void sigma_sac_destroy(SigmaSAC* sac);

// Twin Delayed Deep Deterministic Policy Gradient (TD3)
typedef struct {
    // Actor network
    SigmaMLP* actor;
    SigmaMLP* target_actor;
    
    // Critic networks (twin critics)
    SigmaMLP* critic1;
    SigmaMLP* critic2;
    SigmaMLP* target_critic1;
    SigmaMLP* target_critic2;
    
    // Target policy smoothing
    double policy_noise;
    double noise_clip;
    
    // Delayed policy updates
    uint32_t policy_freq; // Update actor every N critic updates
    uint32_t update_counter;
    
    // Exploration noise
    double exploration_noise;
    
    // Dimensions and hyperparameters
    uint32_t state_dim;
    uint32_t action_dim;
    double* action_min;
    double* action_max;
    double gamma;
    double tau;
    
    // Replay buffer
    void* replay_buffer; // Same structure as SAC
    
    // Metrics
    double actor_loss;
    double critic_loss;
} SigmaTD3;

SigmaTD3* sigma_td3_create(uint32_t state_dim,
                          uint32_t action_dim,
                          double* action_min,
                          double* action_max);

void sigma_td3_select_action(SigmaTD3* td3,
                            double* state,
                            bool deterministic,
                            double* action);
void sigma_td3_train(SigmaTD3* td3,
                    SigmaRLEnvironment* env,
                    uint32_t n_episodes,
                    uint32_t max_steps);
void sigma_td3_destroy(SigmaTD3* td3);

// Proximal Policy Optimization (PPO)
typedef struct {
    // Actor-Critic network
    SigmaMLP* actor_critic; // Shared layers with separate heads
    SigmaMLP* old_actor; // For importance sampling
    
    // PPO specific parameters
    double clip_epsilon; // Usually 0.2
    double value_loss_coef;
    double entropy_coef;
    uint32_t ppo_epochs; // Number of epochs per update
    uint32_t batch_size;
    uint32_t minibatch_size;
    
    // GAE parameters
    double lambda_gae; // Usually 0.95
    double gamma;
    
    // Advantage normalization
    bool normalize_advantages;
    
    // State and action dimensions
    uint32_t state_dim;
    uint32_t action_dim;
    bool discrete_actions;
    
    // Memory (trajectory storage)
    double** states;
    double** actions;
    double* rewards;
    double* values;
    double* log_probs;
    bool* dones;
    double** next_states;
    uint32_t memory_capacity;
    uint32_t memory_size;
    
    // Metrics
    double policy_loss;
    double value_loss;
    double entropy_loss;
    double approx_kl;
    double clip_fraction;
} SigmaPPO;

SigmaPPO* sigma_ppo_create(uint32_t state_dim,
                          uint32_t action_dim,
                          bool discrete_actions,
                          double lr,
                          double gamma,
                          double clip_epsilon);

void sigma_ppo_select_action(SigmaPPO* ppo,
                            double* state,
                            double* action,
                            double* log_prob,
                            double* value);
void sigma_ppo_store_transition(SigmaPPO* ppo,
                               double* state,
                               double* action,
                               double reward,
                               bool done,
                               double* next_state,
                               double log_prob,
                               double value);
void sigma_ppo_update(SigmaPPO* ppo);
void sigma_ppo_compute_gae(SigmaPPO* ppo,
                          double* advantages,
                          double* returns);
void sigma_ppo_train(SigmaPPO* ppo,
                    SigmaRLEnvironment* env,
                    uint32_t n_updates,
                    uint32_t steps_per_update);
void sigma_ppo_destroy(SigmaPPO* ppo);

// A3C (Asynchronous Advantage Actor-Critic)
typedef struct {
    // Global network
    SigmaMLP* global_actor_critic;
    
    // Worker-specific networks (for parallel training)
    uint32_t n_workers;
    SigmaMLP** worker_networks;
    
    // Gradient accumulation
    double** accumulated_gradients;
    
    // Shared optimizer state
    double global_lr;
    
    // Entropy regularization
    double entropy_coef;
    
    // Value loss coefficient
    double value_loss_coef;
    
    // Metrics per worker
    double* worker_rewards;
    uint32_t* worker_episodes;
} SigmaA3C;

SigmaA3C* sigma_a3c_create(uint32_t state_dim,
                          uint32_t action_dim,
                          uint32_t n_workers,
                          double lr);

void sigma_a3c_start_worker(SigmaA3C* a3c,
                           uint32_t worker_id,
                           SigmaRLEnvironment* env);
void sigma_a3c_train_parallel(SigmaA3C* a3c,
                             SigmaRLEnvironment** envs,
                             uint32_t max_episodes);
void sigma_a3c_update_global(SigmaA3C* a3c, uint32_t worker_id);
void sigma_a3c_destroy(SigmaA3C* a3c);

// Trust Region Policy Optimization (TRPO)
typedef struct {
    // Policy network
    SigmaMLP* policy;
    
    // Value network
    SigmaMLP* value_function;
    
    // Trust region constraint
    double max_kl_divergence; // Usually 0.01
    double damping; // For conjugate gradient
    
    // Conjugate gradient parameters
    uint32_t cg_iters; // Usually 10
    uint32_t line_search_iters; // Usually 10
    double line_search_alpha; // Backtracking coefficient
    
    // GAE parameters
    double gamma;
    double lambda_gae;
    
    // State and action dimensions
    uint32_t state_dim;
    uint32_t action_dim;
    bool discrete_actions;
    
    // Trajectory storage
    void* trajectory_buffer;
    
    // Fisher information matrix approximation (for natural gradient)
    double** fvp_buffer; // Fisher-vector product
} SigmaTRPO;

SigmaTRPO* sigma_trpo_create(uint32_t state_dim,
                            uint32_t action_dim,
                            bool discrete_actions,
                            double max_kl,
                            double gamma);

void sigma_trpo_select_action(SigmaTRPO* trpo,
                             double* state,
                             double* action,
                             double* log_prob);
void sigma_trpo_collect_trajectories(SigmaTRPO* trpo,
                                    SigmaRLEnvironment* env,
                                    uint32_t n_steps);
void sigma_trpo_compute_surrogate_loss(SigmaTRPO* trpo,
                                      double* advantages,
                                      double* old_log_probs,
                                      double* new_log_probs,
                                      double* surrogate_loss);
void sigma_trpo_conjugate_gradient(SigmaTRPO* trpo,
                                  double* b,
                                  double* x,
                                  uint32_t max_iters);
void sigma_trpo_fisher_vector_product(SigmaTRPO* trpo,
                                     double* vector,
                                     double* result);
void sigma_trpo_update(SigmaTRPO* trpo);
void sigma_trpo_train(SigmaTRPO* trpo,
                     SigmaRLEnvironment* env,
                     uint32_t n_iterations,
                     uint32_t steps_per_iter);
void sigma_trpo_destroy(SigmaTRPO* trpo);

// Deep Deterministic Policy Gradient (DDPG)
typedef struct {
    // Actor network (deterministic policy)
    SigmaMLP* actor;
    SigmaMLP* target_actor;
    
    // Critic network (Q-function)
    SigmaMLP* critic;
    SigmaMLP* target_critic;
    
    // Exploration noise (Ornstein-Uhlenbeck process)
    double* ou_noise_state;
    double ou_theta;
    double ou_mu;
    double ou_sigma;
    
    // Dimensions and hyperparameters
    uint32_t state_dim;
    uint32_t action_dim;
    double* action_min;
    double* action_max;
    double gamma;
    double tau;
    
    // Replay buffer
    void* replay_buffer;
    
    // Metrics
    double actor_loss;
    double critic_loss;
} SigmaDDPG;

SigmaDDPG* sigma_ddpg_create(uint32_t state_dim,
                            uint32_t action_dim,
                            double* action_min,
                            double* action_max);

void sigma_ddpg_reset_noise(SigmaDDPG* ddpg);
void sigma_ddpg_noise(SigmaDDPG* ddpg, double* noise);
void sigma_ddpg_select_action(SigmaDDPG* ddpg,
                             double* state,
                             bool add_noise,
                             double* action);
void sigma_ddpg_train(SigmaDDPG* ddpg,
                      SigmaRLEnvironment* env,
                      uint32_t n_episodes,
                      uint32_t max_steps);
void sigma_ddpg_destroy(SigmaDDPG* ddpg);

#endif // SIGMA_GENERATIVE_RL_H

