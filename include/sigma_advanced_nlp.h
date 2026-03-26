/*
 * SigmaOS Advanced NLP Architectures
 * ==================================
 * State-of-the-art NLP models:
 * - BERT, RoBERTa, DistilBERT
 * - GPT, GPT-2, GPT-3 style
 * - T5, BART (seq2seq)
 * - ELECTRA, ALBERT (efficient variants)
 * - Advanced tokenizers
 */

#ifndef SIGMA_ADVANCED_NLP_H
#define SIGMA_ADVANCED_NLP_H

#include "sigma_neural_networks.h"

// ==================== BERT (Bidirectional Encoder Representations) ====================

typedef enum {
    SIGMA_BERT_BASE,      // 12 layers, 768 hidden, 12 heads, 110M params
    SIGMA_BERT_LARGE,     // 24 layers, 1024 hidden, 16 heads, 340M params
    SIGMA_BERT_TINY,      // 2 layers, 128 hidden, 2 heads
    SIGMA_BERT_MINI,      // 4 layers, 256 hidden, 4 heads
    SIGMA_BERT_SMALL,     // 4 layers, 512 hidden, 8 heads
    SIGMA_BERT_MEDIUM     // 8 layers, 512 hidden, 8 heads
} SigmaBERTSize;

typedef struct {
    // Architecture
    SigmaBERTSize size;
    uint32_t vocab_size;
    uint32_t max_position_embeddings;
    uint32_t type_vocab_size; // For segment embeddings (2 for BERT)
    uint32_t hidden_size;
    uint32_t num_hidden_layers;
    uint32_t num_attention_heads;
    uint32_t intermediate_size; // Feed-forward dim
    double hidden_dropout_prob;
    double attention_probs_dropout_prob;
    
    // Embeddings
    SigmaMatrix* word_embeddings;
    SigmaMatrix* position_embeddings;
    SigmaMatrix* token_type_embeddings;
    SigmaVector* layer_norm_gamma;
    SigmaVector* layer_norm_beta;
    
    // Transformer encoder layers
    SigmaTransformerEncoderLayer* encoder_layers;
    
    // Pooler (for [CLS] token representation)
    SigmaMatrix* pooler_dense;
    SigmaVector* pooler_bias;
    
    // Pre-training heads
    SigmaMatrix* mlm_dense; // Masked LM prediction
    SigmaVector* mlm_bias;
    SigmaMatrix* mlm_layer_norm_gamma;
    SigmaVector* mlm_layer_norm_beta;
    SigmaMatrix* mlm_decoder; // Vocab projection
    SigmaVector* nsp_classifier; // Next sentence prediction
    
    // Fine-tuning task heads
    char task_type[32]; // "classification", "regression", "ner", "qa", "similarity"
    SigmaMatrix* task_classifier;
    
    // Pre-training config
    double mlm_probability; // 0.15 default
    bool use_nsp; // Next sentence prediction
    
    // Metrics
    double train_loss;
    double val_loss;
    double train_accuracy;
    double val_accuracy;
    double perplexity;
    
    // Pre-trained weights
    bool use_pretrained;
    char pretrained_path[1024];
} SigmaBERT;

SigmaBERT* sigma_bert_create(SigmaBERTSize size, uint32_t vocab_size);
SigmaBERT* sigma_bert_from_pretrained(const char* model_name); // "bert-base-uncased", etc.

// RoBERTa (Robustly optimized BERT approach)
SigmaBERT* sigma_roberta_create(SigmaBERTSize size, uint32_t vocab_size);
SigmaBERT* sigma_roberta_from_pretrained(const char* model_name);

// DistilBERT (distilled BERT)
SigmaBERT* sigma_distilbert_create(uint32_t vocab_size);
SigmaBERT* sigma_distilbert_from_pretrained(const char* model_name);

// ALBERT (A Lite BERT)
typedef struct {
    uint32_t vocab_size;
    uint32_t embedding_size; // Smaller than hidden_size
    uint32_t hidden_size;
    uint32_t num_hidden_layers;
    uint32_t num_attention_heads;
    uint32_t intermediate_size;
    
    // Factorized embeddings
    SigmaMatrix* word_embeddings;
    SigmaMatrix* word_embeddings_projector;
    
    // Cross-layer parameter sharing
    SigmaTransformerEncoderLayer shared_encoder_layer; // Shared across all layers
    
    // Sentence Order Prediction (SOP) instead of NSP
    SigmaVector* sop_classifier;
    
    double train_loss;
    double val_loss;
} SigmaALBERT;

SigmaALBERT* sigma_albert_create(uint32_t vocab_size, uint32_t embedding_size,
                                 uint32_t hidden_size, uint32_t num_layers);
SigmaALBERT* sigma_albert_from_pretrained(const char* model_name);

// ELECTRA (Efficiently Learning an Encoder)
typedef struct {
    // Generator (small MLM model)
    SigmaBERT* generator;
    
    // Discriminator (main model)
    SigmaBERT* discriminator;
    
    // Training
    double discriminator_loss_weight;
    double generator_loss_weight;
    double temperature;
    
    // Replaced token detection
    uint32_t* replaced_token_positions;
    bool* is_replaced;
    
    double train_loss;
    double val_loss;
    double replaced_token_accuracy;
} SigmaELECTRA;

SigmaELECTRA* sigma_electra_create(SigmaBERTSize size, uint32_t vocab_size);
void sigma_electra_pretrain(SigmaELECTRA* electra, char** texts,
                           uint32_t n_texts, uint32_t epochs);

// BERT functions
void sigma_bert_tokenize(SigmaBERT* bert, const char* text,
                        uint32_t* input_ids,
                        uint32_t* attention_mask,
                        uint32_t* token_type_ids,
                        uint32_t* n_tokens);
void sigma_bert_encode(SigmaBERT* bert,
                      uint32_t* input_ids,
                      uint32_t* attention_mask,
                      uint32_t n_tokens,
                      SigmaMatrix* output_embeddings); // [n_tokens, hidden_size]
void sigma_bert_get_pooled_output(SigmaBERT* bert,
                                  SigmaMatrix* sequence_output,
                                  SigmaVector* pooled_output); // [CLS] representation

// Pre-training
void sigma_bert_pretrain_mlm(SigmaBERT* bert, char** texts,
                            uint32_t n_texts, uint32_t epochs);
void sigma_bert_pretrain_nsp(SigmaBERT* bert, char** sentence_pairs,
                            bool* is_next, uint32_t n_pairs, uint32_t epochs);
void sigma_bert_pretrain_mlm_nsp(SigmaBERT* bert, char** texts,
                                uint32_t n_texts, uint32_t epochs);

// Fine-tuning
void sigma_bert_finetune_classification(SigmaBERT* bert,
                                         char** texts,
                                         SigmaVector* labels,
                                         uint32_t n_samples,
                                         uint32_t epochs,
                                         uint32_t num_labels);
void sigma_bert_finetune_ner(SigmaBERT* bert,
                            char** tokens,
                            uint32_t** token_ids,
                            uint32_t* labels,
                            uint32_t n_samples,
                            uint32_t epochs,
                            uint32_t num_tags);
void sigma_bert_finetune_qa(SigmaBERT* bert,
                           char** questions,
                           char** contexts,
                           uint32_t* start_positions,
                           uint32_t* end_positions,
                           uint32_t n_samples,
                           uint32_t epochs);

// Inference
int sigma_bert_predict_classification(SigmaBERT* bert, const char* text);
void sigma_bert_predict_ner(SigmaBERT* bert, const char* text,
                           char** tokens, uint32_t** tags, uint32_t* n_tokens);
void sigma_bert_answer_question(SigmaBERT* bert, const char* question,
                               const char* context,
                               char* answer, uint32_t* start_idx, uint32_t* end_idx);
double sigma_bert_compute_similarity(SigmaBERT* bert, const char* text1,
                                    const char* text2);

void sigma_bert_save(SigmaBERT* bert, const char* path);
void sigma_bert_load(SigmaBERT* bert, const char* path);
void sigma_bert_destroy(SigmaBERT* bert);
void sigma_albert_destroy(SigmaALBERT* albert);
void sigma_electra_destroy(SigmaELECTRA* electra);

// ==================== GPT (Generative Pre-trained Transformer) ====================

typedef enum {
    SIGMA_GPT_SMALL,   // 117M params (GPT)
    SIGMA_GPT_MEDIUM,  // 345M params
    SIGMA_GPT_LARGE,   // 762M params
    SIGMA_GPT_XL,      // 1.5B params
    SIGMA_GPT2_SMALL,  // 124M params
    SIGMA_GPT2_MEDIUM, // 355M params
    SIGMA_GPT2_LARGE,  // 774M params
    SIGMA_GPT2_XL      // 1.5B params
} SigmaGPTSize;

typedef struct {
    // Architecture
    SigmaGPTSize size;
    uint32_t vocab_size;
    uint32_t n_positions;
    uint32_t n_ctx; // Context window size
    uint32_t n_embd; // Embedding dimension
    uint32_t n_layer; // Number of transformer layers
    uint32_t n_head; // Number of attention heads
    double dropout;
    double attn_dropout;
    double resid_dropout;
    
    // Embeddings
    SigmaMatrix* wte; // Word token embeddings [vocab_size, n_embd]
    SigmaMatrix* wpe; // Positional embeddings [n_positions, n_embd]
    
    // Transformer decoder layers (causal attention)
    struct {
        SigmaMatrix* ln_1_gamma;
        SigmaVector* ln_1_beta;
        SigmaMatrix* attn_c_attn_w; // Causal attention weights
        SigmaVector* attn_c_attn_b;
        SigmaMatrix* attn_c_proj_w;
        SigmaVector* attn_c_proj_b;
        SigmaMatrix* ln_2_gamma;
        SigmaVector* ln_2_beta;
        SigmaMatrix* mlp_c_fc_w; // Feed-forward up-project
        SigmaVector* mlp_c_fc_b;
        SigmaMatrix* mlp_c_proj_w; // Feed-forward down-project
        SigmaVector* mlp_c_proj_b;
    }* decoder_layers;
    
    // Final layer norm
    SigmaMatrix* ln_f_gamma;
    SigmaVector* ln_f_beta;
    
    // Language modeling head (tied with wte)
    SigmaMatrix* lm_head;
    bool tie_weights;
    
    // Generation config
    uint32_t max_length;
    double temperature;
    double top_p; // Nucleus sampling
    uint32_t top_k;
    uint32_t repetition_penalty;
    bool do_sample;
    uint32_t num_return_sequences;
    double length_penalty;
    
    // Metrics
    double train_loss;
    double val_loss;
    double perplexity;
    
    // Pre-trained
    bool use_pretrained;
    char pretrained_path[1024];
} SigmaGPT;

SigmaGPT* sigma_gpt_create(SigmaGPTSize size, uint32_t vocab_size);
SigmaGPT* sigma_gpt2_create(SigmaGPTSize size, uint32_t vocab_size);
SigmaGPT* sigma_gpt_from_pretrained(const char* model_name);

// GPT-Neo (open-source GPT-3 replica)
typedef struct {
    uint32_t vocab_size;
    uint32_t n_positions;
    uint32_t n_embd;
    uint32_t n_layer;
    uint32_t n_head;
    
    // Local and global attention
    uint32_t window_size; // For local attention
    bool use_local_attention;
    bool use_global_attention;
    
    // Full attention or sparse
    char attention_type[16]; // "default", "local", "sparse"
    
    // Architecture similar to GPT
    SigmaMatrix* wte;
    SigmaMatrix* wpe;
    void* decoder_layers; // Simplified
    
    double train_loss;
    double val_loss;
} SigmaGPTNeo;

SigmaGPTNeo* sigma_gpt_neo_create(uint32_t vocab_size, uint32_t n_layers);
SigmaGPTNeo* sigma_gpt_neo_from_pretrained(const char* model_name);

// GPT-J
typedef struct {
    uint32_t vocab_size;
    uint32_t n_positions;
    uint32_t n_embd;
    uint32_t n_layer;
    uint32_t n_head;
    uint32_t rotary_dim; // Rotary position embeddings
    
    // Architecture
    SigmaMatrix* wte;
    void* decoder_layers;
    
    // Uses parallel attention and feed-forward ("PArallel Attention")
    bool use_parallel_attention;
    
    double train_loss;
} SigmaGPTJ;

SigmaGPTJ* sigma_gpt_j_create(uint32_t vocab_size);
SigmaGPTJ* sigma_gpt_j_from_pretrained(const char* model_name);

// GPT functions
void sigma_gpt_tokenize(SigmaGPT* gpt, const char* text,
                       uint32_t* input_ids, uint32_t* n_tokens);
char* sigma_gpt_detokenize(SigmaGPT* gpt, uint32_t* input_ids, uint32_t n_tokens);
SigmaMatrix* sigma_gpt_forward(SigmaGPT* gpt, uint32_t* input_ids, 
                            uint32_t n_tokens);

// Pre-training
void sigma_gpt_pretrain(SigmaGPT* gpt, char** texts, uint32_t n_texts,
                       uint32_t epochs, uint32_t batch_size);

// Generation methods
char* sigma_gpt_generate_greedy(SigmaGPT* gpt, const char* prompt,
                               uint32_t max_length);
char* sigma_gpt_generate_sampling(SigmaGPT* gpt, const char* prompt,
                                uint32_t max_length, double temperature);
char* sigma_gpt_generate_top_p(SigmaGPT* gpt, const char* prompt,
                              uint32_t max_length, double top_p);
char* sigma_gpt_generate_top_k(SigmaGPT* gpt, const char* prompt,
                            uint32_t max_length, uint32_t top_k);
char* sigma_gpt_generate_beam_search(SigmaGPT* gpt, const char* prompt,
                                    uint32_t max_length, uint32_t beam_width);
char* sigma_gpt_generate_contrastive(SigmaGPT* gpt, const char* prompt,
                                   uint32_t max_length, double alpha);

// Fine-tuning
void sigma_gpt_finetune_classification(SigmaGPT* gpt, char** texts,
                                      SigmaVector* labels, uint32_t n_samples,
                                      uint32_t epochs, uint32_t num_labels);
void sigma_gpt_finetune_conditional_generation(SigmaGPT* gpt,
                                               char** inputs,
                                               char** targets,
                                               uint32_t n_samples,
                                               uint32_t epochs);

// Advanced generation
char* sigma_gpt_chat(SigmaGPT* gpt, const char* user_message,
                    char** conversation_history,
                    uint32_t n_history_turns);
char* sigma_gpt_complete_code(SigmaGPT* gpt, const char* code_prefix,
                            const char* language);
char* sigma_gpt_summarize(SigmaGPT* gpt, const char* text,
                         uint32_t max_summary_length);
char* sigma_gpt_translate(SigmaGPT* gpt, const char* text,
                         const char* source_lang,
                         const char* target_lang);

void sigma_gpt_save(SigmaGPT* gpt, const char* path);
void sigma_gpt_load(SigmaGPT* gpt, const char* path);
void sigma_gpt_destroy(SigmaGPT* gpt);
void sigma_gpt_neo_destroy(SigmaGPTNeo* neo);
void sigma_gpt_j_destroy(SigmaGPTJ* gptj);

// ==================== T5 (Text-to-Text Transfer Transformer) ====================

typedef enum {
    SIGMA_T5_SMALL,   // 60M params
    SIGMA_T5_BASE,    // 220M params
    SIGMA_T5_LARGE,   // 770M params
    SIGMA_T5_3B,      // 3B params
    SIGMA_T5_11B      // 11B params
} SigmaT5Size;

typedef struct {
    // Architecture
    SigmaT5Size size;
    uint32_t vocab_size;
    uint32_t d_model; // Model dimension
    uint32_t d_kv; // Key-value dimension
    uint32_t d_ff; // Feed-forward dimension
    uint32_t n_layers; // Number of layers (encoder = decoder usually)
    uint32_t n_heads;
    uint32_t n_positions;
    double dropout_rate;
    
    // Relative attention bias
    uint32_t relative_attention_num_buckets;
    uint32_t relative_attention_max_distance;
    SigmaMatrix* relative_attention_bias;
    
    // Shared embedding
    SigmaMatrix* shared_embedding; // Shared between encoder, decoder, and output
    
    // Encoder
    struct {
        SigmaMatrix* layer_norm_gamma;
        SigmaVector* layer_norm_beta;
        // Self-attention
        SigmaMatrix* self_attention_q_w;
        SigmaMatrix* self_attention_k_w;
        SigmaMatrix* self_attention_v_w;
        SigmaMatrix* self_attention_o_w;
        // Feed-forward
        SigmaMatrix* dense_relu_dense_wi; // Wi for GELU activation
        SigmaMatrix* dense_relu_dense_wo; // Wo
    }* encoder_layers;
    
    // Decoder
    struct {
        SigmaMatrix* layer_norm_gamma;
        SigmaVector* layer_norm_beta;
        // Self-attention (causal)
        SigmaMatrix* self_attention_q_w;
        SigmaMatrix* self_attention_k_w;
        SigmaMatrix* self_attention_v_w;
        SigmaMatrix* self_attention_o_w;
        // Cross-attention (encoder-decoder)
        SigmaMatrix* enc_dec_attention_q_w;
        SigmaMatrix* enc_dec_attention_k_w;
        SigmaMatrix* enc_dec_attention_v_w;
        SigmaMatrix* enc_dec_attention_o_w;
        // Feed-forward
        SigmaMatrix* dense_relu_dense_wi;
        SigmaMatrix* dense_relu_dense_wo;
    }* decoder_layers;
    
    // Final layer norm
    SigmaMatrix* final_layer_norm_gamma;
    SigmaVector* final_layer_norm_beta;
    
    // LM head (tied with shared embedding)
    SigmaMatrix* lm_head;
    
    // Task prefix (T5 uses task-specific prefixes)
    char task_prefix[64]; // "translate English to German:", "summarize:", etc.
    
    // Metrics
    double train_loss;
    double val_loss;
    double train_accuracy;
    double val_accuracy;
    double bleu_score;
    double rouge_l;
    
    // Pre-trained
    bool use_pretrained;
    char pretrained_path[1024];
} SigmaT5;

SigmaT5* sigma_t5_create(SigmaT5Size size, uint32_t vocab_size);
SigmaT5* sigma_t5_from_pretrained(const char* model_name);

// T5 functions
void sigma_t5_tokenize(SigmaT5* t5, const char* text,
                      uint32_t* input_ids, uint32_t* n_tokens);
void sigma_t5_encode(SigmaT5* t5, uint32_t* input_ids, uint32_t n_tokens,
                    SigmaMatrix* encoder_output);
void sigma_t5_decode(SigmaT5* t5, SigmaMatrix* encoder_output,
                    uint32_t* decoder_input_ids, uint32_t n_decoder_tokens,
                    uint32_t* output_ids, uint32_t* n_output);

// Generation
char* sigma_t5_generate(SigmaT5* t5, const char* input_text,
                       const char* task_prefix,
                       uint32_t max_length);

// Tasks
char* sigma_t5_translate(SigmaT5* t5, const char* text,
                        const char* source_lang,
                        const char* target_lang);
char* sigma_t5_summarize(SigmaT5* t5, const char* text,
                        uint32_t max_summary_length);
char* sigma_t5_answer_question(SigmaT5* t5, const char* question,
                              const char* context);
char* sigma_t5_classify(SigmaT5* t5, const char* text,
                      char** class_labels, uint32_t n_classes);
char* sigma_t5_generate_sql(SigmaT5* t5, const char* natural_language_query,
                           const char* schema_description);
char* sigma_t5_simplify(SigmaT5* t5, const char* text);
char* sigma_t5_paraphrase(SigmaT5* t5, const char* text);

// Training
void sigma_t5_pretrain(SigmaT5* t5, char** input_texts,
                      char** target_texts,
                      uint32_t n_samples, uint32_t epochs);
void sigma_t5_finetune(SigmaT5* t5, char** input_texts,
                      char** target_texts,
                      uint32_t n_samples, uint32_t epochs,
                      const char* task_prefix);

void sigma_t5_save(SigmaT5* t5, const char* path);
void sigma_t5_load(SigmaT5* t5, const char* path);
void sigma_t5_destroy(SigmaT5* t5);

// ==================== BART (Bidirectional and Auto-Regressive Transformers) ====================

typedef struct {
    uint32_t vocab_size;
    uint32_t d_model;
    uint32_t encoder_layers;
    uint32_t decoder_layers;
    uint32_t encoder_attention_heads;
    uint32_t decoder_attention_heads;
    uint32_t encoder_ffn_dim;
    uint32_t decoder_ffn_dim;
    double dropout;
    double attention_dropout;
    double activation_dropout;
    
    // BART uses standard transformer encoder-decoder with some modifications
    SigmaTransformerEncoderLayer* encoder;
    SigmaTransformerEncoderLayer* decoder; // Actually decoder but uses same struct
    
    // Classification head for sequence classification
    SigmaMatrix* classification_head;
    
    // Generation config
    double label_smoothing;
    
    // Metrics
    double train_loss;
    double val_loss;
} SigmaBART;

SigmaBART* sigma_bart_create(uint32_t vocab_size, uint32_t d_model,
                            uint32_t encoder_layers, uint32_t decoder_layers);
SigmaBART* sigma_bart_from_pretrained(const char* model_name);

void sigma_bart_train_denoising(SigmaBART* bart, char** texts,
                               uint32_t n_texts, uint32_t epochs);
char* sigma_bart_generate(SigmaBART* bart, const char* input_text,
                         uint32_t max_length);
void sigma_bart_destroy(SigmaBART* bart);

// Continue with more NLP models and utilities...
