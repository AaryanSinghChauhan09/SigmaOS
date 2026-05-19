/**
 * SigmaAI.cpp — SigmaAI Intelligence Layer
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-AIML + Syllabus-AdvPython (ML, DL, NLP)
 * Implements: ML Engine, Neural Network runtime, NLP pipeline, Legal AI
 */
#include "SigmaAI.h"

namespace Sigma::AI {

// ─── ML Engine ────────────────────────────────────────────────────────────────

// Data Preprocessing Pipeline
void MLEngine::preprocess(DataSet& ds, PreprocOptions opts) {
    // 1. Handle missing values
    if (opts.fill_missing == FillStrategy::MEAN) {
        for (sigma_u32 f = 0; f < ds.n_features; f++) {
            double sum = 0; sigma_u32 cnt = 0;
            for (sigma_u32 r = 0; r < ds.n_samples; r++)
                if (!ds.is_missing(r, f)) { sum += ds.X[r][f]; cnt++; }
            double mean = cnt ? sum / cnt : 0;
            for (sigma_u32 r = 0; r < ds.n_samples; r++)
                if (ds.is_missing(r, f)) ds.X[r][f] = mean;
        }
    }
    // 2. Standardize (z-score): x = (x - mean) / std
    if (opts.standardize) {
        for (sigma_u32 f = 0; f < ds.n_features; f++) {
            double sum = 0;
            for (sigma_u32 r = 0; r < ds.n_samples; r++) sum += ds.X[r][f];
            double mean = sum / ds.n_samples;
            double var = 0;
            for (sigma_u32 r = 0; r < ds.n_samples; r++) {
                double d = ds.X[r][f] - mean; var += d * d;
            }
            double std = sigma_sqrt(var / ds.n_samples);
            if (std < 1e-9) std = 1.0;
            for (sigma_u32 r = 0; r < ds.n_samples; r++)
                ds.X[r][f] = (ds.X[r][f] - mean) / std;
        }
    }
    // 3. Min-max normalization: x = (x - min) / (max - min)
    if (opts.normalize_minmax) {
        for (sigma_u32 f = 0; f < ds.n_features; f++) {
            double mn = ds.X[0][f], mx = ds.X[0][f];
            for (sigma_u32 r = 1; r < ds.n_samples; r++) {
                if (ds.X[r][f] < mn) mn = ds.X[r][f];
                if (ds.X[r][f] > mx) mx = ds.X[r][f];
            }
            double rng = mx - mn + 1e-9;
            for (sigma_u32 r = 0; r < ds.n_samples; r++)
                ds.X[r][f] = (ds.X[r][f] - mn) / rng;
        }
    }
}

// K-Means Clustering
ClusterResult MLEngine::kmeans(const DataSet& ds, int k, int max_iter) {
    ClusterResult result;
    result.n_clusters = k;
    result.labels = new int[ds.n_samples]();

    // Initialize centroids (first k samples)
    double** centroids = new double*[k];
    for (int c = 0; c < k; c++) {
        centroids[c] = new double[ds.n_features];
        for (sigma_u32 f = 0; f < ds.n_features; f++)
            centroids[c][f] = ds.X[c][f];
    }

    for (int iter = 0; iter < max_iter; iter++) {
        // Assign each sample to nearest centroid
        for (sigma_u32 r = 0; r < ds.n_samples; r++) {
            double best_dist = 1e18; int best_c = 0;
            for (int c = 0; c < k; c++) {
                double dist = 0;
                for (sigma_u32 f = 0; f < ds.n_features; f++) {
                    double d = ds.X[r][f] - centroids[c][f];
                    dist += d * d;
                }
                if (dist < best_dist) { best_dist = dist; best_c = c; }
            }
            result.labels[r] = best_c;
        }
        // Recompute centroids
        for (int c = 0; c < k; c++) {
            sigma_u32 count = 0;
            for (sigma_u32 f = 0; f < ds.n_features; f++) centroids[c][f] = 0;
            for (sigma_u32 r = 0; r < ds.n_samples; r++)
                if (result.labels[r] == c) {
                    for (sigma_u32 f = 0; f < ds.n_features; f++)
                        centroids[c][f] += ds.X[r][f];
                    count++;
                }
            if (count) for (sigma_u32 f = 0; f < ds.n_features; f++)
                centroids[c][f] /= count;
        }
    }
    return result;
}

// Naive Bayes Classifier
void NaiveBayes::fit(const DataSet& ds) {
    // Compute class priors and conditional probabilities
    n_classes_ = 0;
    for (sigma_u32 r = 0; r < ds.n_samples; r++)
        if ((int)ds.y[r] + 1 > n_classes_) n_classes_ = (int)ds.y[r] + 1;

    for (int c = 0; c < n_classes_; c++) {
        sigma_u32 cnt = 0;
        for (sigma_u32 r = 0; r < ds.n_samples; r++) if ((int)ds.y[r] == c) cnt++;
        priors_[c] = (double)cnt / ds.n_samples;
        // Compute mean and variance per feature per class
        for (sigma_u32 f = 0; f < ds.n_features; f++) {
            double sum = 0;
            sigma_u32 n = 0;
            for (sigma_u32 r = 0; r < ds.n_samples; r++)
                if ((int)ds.y[r] == c) { sum += ds.X[r][f]; n++; }
            means_[c][f] = n ? sum / n : 0;
            double var = 0;
            for (sigma_u32 r = 0; r < ds.n_samples; r++)
                if ((int)ds.y[r] == c) { double d = ds.X[r][f] - means_[c][f]; var += d*d; }
            variances_[c][f] = n > 1 ? var / (n-1) : 1e-9;
        }
    }
}

int NaiveBayes::predict(const double* x, sigma_u32 n_features) {
    double best_log_prob = -1e18; int best_class = 0;
    for (int c = 0; c < n_classes_; c++) {
        double log_prob = log_prior(c);
        for (sigma_u32 f = 0; f < n_features; f++) {
            // Gaussian log-likelihood
            double mu = means_[c][f], var = variances_[c][f];
            log_prob -= 0.5 * (x[f] - mu) * (x[f] - mu) / var;
        }
        if (log_prob > best_log_prob) { best_log_prob = log_prob; best_class = c; }
    }
    return best_class;
}

// ─── Neural Network (MLP) ─────────────────────────────────────────────────────

void MLPNetwork::add_layer(sigma_u32 units, Activation act) {
    Layer layer;
    layer.units = units;
    layer.activation = act;
    // Initialize weights with Xavier initialization
    sigma_u32 prev_units = m_layers.empty() ? m_input_dim : m_layers.back().units;
    layer.weights = new float[prev_units * units];
    layer.biases  = new float[units]();
    float scale = sigma_sqrt_f(2.0f / (float)(prev_units + units));
    for (sigma_u32 i = 0; i < prev_units * units; i++)
        layer.weights[i] = (random_float() * 2.0f - 1.0f) * scale;
    m_layers.push(layer);
}

void MLPNetwork::forward(const float* input) {
    const float* current = input;
    for (sigma_u32 l = 0; l < m_layers.size(); l++) {
        Layer& layer = m_layers[l];
        sigma_u32 in_dim = (l == 0) ? m_input_dim : m_layers[l-1].units;
        // Z = W * x + b
        for (sigma_u32 j = 0; j < layer.units; j++) {
            float z = layer.biases[j];
            for (sigma_u32 i = 0; i < in_dim; i++)
                z += layer.weights[i * layer.units + j] * current[i];
            // Activation
            switch (layer.activation) {
                case Activation::RELU:    layer.outputs[j] = z > 0.0f ? z : 0.0f; break;
                case Activation::SIGMOID: layer.outputs[j] = 1.0f/(1.0f + sigma_exp_f(-z)); break;
                case Activation::TANH:    layer.outputs[j] = sigma_tanh_f(z); break;
                case Activation::SOFTMAX: layer.outputs[j] = z; break; // computed after
                default: layer.outputs[j] = z; break;
            }
        }
        // Softmax normalization
        if (layer.activation == Activation::SOFTMAX) {
            float sum_exp = 0.0f;
            for (sigma_u32 j = 0; j < layer.units; j++)
                sum_exp += sigma_exp_f(layer.outputs[j]);
            for (sigma_u32 j = 0; j < layer.units; j++)
                layer.outputs[j] = sigma_exp_f(layer.outputs[j]) / (sum_exp + 1e-9f);
        }
        current = layer.outputs;
    }
}

// ─── NLP Pipeline ─────────────────────────────────────────────────────────────

TokenList SigmaNLP::tokenize(const char* text) {
    TokenList tokens;
    const char* p = text;
    while (*p) {
        // Skip whitespace
        while (*p == ' ' || *p == '\n' || *p == '\t') p++;
        if (!*p) break;
        // Read word
        const char* start = p;
        while (*p && *p != ' ' && *p != '\n' && *p != '\t' &&
               *p != '.' && *p != ',' && *p != '!' && *p != '?') p++;
        if (p > start) {
            Token t;
            t.text = intern_string(start, (sigma_usize)(p - start));
            t.is_stopword = is_stopword(t.text);
            tokens.push(t);
        }
        if (*p && (*p=='.'||*p==','||*p=='!'||*p=='?')) {
            Token punct;
            punct.text = intern_char(*p);
            punct.is_punctuation = true;
            tokens.push(punct);
            p++;
        }
    }
    return tokens;
}

// Named Entity Recognition (rule-based for legal text)
EntityList SigmaNLP::ner(const char* text) {
    EntityList entities;
    // Detect SECTION references: "Section 302 IPC"
    const char* p = text;
    while (*p) {
        if (sigma_strcmp(p, "Section ")) {
            const char* start = p;
            p += 8; // skip "Section "
            while (*p && (*p >= '0' && *p <= '9' || *p >= 'A' && *p <= 'Z')) p++;
            Entity e;
            e.type = "SECTION";
            e.text = intern_string(start, (sigma_usize)(p - start));
            entities.push(e);
        } else p++;
    }
    return entities;
}

// Extractive summarization: pick top-k sentences by TF-IDF score
char* SigmaNLP::summarize(const char* text, float ratio) {
    SentenceList sentences = split_sentences(text);
    TFIDFScorer scorer;
    scorer.fit(sentences);
    sigma_u32 keep = (sigma_u32)(sentences.size() * ratio);
    if (keep < 1) keep = 1;
    return scorer.top_k_sentences(sentences, keep);
}

} // namespace Sigma::AI
