/*
 * SigmaOS ML Projects & Practical Implementations
 * ================================================
 * Complete implementations of 22 practical ML projects:
 * - Data analysis projects
 * - Classification projects
 * - Regression projects
 * - Deep learning projects
 * - NLP projects
 * - Computer vision projects
 * - MLOps projects
 * - Advanced systems
 */

#ifndef SIGMA_ML_PROJECTS_H
#define SIGMA_ML_PROJECTS_H

#include "sigma_ml_algorithms.h"
#include "sigma_neural_networks.h"
#include "sigma_mlops.h"
#include "sigma_cs_algorithms.h"

// ==================== PROJECT 1: EDA PORTFOLIO ====================

typedef struct {
    char dataset_path[1024];
    char output_dir[1024];
    
    // Dataset info
    uint32_t n_rows;
    uint32_t n_columns;
    char** column_names;
    char** data_types;
    
    // Statistics
    SigmaMatrix* summary_stats; // mean, median, std, min, max, quartiles
    SigmaMatrix* correlation_matrix;
    SigmaMatrix* missing_values;
    
    // Visualizations generated
    char histogram_paths[50][1024];
    char boxplot_paths[50][1024];
    char scatterplot_paths[50][1024];
    char heatmap_path[1024];
    char pairplot_path[1024];
    char distribution_path[1024];
    
    // Insights
    char insights[20][2048];
    uint32_t n_insights;
    
    // Report
    char report_path[1024];
    bool report_generated;
} SigmaEDAProject;

SigmaEDAProject* sigma_eda_create(const char* dataset_path, const char* output_dir);
void sigma_eda_load_data(SigmaEDAProject* eda);
void sigma_eda_generate_summary(SigmaEDAProject* eda);
void sigma_eda_detect_missing_values(SigmaEDAProject* eda);
void sigma_eda_correlation_analysis(SigmaEDAProject* eda);
void sigma_eda_generate_histograms(SigmaEDAProject* eda);
void sigma_eda_generate_boxplots(SigmaEDAProject* eda);
void sigma_eda_generate_scatterplots(SigmaEDAProject* eda);
void sigma_eda_generate_heatmap(SigmaEDAProject* eda);
void sigma_eda_generate_pairplot(SigmaEDAProject* eda);
void sigma_eda_detect_outliers(SigmaEDAProject* eda);
void sigma_eda_feature_analysis(SigmaEDAProject* eda);
void sigma_eda_generate_insights(SigmaEDAProject* eda);
void sigma_eda_create_report(SigmaEDAProject* eda);
void sigma_eda_run_all(SigmaEDAProject* eda);
void sigma_eda_destroy(SigmaEDAProject* eda);

// Command: sigma_ml_project eda --dataset=data.csv --output=eda_report/

// ==================== PROJECT 2: IRIS DATASET CLASSIFICATION ====================

typedef struct {
    // Dataset
    SigmaDataset* train_data;
    SigmaDataset* test_data;
    
    // Features
    char* feature_names[4];
    char* target_names[3];
    
    // Models
    SigmaKNN* knn_model;
    SigmaDecisionTree* dt_model;
    SigmaSVM* svm_model;
    SigmaNaiveBayes* nb_model;
    SigmaRandomForest* rf_model;
    SigmaLogisticRegression* lr_model;
    
    // Results
    double knn_accuracy;
    double dt_accuracy;
    double svm_accuracy;
    double nb_accuracy;
    double rf_accuracy;
    double lr_accuracy;
    
    // Best model
    char best_model_name[64];
    double best_accuracy;
    void* best_model;
    
    // Visualization
    char confusion_matrix_path[1024];
    char decision_boundary_path[1024];
    char pairplot_path[1024];
} SigmaIrisProject;

SigmaIrisProject* sigma_iris_create(void);
void sigma_iris_load_dataset(SigmaIrisProject* iris);
void sigma_iris_split_data(SigmaIrisProject* iris, double test_size);
void sigma_iris_train_knn(SigmaIrisProject* iris, uint32_t k);
void sigma_iris_train_decision_tree(SigmaIrisProject* iris);
void sigma_iris_train_svm(SigmaIrisProject* iris);
void sigma_iris_train_naive_bayes(SigmaIrisProject* iris);
void sigma_iris_train_random_forest(SigmaIrisProject* iris);
void sigma_iris_train_logistic_regression(SigmaIrisProject* iris);
void sigma_iris_compare_models(SigmaIrisProject* iris);
void sigma_iris_visualize_results(SigmaIrisProject* iris);
void sigma_iris_predict_sample(SigmaIrisProject* iris, double sepal_length, 
                               double sepal_width, double petal_length, 
                               double petal_width, char* predicted_class);
void sigma_iris_run_all(SigmaIrisProject* iris);
void sigma_iris_destroy(SigmaIrisProject* iris);

// Command: sigma_ml_project iris --models=all --visualize=true

// ==================== PROJECT 3: BUILD YOUR OWN LINEAR REGRESSION ====================

typedef struct {
    // Implementation from scratch
    double* weights;
    double bias;
    uint32_t n_features;
    
    // Training data
    SigmaMatrix* X_train;
    SigmaVector* y_train;
    SigmaMatrix* X_test;
    SigmaVector* y_test;
    
    // Hyperparameters
    double learning_rate;
    uint32_t n_iterations;
    
    // Training history
    double* loss_history;
    uint32_t history_size;
    
    // Metrics
    double mse;
    double rmse;
    double mae;
    double r2_score;
    
    // Dataset (can be any: synthetic, Boston, California, etc.)
    char dataset_name[64];
    bool use_builtin_dataset;
} SigmaCustomLinearRegression;

SigmaCustomLinearRegression* sigma_custom_lr_create(uint32_t n_features);
void sigma_custom_lr_initialize_weights(SigmaCustomLinearRegression* model);
void sigma_custom_lr_load_dataset(SigmaCustomLinearRegression* model, 
                                 const char* dataset_name);
double sigma_custom_lr_hypothesis(SigmaCustomLinearRegression* model, 
                                   double* x);
void sigma_custom_lr_compute_gradients(SigmaCustomLinearRegression* model,
                                       double* gradients);
void sigma_custom_lr_update_weights(SigmaCustomLinearRegression* model,
                                     double* gradients);
void sigma_custom_lr_train(SigmaCustomLinearRegression* model,
                          SigmaMatrix* X, SigmaVector* y);
void sigma_custom_lr_predict(SigmaCustomLinearRegression* model,
                             SigmaMatrix* X, SigmaVector* predictions);
void sigma_custom_lr_evaluate(SigmaCustomLinearRegression* model);
void sigma_custom_lr_plot_loss(SigmaCustomLinearRegression* model,
                               const char* output_path);
void sigma_custom_lr_compare_with_sklearn(SigmaCustomLinearRegression* model);
void sigma_custom_lr_run_demo(SigmaCustomLinearRegression* model);
void sigma_custom_lr_destroy(SigmaCustomLinearRegression* model);

// Command: sigma_ml_project linear_regression_from_scratch --dataset=boston --lr=0.01 --iterations=1000

// ==================== PROJECT 4: TITANIC SURVIVAL PREDICTION ====================

typedef struct {
    // Raw data
    SigmaMatrix* raw_train;
    SigmaMatrix* raw_test;
    
    // Processed data
    SigmaDataset* train_data;
    SigmaDataset* test_data;
    
    // Feature engineering
    bool feature_engineered;
    char engineered_features[20][64];
    uint32_t n_engineered_features;
    
    // Missing value handling
    double age_mean;
    double fare_mean;
    char embarked_mode[16];
    
    // Encoded categorical variables
    SigmaHashTable* sex_encoding;
    SigmaHashTable* embarked_encoding;
    SigmaHashTable* pclass_encoding;
    
    // Models
    SigmaLogisticRegression* lr_model;
    SigmaRandomForest* rf_model;
    SigmaGradientBoosting* gb_model;
    SigmaXGBoost* xgb_model;
    
    // Model performance
    double accuracy;
    double precision;
    double recall;
    double f1_score;
    double auc_roc;
    
    // Predictions
    SigmaVector* test_predictions;
    char submission_path[1024];
    
    // Feature importance
    char feature_importance_plot[1024];
} SigmaTitanicProject;

SigmaTitanicProject* sigma_titanic_create(const char* train_path, 
                                           const char* test_path);
void sigma_titanic_explore_data(SigmaTitanicProject* titanic);
void sigma_titanic_handle_missing_values(SigmaTitanicProject* titanic);
void sigma_titanic_feature_engineering(SigmaTitanicProject* titanic);
void sigma_titanic_encode_categorical(SigmaTitanicProject* titanic);
void sigma_titanic_normalize_features(SigmaTitanicProject* titanic);
void sigma_titanic_split_data(SigmaTitanicProject* titanic);
void sigma_titanic_train_models(SigmaTitanicProject* titanic);
void sigma_titanic_evaluate_models(SigmaTitanicProject* titanic);
void sigma_titanic_select_best_model(SigmaTitanicProject* titanic);
void sigma_titanic_generate_predictions(SigmaTitanicProject* titanic);
void sigma_titanic_create_submission(SigmaTitanicProject* titanic);
void sigma_titanic_plot_feature_importance(SigmaTitanicProject* titanic);
void sigma_titanic_run_all(SigmaTitanicProject* titanic);
void sigma_titanic_destroy(SigmaTitanicProject* titanic);

// Command: sigma_ml_project titanic --train=train.csv --test=test.csv --submit=true

// ==================== PROJECT 5: HOUSING PRICE PREDICTOR ====================

typedef struct {
    // Dataset (supports: Boston, California, Ames, or custom)
    char dataset_name[64];
    SigmaDataset* train_data;
    SigmaDataset* test_data;
    
    // Features
    uint32_t n_features;
    char** feature_names;
    
    // Preprocessing
    bool log_transform_target;
    bool feature_scaled;
    bool outlier_removed;
    
    // Models
    SigmaLinearRegression* linear_model;
    SigmaLinearRegression* ridge_model;
    SigmaLinearRegression* lasso_model;
    SigmaRandomForest* rf_model;
    SigmaXGBoost* xgb_model;
    
    // Model performance (RMSE, MAE, R²)
    double linear_rmse, linear_mae, linear_r2;
    double ridge_rmse, ridge_mae, ridge_r2;
    double lasso_rmse, lasso_mae, lasso_r2;
    double rf_rmse, rf_mae, rf_r2;
    double xgb_rmse, xgb_mae, xgb_r2;
    
    // Best model
    char best_model[64];
    void* best_model_ptr;
    
    // Predictions
    SigmaVector* predictions;
    char prediction_plot_path[1024];
    char residual_plot_path[1024];
    
    // Feature importance
    char feature_importance_path[1024];
} SigmaHousingProject;

SigmaHousingProject* sigma_housing_create(const char* dataset_name);
void sigma_housing_load_data(SigmaHousingProject* housing);
void sigma_housing_explore_data(SigmaHousingProject* housing);
void sigma_housing_handle_outliers(SigmaHousingProject* housing);
void sigma_housing_feature_engineering(SigmaHousingProject* housing);
void sigma_housing_preprocess(SigmaHousingProject* housing);
void sigma_housing_train_linear(SigmaHousingProject* housing);
void sigma_housing_train_ridge(SigmaHousingProject* housing, double alpha);
void sigma_housing_train_lasso(SigmaHousingProject* housing, double alpha);
void sigma_housing_train_rf(SigmaHousingProject* housing, uint32_t n_estimators);
void sigma_housing_train_xgb(SigmaHousingProject* housing);
void sigma_housing_compare_models(SigmaHousingProject* housing);
void sigma_housing_predict(SigmaHousingProject* housing, SigmaMatrix* X_test);
void sigma_housing_plot_predictions(SigmaHousingProject* housing);
void sigma_housing_plot_residuals(SigmaHousingProject* housing);
void sigma_housing_plot_feature_importance(SigmaHousingProject* housing);
void sigma_housing_run_all(SigmaHousingProject* housing);
void sigma_housing_destroy(SigmaHousingProject* housing);

// Command: sigma_ml_project housing --dataset=boston --models=all

// ==================== PROJECT 6: IMAGE CLASSIFICATION SYSTEM ====================

typedef struct {
    // Dataset (MNIST, CIFAR-10, Fashion-MNIST, or custom)
    char dataset_name[64];
    uint32_t image_height;
    uint32_t image_width;
    uint32_t n_channels;
    uint32_t n_classes;
    
    // Data
    SigmaMatrix* X_train;
    SigmaVector* y_train;
    SigmaMatrix* X_test;
    SigmaVector* y_test;
    
    // Normalization
    double mean;
    double std;
    
    // Models
    SigmaCNN* cnn_model;
    SigmaMLP* mlp_model;
    
    // Training config
    uint32_t batch_size;
    uint32_t epochs;
    char optimizer[16];
    double learning_rate;
    
    // Data augmentation
    bool use_augmentation;
    bool flip_horizontal;
    bool random_rotation;
    bool random_zoom;
    
    // Results
    double train_accuracy;
    double test_accuracy;
    double test_loss;
    
    // Visualizations
    char training_history_plot[1024];
    char confusion_matrix_plot[1024];
    char sample_predictions_plot[1024];
    
    // Prediction
    char class_names[100][64];
} SigmaImageClassification;

SigmaImageClassification* sigma_img_class_create(const char* dataset_name);
void sigma_img_class_load_dataset(SigmaImageClassification* img);
void sigma_img_class_preprocess(SigmaImageClassification* img);
void sigma_img_class_setup_augmentation(SigmaImageClassification* img,
                                         bool flip, bool rotate, bool zoom);
void sigma_img_class_build_cnn(SigmaImageClassification* img,
                                uint32_t* filters, uint32_t n_conv_layers);
void sigma_img_class_build_mlp(SigmaImageClassification* img,
                               uint32_t* hidden_units, uint32_t n_hidden);
void sigma_img_class_train(SigmaImageClassification* img);
void sigma_img_class_evaluate(SigmaImageClassification* img);
void sigma_img_class_plot_history(SigmaImageClassification* img);
void sigma_img_class_plot_confusion_matrix(SigmaImageClassification* img);
void sigma_img_class_plot_predictions(SigmaImageClassification* img);
int sigma_img_class_predict_image(SigmaImageClassification* img,
                                   double* image_data);
void sigma_img_class_save_model(SigmaImageClassification* img, 
                                const char* path);
void sigma_img_class_run_all(SigmaImageClassification* img);
void sigma_img_class_destroy(SigmaImageClassification* img);

// Command: sigma_ml_project image_classification --dataset=mnist --model=cnn --epochs=10

// ==================== PROJECT 7: SENTIMENT ANALYSIS SYSTEM ====================

typedef struct {
    // Dataset (IMDB, Twitter, Yelp, or custom)
    char dataset_name[64];
    char dataset_path[1024];
    
    // Text preprocessing
    bool lowercase;
    bool remove_stopwords;
    bool stemming;
    bool lemmatization;
    uint32_t max_features;
    uint32_t max_sequence_length;
    
    // Vocabulary
    SigmaHashTable* word_to_index;
    char** index_to_word;
    uint32_t vocab_size;
    
    // Vectorization (TF-IDF or Word Embeddings)
    char vectorizer_type[16]; // "tfidf", "count", "word2vec", "glove"
    SigmaMatrix* tfidf_matrix;
    
    // Data
    char** train_texts;
    SigmaVector* train_labels;
    char** test_texts;
    SigmaVector* test_labels;
    uint32_t n_train;
    uint32_t n_test;
    
    // Models
    SigmaNaiveBayes* nb_model;
    SigmaLogisticRegression* lr_model;
    SigmaMLP* mlp_model;
    SigmaRNN* lstm_model;
    
    // Results
    double accuracy;
    double precision;
    double recall;
    double f1_score;
    
    // Analysis
    char positive_words[100][64];
    char negative_words[100][64];
    uint32_t n_positive_words;
    uint32_t n_negative_words;
} SigmaSentimentAnalysis;

SigmaSentimentAnalysis* sigma_sentiment_create(const char* dataset_name);
void sigma_sentiment_load_data(SigmaSentimentAnalysis* sa);
void sigma_sentiment_preprocess_text(SigmaSentimentAnalysis* sa);
void sigma_sentiment_build_vocabulary(SigmaSentimentAnalysis* sa);
void sigma_sentiment_vectorize_tfidf(SigmaSentimentAnalysis* sa);
void sigma_sentiment_vectorize_embeddings(SigmaSentimentAnalysis* sa,
                                          const char* embedding_type);
void sigma_sentiment_train_naive_bayes(SigmaSentimentAnalysis* sa);
void sigma_sentiment_train_logistic_regression(SigmaSentimentAnalysis* sa);
void sigma_sentiment_train_mlp(SigmaSentimentAnalysis* sa);
void sigma_sentiment_train_lstm(SigmaSentimentAnalysis* sa);
void sigma_sentiment_evaluate(SigmaSentimentAnalysis* sa);
void sigma_sentiment_extract_important_words(SigmaSentimentAnalysis* sa);
double sigma_sentiment_predict_text(SigmaSentimentAnalysis* sa, 
                                     const char* text);
void sigma_sentiment_analyze_batch(SigmaSentimentAnalysis* sa,
                                    char** texts, uint32_t n_texts,
                                    double* predictions);
void sigma_sentiment_create_wordcloud(SigmaSentimentAnalysis* sa);
void sigma_sentiment_run_all(SigmaSentimentAnalysis* sa);
void sigma_sentiment_destroy(SigmaSentimentAnalysis* sa);

// Command: sigma_ml_project sentiment --dataset=imdb --model=lstm --max_features=10000

// Continue with more projects...
