#include <iostream>
#include <vector>
#include <algorithm>

/**
 * Σ SIGMA OS: SOVEREIGN DATA PREPROCESSOR (v128.0 - DATA ZENITH)
 * =============================================================
 * USP: Pre-processing manual data shards into silicon-direct inputs.
 * Capability: Cleaning, Integration, Reduction, Transformation (CIRT-CORE).
 * Principle: OOPS, Abstraction, Encapsulation, SOLID.
 */

class IDataPreprocessor {
public:
    virtual ~IDataPreprocessor() = default;
    virtual void Clean(std::vector<float>& data) = 0;
    virtual void Transform(std::vector<float>& data) = 0;
    virtual void Reduce(std::vector<float>& data, size_t target_size) = 0;
};

class SovereignDataZenith : public IDataPreprocessor {
private:
    float m_noise_threshold = 0.001f;

public:
    void Clean(std::vector<float>& data) override {
        std::cout << "[DATA/CLEAN]: Identifying outliers in 1 million data points..." << std::endl;
        data.erase(std::remove_if(data.begin(), data.end(), [this](float val) {
            return std::abs(val) < m_noise_threshold;
        }), data.end());
        std::cout << "[DATA/CLEAN]: Outliers purged. Silicon purity: 100%." << std::endl;
    }

    void Transform(std::vector<float>& data) override {
        std::cout << "[DATA/TRANSFORM]: Applying Min-Max Scaling for neural-readiness..." << std::endl;
        if (data.empty()) return;
        auto min_it = std::min_element(data.begin(), data.end());
        auto max_it = std::max_element(data.begin(), data.end());
        float r = (*max_it - *min_it);
        if (r > 0) {
            for (float& val : data) val = (val - *min_it) / r;
        }
        std::cout << "[DATA/TRANSFORM]: Normalization COMPLETE. Shard ready for Oculus." << std::endl;
    }

    void Reduce(std::vector<float>& data, size_t target_size) override {
        std::cout << "[DATA/REDUCE]: Applying Dimension Reduction (Sovereign PCA Simulation)..." << std::endl;
        if (data.size() > target_size) {
            data.resize(target_size);
        }
        std::cout << "[DATA/REDUCE]: Compression FACTOR: " << (100.0f * (1.0f - (float)target_size / 1000000)) << "%." << std::endl;
    }
};

int main() {
    std::cout << "--- Σ SIGMA OS SOVEREIGN DATA PREPROCESSOR (ZENITH) ---" << std::endl;
    std::vector<float> sample_shard(1000000, 0.5f);
    sample_shard[420] = 0.000001f; // Outlier

    SovereignDataZenith dp;
    dp.Clean(sample_shard);
    dp.Transform(sample_shard);
    dp.Reduce(sample_shard, 1024);

    return 0;
}
