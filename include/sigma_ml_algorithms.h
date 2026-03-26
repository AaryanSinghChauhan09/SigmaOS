/*
 * SigmaOS Comprehensive ML/AI Algorithms Implementation
 * =========================================================
 * Complete implementation of machine learning algorithms:
 * - Supervised Learning: Regression & Classification
 * - Unsupervised Learning: Clustering & Dimensionality Reduction
 * - Neural Networks & Deep Learning
 * - Ensemble Methods
 * - Statistical Methods
 */

#ifndef SIGMA_ML_ALGORITHMS_H
#define SIGMA_ML_ALGORITHMS_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <math.h>

// ==================== DATA STRUCTURES ====================

// Matrix structure for numerical operations
typedef struct {
    double** data;
    uint32_t rows;
    uint32_t cols;
    char name[64];
} SigmaMatrix;

// Vector structure
typedef struct {
    double* data;
    uint32_t size;
    char name[64];
} SigmaVector;

// Dataset structure
typedef struct {
    SigmaMatrix* X;           // Features
    SigmaVector* y;           // Labels
    uint32_t n_samples;
    uint32_t n_features;
    char** feature_names;
    char* target_name;
} SigmaDataset;

// Model hyperparameters
typedef struct {
    double learning_rate;
    uint32_t max_iterations;
    double tolerance;
    uint32_t batch_size;
    double regularization;
    uint32_t random_seed;
    bool verbose;
} SigmaHyperParams;

// ==================== LINEAR REGRESSION ====================

typedef struct {
    SigmaVector* coefficients;
    double intercept;
    double r_squared;
    double mse;
    double rmse;
    double mae;
    SigmaHyperParams params;
} SigmaLinearRegression;

// Linear Regression functions
SigmaLinearRegression* sigma_lr_create(SigmaHyperParams* params);
void sigma_lr_fit(SigmaLinearRegression* model, SigmaDataset* data);
SigmaVector* sigma_lr_predict(SigmaLinearRegression* model, SigmaMatrix* X);
void sigma_lr_evaluate(SigmaLinearRegression* model, SigmaDataset* data);
void sigma_lr_destroy(SigmaLinearRegression* model);

// ==================== LOGISTIC REGRESSION ====================

typedef struct {
    SigmaVector* coefficients;
    double intercept;
    double accuracy;
    double precision;
    double recall;
    double f1_score;
    double auc_roc;
    uint32_t n_classes;
    SigmaHyperParams params;
} SigmaLogisticRegression;

SigmaLogisticRegression* sigma_logreg_create(uint32_t n_classes, SigmaHyperParams* params);
void sigma_logreg_fit(SigmaLogisticRegression* model, SigmaDataset* data);
SigmaVector* sigma_logreg_predict(SigmaLogisticRegression* model, SigmaMatrix* X);
SigmaMatrix* sigma_logreg_predict_proba(SigmaLogisticRegression* model, SigmaMatrix* X);
void sigma_logreg_evaluate(SigmaLogisticRegression* model, SigmaDataset* data);
void sigma_logreg_destroy(SigmaLogisticRegression* model);

// ==================== DECISION TREES ====================

typedef enum {
    SIGMA_CRITERION_GINI,
    SIGMA_CRITERION_ENTROPY,
    SIGMA_CRITERION_MSE,
    SIGMA_CRITERION_MAE
} SigmaCriterion;

typedef struct SigmaTreeNode {
    uint32_t feature_index;
    double threshold;
    double value;
    bool is_leaf;
    struct SigmaTreeNode* left;
    struct SigmaTreeNode* right;
    uint32_t depth;
    uint32_t n_samples;
} SigmaTreeNode;

typedef struct {
    SigmaTreeNode* root;
    uint32_t max_depth;
    uint32_t min_samples_split;
    uint32_t min_samples_leaf;
    SigmaCriterion criterion;
    bool is_classifier;
    double accuracy;
    SigmaHyperParams params;
} SigmaDecisionTree;

SigmaDecisionTree* sigma_dt_create(bool is_classifier, SigmaHyperParams* params);
void sigma_dt_fit(SigmaDecisionTree* model, SigmaDataset* data);
SigmaVector* sigma_dt_predict(SigmaDecisionTree* model, SigmaMatrix* X);
void sigma_dt_evaluate(SigmaDecisionTree* model, SigmaDataset* data);
void sigma_dt_destroy(SigmaDecisionTree* model);
void sigma_dt_export_graphviz(SigmaDecisionTree* model, const char* filename);

// ==================== RANDOM FOREST ====================

typedef struct {
    SigmaDecisionTree** trees;
    uint32_t n_estimators;
    uint32_t max_features;
    bool bootstrap;
    double oob_score;
    SigmaHyperParams params;
} SigmaRandomForest;

SigmaRandomForest* sigma_rf_create(uint32_t n_estimators, bool is_classifier, SigmaHyperParams* params);
void sigma_rf_fit(SigmaRandomForest* model, SigmaDataset* data);
SigmaVector* sigma_rf_predict(SigmaRandomForest* model, SigmaMatrix* X);
SigmaMatrix* sigma_rf_feature_importance(SigmaRandomForest* model);
void sigma_rf_evaluate(SigmaRandomForest* model, SigmaDataset* data);
void sigma_rf_destroy(SigmaRandomForest* model);

// ==================== GRADIENT BOOSTING ====================

typedef struct {
    SigmaDecisionTree** estimators;
    double* learning_rates;
    uint32_t n_estimators;
    double subsample;
    double validation_score;
    SigmaHyperParams params;
} SigmaGradientBoosting;

SigmaGradientBoosting* sigma_gb_create(uint32_t n_estimators, SigmaHyperParams* params);
void sigma_gb_fit(SigmaGradientBoosting* model, SigmaDataset* data);
SigmaVector* sigma_gb_predict(SigmaGradientBoosting* model, SigmaMatrix* X);
void sigma_gb_evaluate(SigmaGradientBoosting* model, SigmaDataset* data);
void sigma_gb_destroy(SigmaGradientBoosting* model);

// ==================== XGBOOST ====================

typedef struct {
    SigmaMatrix* trees;
    uint32_t n_trees;
    double eta;
    uint32_t max_depth;
    double min_child_weight;
    double subsample;
    double colsample_bytree;
    double lambda;
    double alpha;
    uint32_t num_class;
    SigmaHyperParams params;
} SigmaXGBoost;

SigmaXGBoost* sigma_xgb_create(SigmaHyperParams* params);
void sigma_xgb_fit(SigmaXGBoost* model, SigmaDataset* data);
SigmaVector* sigma_xgb_predict(SigmaXGBoost* model, SigmaMatrix* X);
void sigma_xgb_evaluate(SigmaXGBoost* model, SigmaDataset* data);
void sigma_xgb_destroy(SigmaXGBoost* model);

// ==================== SUPPORT VECTOR MACHINES ====================

typedef enum {
    SIGMA_KERNEL_LINEAR,
    SIGMA_KERNEL_RBF,
    SIGMA_KERNEL_POLY,
    SIGMA_KERNEL_SIGMOID
} SigmaKernelType;

typedef struct {
    SigmaVector* support_vectors;
    SigmaVector* dual_coefficients;
    double intercept;
    SigmaKernelType kernel;
    double gamma;
    double C;
    double epsilon;
    double degree;
    double coef0;
    double accuracy;
    SigmaHyperParams params;
} SigmaSVM;

SigmaSVM* sigma_svm_create(SigmaKernelType kernel, SigmaHyperParams* params);
void sigma_svm_fit(SigmaSVM* model, SigmaDataset* data);
SigmaVector* sigma_svm_predict(SigmaSVM* model, SigmaMatrix* X);
void sigma_svm_evaluate(SigmaSVM* model, SigmaDataset* data);
void sigma_svm_destroy(SigmaSVM* model);

// ==================== NAIVE BAYES ====================

typedef enum {
    SIGMA_NB_GAUSSIAN,
    SIGMA_NB_MULTINOMIAL,
    SIGMA_NB_BERNOULLI
} SigmaNBType;

typedef struct {
    SigmaMatrix* class_priors;
    SigmaMatrix* means;
    SigmaMatrix* variances;
    uint32_t n_classes;
    SigmaNBType type;
    double accuracy;
    SigmaHyperParams params;
} SigmaNaiveBayes;

SigmaNaiveBayes* sigma_nb_create(SigmaNBType type, SigmaHyperParams* params);
void sigma_nb_fit(SigmaNaiveBayes* model, SigmaDataset* data);
SigmaVector* sigma_nb_predict(SigmaNaiveBayes* model, SigmaMatrix* X);
SigmaMatrix* sigma_nb_predict_proba(SigmaNaiveBayes* model, SigmaMatrix* X);
void sigma_nb_evaluate(SigmaNaiveBayes* model, SigmaDataset* data);
void sigma_nb_destroy(SigmaNaiveBayes* model);

// ==================== K-NEAREST NEIGHBORS ====================

typedef enum {
    SIGMA_DISTANCE_EUCLIDEAN,
    SIGMA_DISTANCE_MANHATTAN,
    SIGMA_DISTANCE_MINKOWSKI,
    SIGMA_DISTANCE_COSINE
} SigmaDistanceMetric;

typedef struct {
    SigmaMatrix* X_train;
    SigmaVector* y_train;
    uint32_t k;
    SigmaDistanceMetric metric;
    uint32_t p;
    bool weights_uniform;
    double accuracy;
    SigmaHyperParams params;
} SigmaKNN;

SigmaKNN* sigma_knn_create(uint32_t k, SigmaDistanceMetric metric, SigmaHyperParams* params);
void sigma_knn_fit(SigmaKNN* model, SigmaDataset* data);
SigmaVector* sigma_knn_predict(SigmaKNN* model, SigmaMatrix* X);
void sigma_knn_evaluate(SigmaKNN* model, SigmaDataset* data);
void sigma_knn_destroy(SigmaKNN* model);

// ==================== K-MEANS CLUSTERING ====================

typedef struct {
    SigmaMatrix* centroids;
    SigmaVector* labels;
    uint32_t k;
    uint32_t max_iter;
    double inertia;
    double silhouette_score;
    SigmaHyperParams params;
} SigmaKMeans;

SigmaKMeans* sigma_kmeans_create(uint32_t k, SigmaHyperParams* params);
void sigma_kmeans_fit(SigmaKMeans* model, SigmaMatrix* X);
SigmaVector* sigma_kmeans_predict(SigmaKMeans* model, SigmaMatrix* X);
SigmaMatrix* sigma_kmeans_transform(SigmaKMeans* model, SigmaMatrix* X);
void sigma_kmeans_evaluate(SigmaKMeans* model, SigmaMatrix* X);
void sigma_kmeans_destroy(SigmaKMeans* model);

// ==================== DBSCAN ====================

typedef struct {
    SigmaVector* labels;
    double eps;
    uint32_t min_samples;
    uint32_t n_clusters;
    uint32_t n_noise;
    SigmaHyperParams params;
} SigmaDBSCAN;

SigmaDBSCAN* sigma_dbscan_create(double eps, uint32_t min_samples, SigmaHyperParams* params);
void sigma_dbscan_fit(SigmaDBSCAN* model, SigmaMatrix* X);
SigmaVector* sigma_dbscan_predict(SigmaDBSCAN* model, SigmaMatrix* X);
void sigma_dbscan_evaluate(SigmaDBSCAN* model, SigmaMatrix* X);
void sigma_dbscan_destroy(SigmaDBSCAN* model);

// ==================== HIERARCHICAL CLUSTERING ====================

typedef struct {
    SigmaMatrix* linkage_matrix;
    SigmaVector* labels;
    uint32_t n_clusters;
    double distance_threshold;
    SigmaHyperParams params;
} SigmaHierarchicalClustering;

SigmaHierarchicalClustering* sigma_hc_create(uint32_t n_clusters, SigmaHyperParams* params);
void sigma_hc_fit(SigmaHierarchicalClustering* model, SigmaMatrix* X);
SigmaVector* sigma_hc_predict(SigmaHierarchicalClustering* model, SigmaMatrix* X);
void sigma_hc_destroy(SigmaHierarchicalClustering* model);

// ==================== PCA ====================

typedef struct {
    SigmaMatrix* components;
    SigmaVector* explained_variance;
    SigmaVector* explained_variance_ratio;
    uint32_t n_components;
    SigmaMatrix* mean;
    SigmaHyperParams params;
} SigmaPCA;

SigmaPCA* sigma_pca_create(uint32_t n_components, SigmaHyperParams* params);
void sigma_pca_fit(SigmaPCA* model, SigmaMatrix* X);
SigmaMatrix* sigma_pca_transform(SigmaPCA* model, SigmaMatrix* X);
SigmaMatrix* sigma_pca_inverse_transform(SigmaPCA* model, SigmaMatrix* X);
void sigma_pca_destroy(SigmaPCA* model);

// ==================== UTILITY FUNCTIONS ====================

// Matrix operations
SigmaMatrix* sigma_matrix_create(uint32_t rows, uint32_t cols);
void sigma_matrix_destroy(SigmaMatrix* mat);
void sigma_matrix_fill(SigmaMatrix* mat, double value);
void sigma_matrix_random(SigmaMatrix* mat, double min, double max);
SigmaMatrix* sigma_matrix_transpose(SigmaMatrix* mat);
SigmaMatrix* sigma_matrix_multiply(SigmaMatrix* A, SigmaMatrix* B);
SigmaMatrix* sigma_matrix_add(SigmaMatrix* A, SigmaMatrix* B);
SigmaMatrix* sigma_matrix_subtract(SigmaMatrix* A, SigmaMatrix* B);
SigmaMatrix* sigma_matrix_scale(SigmaMatrix* mat, double scalar);
double sigma_matrix_determinant(SigmaMatrix* mat);
SigmaMatrix* sigma_matrix_inverse(SigmaMatrix* mat);
void sigma_matrix_print(SigmaMatrix* mat);

// Vector operations
SigmaVector* sigma_vector_create(uint32_t size);
void sigma_vector_destroy(SigmaVector* vec);
double sigma_vector_dot(SigmaVector* a, SigmaVector* b);
double sigma_vector_norm(SigmaVector* vec);
void sigma_vector_normalize(SigmaVector* vec);
void sigma_vector_fill(SigmaVector* vec, double value);
void sigma_vector_print(SigmaVector* vec);

// Statistical functions
double sigma_mean(double* data, uint32_t n);
double sigma_std(double* data, uint32_t n);
double sigma_variance(double* data, uint32_t n);
double sigma_correlation(double* x, double* y, uint32_t n);
double sigma_covariance(double* x, double* y, uint32_t n);

// Activation functions (for neural networks)
double sigma_sigmoid(double x);
double sigma_relu(double x);
double sigma_tanh(double x);
double sigma_softmax(double* x, uint32_t n, uint32_t i);
double sigma_leaky_relu(double x, double alpha);
double sigma_elu(double x, double alpha);

// Loss functions
double sigma_mse_loss(double* y_true, double* y_pred, uint32_t n);
double sigma_cross_entropy_loss(double* y_true, double* y_pred, uint32_t n);
double sigma_hinge_loss(double* y_true, double* y_pred, uint32_t n);
double sigma_huber_loss(double* y_true, double* y_pred, uint32_t n, double delta);

// Optimization
void sigma_gradient_descent(SigmaVector* weights, SigmaVector* gradient, double lr);
void sigma_adam_update(SigmaVector* weights, SigmaVector* m, SigmaVector* v, 
                       SigmaVector* gradient, double lr, double beta1, double beta2, 
                       double epsilon, uint32_t t);

#endif // SIGMA_ML_ALGORITHMS_H
