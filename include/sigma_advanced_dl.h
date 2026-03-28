/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Advanced Deep Learning Architectures
 * ==============================================
 * State-of-the-art deep learning models:
 * - ResNet, DenseNet, EfficientNet
 * - Vision Transformers (ViT)
 * - Inception, MobileNet
 * - U-Net, Segmentation models
 * - YOLO, Object Detection
 * - Siamese Networks, Triplet Loss
 */

#ifndef SIGMA_ADVANCED_DL_H
#define SIGMA_ADVANCED_DL_H

#include "sigma_neural_networks.h"

// ==================== RESNET ARCHITECTURES ====================

typedef struct {
    // Residual block components
    uint32_t n_blocks;
    uint32_t* block_channels;
    uint32_t* block_strides;
    
    // Skip connection type
    bool use_bottleneck; // True for ResNet50+, False for ResNet18/34
    
    // Batch normalization
    bool use_bn;
    double bn_momentum;
    
    // Architecture parameters
    uint32_t input_channels;
    uint32_t n_classes;
    uint32_t initial_filters;
    
    // Pre-trained weights
    bool use_pretrained;
    char pretrained_weights_path[1024];
    
    // Layers (simplified representation)
    SigmaLayer* initial_conv;
    SigmaLayer** residual_blocks;
    SigmaLayer* global_avg_pool;
    SigmaLayer* fc_layer;
    
    double train_accuracy;
    double val_accuracy;
} SigmaResNet;

// ResNet variants
SigmaResNet* sigma_resnet18_create(uint32_t n_classes, bool pretrained);
SigmaResNet* sigma_resnet34_create(uint32_t n_classes, bool pretrained);
SigmaResNet* sigma_resnet50_create(uint32_t n_classes, bool pretrained);
SigmaResNet* sigma_resnet101_create(uint32_t n_classes, bool pretrained);
SigmaResNet* sigma_resnet152_create(uint32_t n_classes, bool pretrained);

// Wide ResNet
SigmaResNet* sigma_wide_resnet_create(uint32_t depth, uint32_t width_factor, 
                                      uint32_t n_classes);

// ResNeXt
SigmaResNet* sigma_resnext_create(uint32_t cardinality, uint32_t depth, 
                                  uint32_t n_classes);

void sigma_resnet_add_residual_block(SigmaResNet* model, uint32_t in_channels,
                                      uint32_t out_channels, uint32_t stride,
                                      bool use_bottleneck);
void sigma_resnet_forward(SigmaResNet* model, SigmaMatrix* input, 
                          SigmaMatrix* output);
void sigma_resnet_train(SigmaResNet* model, SigmaDataset* data, 
                        uint32_t epochs, uint32_t batch_size);
void sigma_resnet_save(SigmaResNet* model, const char* path);
void sigma_resnet_load(SigmaResNet* model, const char* path);
void sigma_resnet_destroy(SigmaResNet* model);

// ==================== DENSENET ====================

typedef struct {
    // Dense block parameters
    uint32_t growth_rate;
    uint32_t n_dense_blocks;
    uint32_t* n_layers_per_block;
    double compression_factor; // For transition layers
    
    // Architecture
    uint32_t input_channels;
    uint32_t n_classes;
    uint32_t initial_num_features;
    
    // Dropout
    double dropout_rate;
    
    // Memory efficient mode
    bool memory_efficient;
    
    // Layers
    SigmaLayer* initial_conv;
    SigmaLayer** dense_blocks;
    SigmaLayer** transition_layers;
    SigmaLayer* bn_final;
    SigmaLayer* fc_layer;
    
    double train_accuracy;
    double val_accuracy;
} SigmaDenseNet;

// DenseNet variants
SigmaDenseNet* sigma_densenet121_create(uint32_t n_classes, bool pretrained);
SigmaDenseNet* sigma_densenet169_create(uint32_t n_classes, bool pretrained);
SigmaDenseNet* sigma_densenet201_create(uint32_t n_classes, bool pretrained);
SigmaDenseNet* sigma_densenet264_create(uint32_t n_classes, bool pretrained);

// Custom DenseNet builder
SigmaDenseNet* sigma_densenet_custom_create(uint32_t growth_rate,
                                            uint32_t block_config[4],
                                            uint32_t n_classes);

void sigma_densenet_add_dense_block(SigmaDenseNet* model, uint32_t num_layers,
                                   uint32_t num_input_features);
void sigma_densenet_add_transition(SigmaDenseNet* model, uint32_t num_input_features,
                                  uint32_t num_output_features);
void sigma_densenet_train(SigmaDenseNet* model, SigmaDataset* data,
                         uint32_t epochs, uint32_t batch_size);
void sigma_densenet_destroy(SigmaDenseNet* model);

// ==================== EFFICIENTNET ====================

typedef enum {
    SIGMA_EFFICIENTNET_B0,
    SIGMA_EFFICIENTNET_B1,
    SIGMA_EFFICIENTNET_B2,
    SIGMA_EFFICIENTNET_B3,
    SIGMA_EFFICIENTNET_B4,
    SIGMA_EFFICIENTNET_B5,
    SIGMA_EFFICIENTNET_B6,
    SIGMA_EFFICIENTNET_B7
} SigmaEfficientNetVersion;

typedef struct {
    // Compound scaling parameters
    double width_coefficient;
    double depth_coefficient;
    double resolution_coefficient;
    double dropout_rate;
    
    // Architecture
    SigmaEfficientNetVersion version;
    uint32_t input_resolution;
    uint32_t n_classes;
    
    // Mobile inverted bottleneck convolution (MBConv) parameters
    uint32_t expand_ratio;
    uint32_t kernel_size;
    double se_ratio; // Squeeze-and-excitation ratio
    
    // Swish activation
    bool use_swish;
    
    // Layers
    SigmaLayer* stem_conv;
    SigmaLayer** mbconv_blocks;
    SigmaLayer* se_layers;
    SigmaLayer* final_conv;
    SigmaLayer* fc_layer;
    
    double train_accuracy;
    double val_accuracy;
} SigmaEfficientNet;

SigmaEfficientNet* sigma_efficientnet_create(SigmaEfficientNetVersion version,
                                            uint32_t n_classes,
                                            bool pretrained);
void sigma_efficientnet_compound_scale(SigmaEfficientNet* model,
                                      double width_mult,
                                      double depth_mult,
                                      double resolution_mult);
void sigma_efficientnet_add_mbconv_block(SigmaEfficientNet* model,
                                        uint32_t in_channels,
                                        uint32_t out_channels,
                                        uint32_t expand_ratio,
                                        uint32_t kernel_size,
                                        uint32_t stride,
                                        double se_ratio);
void sigma_efficientnet_add_se_layer(SigmaEfficientNet* model,
                                     uint32_t channels,
                                     double reduction_ratio);
void sigma_efficientnet_train(SigmaEfficientNet* model, SigmaDataset* data,
                             uint32_t epochs, uint32_t batch_size);
void sigma_efficientnet_destroy(SigmaEfficientNet* model);

// ==================== VISION TRANSFORMER (ViT) ====================

typedef struct {
    // Patch embedding parameters
    uint32_t image_size;
    uint32_t patch_size;
    uint32_t n_channels;
    uint32_t n_patches;
    uint32_t patch_dim;
    uint32_t embed_dim;
    
    // Transformer parameters
    uint32_t n_layers;
    uint32_t n_heads;
    uint32_t mlp_dim;
    double dropout;
    double attention_dropout;
    
    // Classification head
    uint32_t n_classes;
    char pool_type[16]; // "cls", "mean", "max"
    
    // Components
    SigmaLayer* patch_embedding;
    SigmaVector* cls_token;
    SigmaMatrix* position_embedding;
    SigmaTransformerEncoderLayer* transformer_layers;
    SigmaLayer* mlp_head;
    
    // Pre-training
    bool use_pretrained_imagenet;
    char pretrained_path[1024];
    
    double train_accuracy;
    double val_accuracy;
} SigmaVisionTransformer;

// ViT variants
SigmaVisionTransformer* sigma_vit_tiny_create(uint32_t image_size, 
                                              uint32_t n_classes);
SigmaVisionTransformer* sigma_vit_small_create(uint32_t image_size,
                                              uint32_t n_classes);
SigmaVisionTransformer* sigma_vit_base_create(uint32_t image_size,
                                             uint32_t n_classes);
SigmaVisionTransformer* sigma_vit_large_create(uint32_t image_size,
                                              uint32_t n_classes);
SigmaVisionTransformer* sigma_vit_huge_create(uint32_t image_size,
                                             uint32_t n_classes);

// DeiT (Data-efficient Image Transformer)
SigmaVisionTransformer* sigma_deit_create(uint32_t image_size,
                                         uint32_t n_classes);

// Swin Transformer (Hierarchical Vision Transformer)
typedef struct {
    uint32_t image_size;
    uint32_t patch_size;
    uint32_t embed_dim;
    uint32_t n_classes;
    
    // Hierarchical stages
    uint32_t n_stages;
    uint32_t* depths; // Number of blocks per stage
    uint32_t* num_heads; // Number of attention heads per stage
    uint32_t* window_size; // Window size for local attention
    
    // Shifted window
    bool use_shifted_window;
    
    // Layers
    SigmaLayer* patch_embed;
    SigmaTransformerEncoderLayer** stages;
    SigmaLayer* patch_merging; // Downsample between stages
    SigmaLayer* norm;
    SigmaLayer* head;
} SigmaSwinTransformer;

SigmaSwinTransformer* sigma_swin_tiny_create(uint32_t image_size, 
                                            uint32_t n_classes);
SigmaSwinTransformer* sigma_swin_small_create(uint32_t image_size,
                                             uint32_t n_classes);
SigmaSwinTransformer* sigma_swin_base_create(uint32_t image_size,
                                            uint32_t n_classes);

void sigma_vit_train(SigmaVisionTransformer* vit, SigmaDataset* data,
                    uint32_t epochs, uint32_t batch_size);
void sigma_vit_finetune(SigmaVisionTransformer* vit, SigmaDataset* data,
                       uint32_t epochs, double lr);
void sigma_vit_destroy(SigmaVisionTransformer* vit);
void sigma_swin_destroy(SigmaSwinTransformer* swin);

// ==================== INCEPTION ====================

typedef struct {
    // Inception module parameters
    uint32_t n_inception_modules;
    uint32_t* filter_sizes; // 1x1, 3x3, 5x5, pool projections
    
    // Architecture
    uint32_t n_classes;
    uint32_t input_channels;
    
    // Auxiliary classifiers (for Inception-v3)
    bool use_auxiliary_classifiers;
    
    // Layers
    SigmaLayer* initial_conv;
    SigmaLayer** inception_modules;
    SigmaLayer* auxiliary_classifiers[2];
    SigmaLayer* final_fc;
    
    double train_accuracy;
    double val_accuracy;
} SigmaInception;

SigmaInception* sigma_inception_v3_create(uint32_t n_classes, bool pretrained);
SigmaInception* sigma_inception_resnet_v2_create(uint32_t n_classes, bool pretrained);
SigmaInception* sigma_xception_create(uint32_t n_classes, bool pretrained);

void sigma_inception_add_module(SigmaInception* model, uint32_t in_channels,
                               uint32_t c1x1, uint32_t c3x3_reduce, uint32_t c3x3,
                               uint32_t c5x5_reduce, uint32_t c5x5, 
                               uint32_t pool_proj);
void sigma_inception_train(SigmaInception* model, SigmaDataset* data,
                          uint32_t epochs, uint32_t batch_size);
void sigma_inception_destroy(SigmaInception* model);

// ==================== MOBILENET ====================

typedef struct {
    // Depthwise separable convolution
    uint32_t width_multiplier;
    double resolution_multiplier;
    
    // Architecture
    uint32_t n_classes;
    uint32_t input_resolution;
    
    // Inverted residual and linear bottleneck (V2)
    bool use_inverted_residuals;
    uint32_t expansion_factor;
    
    // Layers
    SigmaLayer* stem_conv;
    SigmaLayer** inverted_residual_blocks;
    SigmaLayer* final_conv;
    SigmaLayer* classifier;
    
    double train_accuracy;
    double val_accuracy;
} SigmaMobileNet;

SigmaMobileNet* sigma_mobilenet_v1_create(uint32_t n_classes, float width_mult,
                                         bool pretrained);
SigmaMobileNet* sigma_mobilenet_v2_create(uint32_t n_classes, float width_mult,
                                         bool pretrained);
SigmaMobileNet* sigma_mobilenet_v3_small_create(uint32_t n_classes, bool pretrained);
SigmaMobileNet* sigma_mobilenet_v3_large_create(uint32_t n_classes, bool pretrained);

void sigma_mobilenet_add_depthwise_separable(SigmaMobileNet* model,
                                            uint32_t in_channels,
                                            uint32_t out_channels,
                                            uint32_t stride);
void sigma_mobilenet_add_inverted_residual(SigmaMobileNet* model,
                                          uint32_t in_channels,
                                          uint32_t out_channels,
                                          uint32_t expansion_factor,
                                          uint32_t stride,
                                          bool use_se); // Squeeze-and-Excite
void sigma_mobilenet_train(SigmaMobileNet* model, SigmaDataset* data,
                          uint32_t epochs, uint32_t batch_size);
void sigma_mobilenet_destroy(SigmaMobileNet* model);

// ==================== U-NET (SEGMENTATION) ====================

typedef struct {
    // Encoder (contracting path)
    uint32_t n_encoder_blocks;
    uint32_t* encoder_channels; // Channels per level
    
    // Decoder (expansive path)
    uint32_t n_decoder_blocks;
    uint32_t* decoder_channels;
    
    // Skip connections
    bool use_skip_connections;
    
    // Output
    uint32_t n_classes; // Number of segmentation classes
    uint32_t input_channels;
    uint32_t input_height;
    uint32_t input_width;
    
    // Architecture variants
    bool use_attention; // Attention U-Net
    bool use_residual; // Residual U-Net
    bool use_nested; // U-Net++ (nested skip connections)
    
    // Layers
    SigmaLayer** encoder_convs;
    SigmaLayer** encoder_pools;
    SigmaLayer** decoder_upsamples;
    SigmaLayer** decoder_convs;
    SigmaLayer* final_conv; // 1x1 conv for classification
    
    // Skip connection layers
    SigmaLayer** skip_connections;
    
    double train_iou;
    double val_iou;
    double train_dice;
    double val_dice;
} SigmaUNet;

SigmaUNet* sigma_unet_create(uint32_t n_classes, uint32_t input_channels,
                            uint32_t base_channels);
SigmaUNet* sigma_attention_unet_create(uint32_t n_classes, 
                                      uint32_t input_channels);
SigmaUNet* sigma_unet_plus_plus_create(uint32_t n_classes,
                                      uint32_t input_channels,
                                      uint32_t n_nested_layers);

void sigma_unet_add_encoder_block(SigmaUNet* model, uint32_t in_channels,
                                 uint32_t out_channels);
void sigma_unet_add_decoder_block(SigmaUNet* model, uint32_t in_channels,
                                 uint32_t out_channels);
void sigma_unet_add_attention_gate(SigmaUNet* model, uint32_t g_channels,
                                 uint32_t x_channels, uint32_t intermediate_channels);
void sigma_unet_train(SigmaUNet* model, SigmaDataset* data,
                    uint32_t epochs, uint32_t batch_size);
SigmaMatrix* sigma_unet_predict_segmentation(SigmaUNet* model, SigmaMatrix* input);
void sigma_unet_destroy(SigmaUNet* model);

// ==================== OBJECT DETECTION (YOLO) ====================

typedef struct {
    // Detection parameters
    uint32_t n_classes;
    uint32_t n_anchors;
    uint32_t* anchor_boxes[3]; // For 3 scales in YOLOv3/v4
    uint32_t n_boxes_per_scale;
    
    // Grid sizes for multi-scale detection
    uint32_t grid_sizes[3]; // 13x13, 26x26, 52x52 for 416 input
    
    // Architecture
    char backbone[32]; // "darknet53", "cspdarknet53", "efficientnet"
    uint32_t input_size;
    
    // YOLO specific
    double iou_threshold;
    double score_threshold;
    double nms_threshold; // Non-max suppression
    
    // Feature Pyramid Network (FPN) / Path Aggregation Network (PAN)
    bool use_fpn;
    bool use_pan;
    
    // Layers
    SigmaCNN* backbone_cnn;
    SigmaLayer** detection_heads; // For each scale
    SigmaLayer* fpn_layers;
    SigmaLayer* pan_layers;
    
    // Loss weights
    double lambda_coord;
    double lambda_noobj;
    double lambda_obj;
    double lambda_cls;
    
    double train_map;
    double val_map;
} SigmaYOLO;

// YOLO variants
SigmaYOLO* sigma_yolo_v3_create(uint32_t n_classes, uint32_t input_size);
SigmaYOLO* sigma_yolo_v4_create(uint32_t n_classes, uint32_t input_size);
SigmaYOLO* sigma_yolo_v5_create(uint32_t n_classes, char* model_size); // "s", "m", "l", "x"
SigmaYOLO* sigma_yolo_tiny_create(uint32_t n_classes);

// SSD (Single Shot MultiBox Detector)
typedef struct {
    uint32_t n_classes;
    uint32_t input_size;
    uint32_t n_anchors;
    
    // Feature maps for detection
    uint32_t* feature_map_sizes;
    uint32_t n_feature_maps;
    
    // Default boxes (aspect ratios)
    double** aspect_ratios;
    
    // VGG or ResNet backbone
    char backbone[32];
    
    SigmaCNN* backbone_net;
    SigmaLayer** extra_layers;
    SigmaLayer** detection_layers;
    
    double train_map;
    double val_map;
} SigmaSSD;

SigmaSSD* sigma_ssd300_create(uint32_t n_classes, bool pretrained);
SigmaSSD* sigma_ssd512_create(uint32_t n_classes, bool pretrained);

// Faster R-CNN
typedef struct {
    uint32_t n_classes;
    
    // Region Proposal Network
    uint32_t rpn_n_anchors;
    double rpn_positive_iou_threshold;
    double rpn_negative_iou_threshold;
    
    // ROI Pooling / ROI Align
    uint32_t roi_output_size;
    char roi_type[16]; // "pool", "align"
    
    // Backbone
    char backbone[32]; // "resnet50", "resnet101"
    
    // Components
    SigmaResNet* backbone_net;
    SigmaLayer* rpn_conv;
    SigmaLayer* rpn_cls;
    SigmaLayer* rpn_bbox;
    SigmaLayer* roi_align;
    SigmaLayer* fast_rcnn_head;
    
    double train_map;
    double val_map;
} SigmaFasterRCNN;

SigmaFasterRCNN* sigma_faster_rcnn_create(uint32_t n_classes, 
                                         const char* backbone,
                                         bool pretrained);

void sigma_yolo_train(SigmaYOLO* yolo, SigmaDataset* data,
                     uint32_t epochs, uint32_t batch_size);
void sigma_yolo_detect(SigmaYOLO* yolo, SigmaMatrix* image,
                      double** boxes,      // [x, y, w, h]
                      double** scores,   // Confidence scores
                      uint32_t** classes, // Class IDs
                      uint32_t* n_detections);
void sigma_yolo_nms(double** boxes, double** scores, uint32_t* classes,
                   uint32_t n_detections, double iou_threshold,
                   double** nms_boxes, uint32_t* n_nms_detections);
void sigma_yolo_destroy(SigmaYOLO* yolo);
void sigma_ssd_destroy(SigmaSSD* ssd);
void sigma_faster_rcnn_destroy(SigmaFasterRCNN* rcnn);

// ==================== SIAMESE NETWORKS & METRIC LEARNING ====================

typedef struct {
    // Twin networks (shared weights)
    SigmaCNN* base_network;
    
    // Distance metric
    char distance_metric[16]; // "euclidean", "cosine", "manhattan"
    
    // Contrastive loss margin
    double contrastive_margin;
    
    // Triplet loss margin (if using triplet)
    double triplet_margin;
    
    // Loss type
    char loss_type[16]; // "contrastive", "triplet", "softmax"
    
    // Embedding dimension
    uint32_t embedding_dim;
    
    // Training
    double train_loss;
    double val_loss;
} SigmaSiameseNetwork;

SigmaSiameseNetwork* sigma_siamese_create(SigmaCNN* base_network,
                                        uint32_t embedding_dim,
                                        const char* distance_metric);
void sigma_siamese_train_contrastive(SigmaSiameseNetwork* siamese,
                                    SigmaDataset* pairs, // (img1, img2, label)
                                    uint32_t epochs,
                                    uint32_t batch_size);
void sigma_siamese_train_triplet(SigmaSiameseNetwork* siamese,
                                SigmaDataset* triplets, // (anchor, pos, neg)
                                uint32_t epochs,
                                uint32_t batch_size);
double sigma_siamese_compute_distance(SigmaSiameseNetwork* siamese,
                                     SigmaMatrix* image1,
                                     SigmaMatrix* image2);
bool sigma_siamese_verify(SigmaSiameseNetwork* siamese,
                         SigmaMatrix* image1,
                         SigmaMatrix* image2,
                         double threshold);
void sigma_siamese_destroy(SigmaSiameseNetwork* siamese);

// ==================== PRE-TRAINED MODEL UTILITIES ====================

// Model zoo
void sigma_model_zoo_list_available(void);
void sigma_model_zoo_download(const char* model_name, const char* save_path);
void sigma_model_zoo_load_weights(void* model, const char* model_name);

// Transfer learning utilities
void sigma_freeze_layers(void* model, uint32_t n_layers_to_freeze);
void sigma_unfreeze_layers(void* model);
void sigma_set_layer_trainable(void* model, uint32_t layer_idx, bool trainable);

// Fine-tuning
void sigma_fine_tune_model(void* model, SigmaDataset* data,
                          uint32_t epochs, double learning_rate);

// Feature extraction
SigmaMatrix* sigma_extract_features(void* model, SigmaMatrix* images,
                                   const char* layer_name);

#endif // SIGMA_ADVANCED_DL_H

