// ==================== PROJECT 8: CUSTOMER CHURN PREDICTOR ====================

typedef struct {
    // Dataset
    char dataset_path[1024];
    SigmaDataset* train_data;
    SigmaDataset* test_data;
    
    // Features
    uint32_t n_features;
    char** feature_names;
    
    // Preprocessing
    bool categorical_encoded;
    bool features_scaled;
    bool balanced;
    
    // Class imbalance handling
    char balancing_method[32]; // "smote", "adasyn", "random_oversample", "none"
    double class_imbalance_ratio;
    
    // Models
    SigmaLogisticRegression* lr_model;
    SigmaRandomForest* rf_model;
    SigmaGradientBoosting* gb_model;
    SigmaXGBoost* xgb_model;
    
    // Metrics (focus on recall for churn - we don't want to miss churners)
    double accuracy;
    double precision;
    double recall;
    double f1_score;
    double auc_roc;
    double auc_pr; // Precision-Recall AUC (important for imbalanced data)
    
    // Confusion matrix
    uint32_t true_negatives;
    uint32_t false_positives;
    uint32_t false_negatives;
    uint32_t true_positives;
    
    // Feature importance
    char** important_features;
    double* feature_importance_scores;
    uint32_t n_important_features;
    
    // Retention strategies
    char high_risk_customers[1000][256];
    uint32_t n_high_risk;
    double potential_revenue_at_risk;
} SigmaChurnPredictor;

SigmaChurnPredictor* sigma_churn_create(const char* dataset_path);
void sigma_churn_load_data(SigmaChurnPredictor* churn);
void sigma_churn_explore_data(SigmaChurnPredictor* churn);
void sigma_churn_preprocess(SigmaChurnPredictor* churn);
void sigma_churn_handle_imbalance(SigmaChurnPredictor* churn, const char* method);
void sigma_churn_train_lr(SigmaChurnPredictor* churn);
void sigma_churn_train_rf(SigmaChurnPredictor* churn);
void sigma_churn_train_gb(SigmaChurnPredictor* churn);
void sigma_churn_train_xgb(SigmaChurnPredictor* churn);
void sigma_churn_evaluate(SigmaChurnPredictor* churn);
void sigma_churn_analyze_feature_importance(SigmaChurnPredictor* churn);
void sigma_churn_identify_high_risk(SigmaChurnPredictor* churn, double threshold);
void sigma_churn_propose_retention_strategies(SigmaChurnPredictor* churn);
void sigma_churn_calculate_revenue_impact(SigmaChurnPredictor* churn);
void sigma_churn_run_all(SigmaChurnPredictor* churn);
void sigma_churn_destroy(SigmaChurnPredictor* churn);

// Command: sigma_ml_project churn --dataset=customer_data.csv --balance=smote

// ==================== PROJECT 9: STOCK PRICE PREDICTOR ====================

typedef struct {
    // Stock data
    char stock_symbol[16];
    char data_source[64]; // "yahoo", "alpha_vantage", "csv"
    char start_date[16];
    char end_date[16];
    
    // Time series data
    SigmaVector* dates;
    SigmaVector* open_prices;
    SigmaVector* high_prices;
    SigmaVector* low_prices;
    SigmaVector* close_prices;
    SigmaVector* volume;
    
    // Technical indicators
    SigmaVector* sma_20; // Simple Moving Average
    SigmaVector* sma_50;
    SigmaVector* ema_12; // Exponential Moving Average
    SigmaVector* ema_26;
    SigmaVector* rsi;    // Relative Strength Index
    SigmaVector* macd;
    SigmaVector* bollinger_upper;
    SigmaVector* bollinger_lower;
    
    // Features for ML
    SigmaMatrix* features;
    SigmaVector* target; // Next day price or price change
    
    // Model types
    bool use_lstm;
    bool use_arima;
    bool use_linear;
    bool use_xgb;
    
    // Models
    SigmaLinearRegression* linear_model;
    SigmaRNN* lstm_model;
    SigmaXGBoost* xgb_model;
    
    // ARIMA parameters (p, d, q)
    uint32_t arima_p;
    uint32_t arima_d;
    uint32_t arima_q;
    
    // Forecast horizon
    uint32_t forecast_days;
    SigmaVector* predictions;
    
    // Metrics
    double rmse;
    double mae;
    double mape;
    double directional_accuracy; // % of correct up/down predictions
    
    // Visualization
    char price_chart_path[1024];
    char prediction_chart_path[1024];
    char technical_indicators_path[1024];
} SigmaStockPredictor;

SigmaStockPredictor* sigma_stock_create(const char* symbol, 
                                        const char* start_date,
                                        const char* end_date);
void sigma_stock_fetch_data(SigmaStockPredictor* stock);
void sigma_stock_calculate_indicators(SigmaStockPredictor* stock);
void sigma_stock_create_features(SigmaStockPredictor* stock);
void sigma_stock_prepare_sequences(SigmaStockPredictor* stock, uint32_t lookback);
void sigma_stock_split_train_test(SigmaStockPredictor* stock, double test_ratio);
void sigma_stock_train_linear(SigmaStockPredictor* stock);
void sigma_stock_train_lstm(SigmaStockPredictor* stock, uint32_t lookback);
void sigma_stock_train_xgb(SigmaStockPredictor* stock);
void sigma_stock_forecast(SigmaStockPredictor* stock, uint32_t days);
void sigma_stock_evaluate(SigmaStockPredictor* stock);
void sigma_stock_plot_price_history(SigmaStockPredictor* stock);
void sigma_stock_plot_predictions(SigmaStockPredictor* stock);
void sigma_stock_plot_technical_indicators(SigmaStockPredictor* stock);
void sigma_stock_generate_signals(SigmaStockPredictor* stock); // Buy/Sell signals
void sigma_stock_run_all(SigmaStockPredictor* stock);
void sigma_stock_destroy(SigmaStockPredictor* stock);

// Command: sigma_ml_project stock --symbol=AAPL --start=2020-01-01 --model=lstm --forecast=30

// ==================== PROJECT 10: BUILD YOUR OWN NEURAL NETWORK ====================

typedef struct {
    // Network architecture (user-defined)
    uint32_t input_size;
    uint32_t n_hidden_layers;
    uint32_t* hidden_layer_sizes;
    uint32_t output_size;
    char activation_function[16]; // "relu", "sigmoid", "tanh"
    char output_activation[16];  // "softmax", "sigmoid", "linear"
    
    // Weights and biases (all stored here for learning)
    SigmaMatrix** weights; // Array of weight matrices between layers
    SigmaVector** biases;  // Array of bias vectors
    
    // Forward pass cache
    SigmaMatrix** layer_outputs;  // Outputs of each layer
    SigmaMatrix** layer_inputs;     // Pre-activation values
    
    // Backward pass cache
    SigmaMatrix** weight_gradients;
    SigmaVector** bias_gradients;
    SigmaMatrix** delta; // Error terms for each layer
    
    // Training parameters
    double learning_rate;
    uint32_t epochs;
    uint32_t batch_size;
    char optimizer[16]; // "sgd", "momentum", "adam", "rmsprop"
    double momentum;
    double beta1; // For Adam
    double beta2;
    double epsilon;
    
    // Optimizer state
    SigmaMatrix** m_weights; // First moment (Adam)
    SigmaMatrix** v_weights; // Second moment (Adam)
    SigmaVector** m_biases;
    SigmaVector** v_biases;
    uint32_t timestep;
    
    // Loss history
    double* training_loss;
    double* validation_loss;
    uint32_t history_length;
    
    // Regularization
    double l2_lambda;
    double dropout_rate;
    
    // Dataset
    SigmaMatrix* X_train;
    SigmaVector* y_train;
    SigmaMatrix* X_val;
    SigmaVector* y_val;
} SigmaCustomNeuralNetwork;

SigmaCustomNeuralNetwork* sigma_custom_nn_create(uint32_t input_size,
                                                  uint32_t* hidden_sizes,
                                                  uint32_t n_hidden,
                                                  uint32_t output_size);
void sigma_custom_nn_initialize_weights(SigmaCustomNeuralNetwork* nn,
                                        const char* method); // "xavier", "he", "random"
double sigma_custom_nn_activation(double x, const char* type);
double sigma_custom_nn_activation_derivative(double x, const char* type);
SigmaMatrix* sigma_custom_nn_forward(SigmaCustomNeuralNetwork* nn, 
                                      SigmaMatrix* X);
void sigma_custom_nn_compute_loss(SigmaCustomNeuralNetwork* nn,
                                   SigmaVector* y_true,
                                   SigmaMatrix* y_pred,
                                   double* loss);
void sigma_custom_nn_backward(SigmaCustomNeuralNetwork* nn,
                               SigmaVector* y_true,
                               SigmaMatrix* y_pred);
void sigma_custom_nn_update_weights_sgd(SigmaCustomNeuralNetwork* nn);
void sigma_custom_nn_update_weights_momentum(SigmaCustomNeuralNetwork* nn);
void sigma_custom_nn_update_weights_adam(SigmaCustomNeuralNetwork* nn);
void sigma_custom_nn_train_epoch(SigmaCustomNeuralNetwork* nn);
void sigma_custom_nn_train(SigmaCustomNeuralNetwork* nn,
                            SigmaMatrix* X, SigmaVector* y,
                            SigmaMatrix* X_val, SigmaVector* y_val,
                            uint32_t epochs);
SigmaMatrix* sigma_custom_nn_predict(SigmaCustomNeuralNetwork* nn,
                                       SigmaMatrix* X);
void sigma_custom_nn_evaluate(SigmaCustomNeuralNetwork* nn,
                               SigmaMatrix* X, SigmaVector* y);
void sigma_custom_nn_save(SigmaCustomNeuralNetwork* nn, const char* path);
void sigma_custom_nn_load(SigmaCustomNeuralNetwork* nn, const char* path);
void sigma_custom_nn_visualize_architecture(SigmaCustomNeuralNetwork* nn);
void sigma_custom_nn_plot_loss(SigmaCustomNeuralNetwork* nn);
void sigma_custom_nn_run_demo(SigmaCustomNeuralNetwork* nn);
void sigma_custom_nn_destroy(SigmaCustomNeuralNetwork* nn);

// Command: sigma_ml_project neural_network_from_scratch --layers="784,256,128,10" --activation=relu

// ==================== PROJECT 11: FACE RECOGNITION SYSTEM ====================

typedef struct {
    // Face detection
    char detector_model[64]; // "haarcascade", "mtcnn", "dlib"
    
    // Face embedding model
    char embedding_model[64]; // "facenet", "openface", "deepface"
    uint32_t embedding_dim;
    
    // Database
    char** person_names;
    SigmaMatrix* face_embeddings; // Each row is an embedding
    uint32_t n_persons;
    uint32_t* samples_per_person;
    
    // Recognition settings
    double similarity_threshold;
    char distance_metric[16]; // "euclidean", "cosine"
    
    // Live recognition
    bool use_webcam;
    uint32_t webcam_id;
    
    // Training mode
    bool training_mode;
    char new_person_name[256];
    uint32_t samples_to_collect;
    
    // Performance
    double recognition_accuracy;
    double false_accept_rate;
    double false_reject_rate;
    double inference_time_ms;
    
    // Output
    char recognized_person[256];
    double confidence;
} SigmaFaceRecognition;

SigmaFaceRecognition* sigma_face_recognition_create(const char* detector,
                                                     const char* embedder);
void sigma_face_recognition_load_database(SigmaFaceRecognition* fr,
                                           const char* database_path);
void sigma_face_recognition_add_person(SigmaFaceRecognition* fr,
                                        const char* name,
                                        char** image_paths,
                                        uint32_t n_images);
void sigma_face_recognition_remove_person(SigmaFaceRecognition* fr,
                                          const char* name);
void sigma_face_recognition_extract_embedding(SigmaFaceRecognition* fr,
                                              double* face_image,
                                              double* embedding_out);
double sigma_face_recognition_compute_distance(SigmaFaceRecognition* fr,
                                               double* embedding1,
                                               double* embedding2);
char* sigma_face_recognition_identify(SigmaFaceRecognition* fr,
                                       double* face_embedding,
                                       double* confidence_out);
void sigma_face_recognition_recognize_from_image(SigmaFaceRecognition* fr,
                                                  const char* image_path);
void sigma_face_recognition_start_webcam(SigmaFaceRecognition* fr);
void sigma_face_recognition_stop_webcam(SigmaFaceRecognition* fr);
void sigma_face_recognition_train_mode(SigmaFaceRecognition* fr,
                                        const char* person_name,
                                        uint32_t n_samples);
void sigma_face_recognition_evaluate_accuracy(SigmaFaceRecognition* fr,
                                               const char* test_dataset_path);
void sigma_face_recognition_save_database(SigmaFaceRecognition* fr,
                                          const char* path);
void sigma_face_recognition_run_demo(SigmaFaceRecognition* fr);
void sigma_face_recognition_destroy(SigmaFaceRecognition* fr);

// Command: sigma_ml_project face_recognition --database=faces.db --webcam=true

// ==================== PROJECT 12: RECOMMENDATION SYSTEM ====================

typedef struct {
    // Data
    char dataset_name[64]; // "movielens", "amazon", "netflix", or custom
    uint32_t n_users;
    uint32_t n_items;
    
    // User-item interaction matrix
    SigmaMatrix* user_item_matrix;
    
    // Collaborative filtering
    char cf_type[16]; // "user_based", "item_based", "matrix_factorization"
    uint32_t n_factors; // For matrix factorization
    double regularization;
    
    // Matrix factorization components
    SigmaMatrix* user_factors;
    SigmaMatrix* item_factors;
    SigmaVector* user_bias;
    SigmaVector* item_bias;
    double global_bias;
    
    // Content-based filtering
    SigmaMatrix* item_features;
    char item_feature_names[50][64];
    uint32_t n_item_features;
    
    // Hybrid weights
    double cf_weight;
    double cb_weight;
    
    // Similarity metrics
    char similarity_metric[16]; // "cosine", "pearson", "jaccard"
    
    // Evaluation
    double rmse;
    double mae;
    double precision_at_k;
    double recall_at_k;
    double ndcg_at_k;
    
    // Recommendations
    uint32_t** user_recommendations; // [user_id][recommended_items]
    double** recommendation_scores;
    uint32_t top_k;
} SigmaRecommendationSystem;

SigmaRecommendationSystem* sigma_recommendation_create(const char* dataset_name,
                                                       const char* algorithm);
void sigma_recommendation_load_data(SigmaRecommendationSystem* rec);
void sigma_recommendation_build_user_item_matrix(SigmaRecommendationSystem* rec);
void sigma_recommendation_train_user_based_cf(SigmaRecommendationSystem* rec,
                                               uint32_t k_neighbors);
void sigma_recommendation_train_item_based_cf(SigmaRecommendationSystem* rec,
                                               uint32_t k_neighbors);
void sigma_recommendation_train_matrix_factorization(SigmaRecommendationSystem* rec,
                                                      uint32_t n_factors,
                                                      double learning_rate,
                                                      uint32_t epochs);
void sigma_recommendation_train_content_based(SigmaRecommendationSystem* rec);
void sigma_recommendation_train_hybrid(SigmaRecommendationSystem* rec,
                                        double cf_weight,
                                        double cb_weight);
void sigma_recommendation_evaluate(SigmaRecommendationSystem* rec,
                                    uint32_t k);
uint32_t* sigma_recommendation_get_recommendations(SigmaRecommendationSystem* rec,
                                                    uint32_t user_id,
                                                    uint32_t n_recommendations,
                                                    double* scores);
void sigma_recommendation_get_similar_items(SigmaRecommendationSystem* rec,
                                           uint32_t item_id,
                                           uint32_t n_similar,
                                           uint32_t* similar_items,
                                           double* similarity_scores);
void sigma_recommendation_predict_rating(SigmaRecommendationSystem* rec,
                                         uint32_t user_id,
                                         uint32_t item_id,
                                         double* predicted_rating);
void sigma_recommendation_save_model(SigmaRecommendationSystem* rec,
                                     const char* path);
void sigma_recommendation_load_model(SigmaRecommendationSystem* rec,
                                     const char* path);
void sigma_recommendation_run_demo(SigmaRecommendationSystem* rec);
void sigma_recommendation_destroy(SigmaRecommendationSystem* rec);

// Command: sigma_ml_project recommendation --dataset=movielens --algorithm=matrix_factorization --factors=50

// Continue with remaining projects...
