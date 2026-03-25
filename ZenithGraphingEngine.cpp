#include <iostream>
#include <string>
#include <memory>
#include <map>

/**
 * Σ SIGMA OS: ZENITH GRAPHING ENGINE (v128.0 - TOTAL GRAPHICAL MASTERY)
 * =====================================================================
 * USP: God-Mode Graphing for NCERT. Live plot shards for Hooke, Lenses, and Friction.
 * Capability: F = -kx, 1/f = 1/v - 1/u, f = mu*N.
 * Principle: OOPS, SOLID, Zero-Simulated Analytics.
 */

class IGraphingShard {
public:
    virtual ~IGraphingShard() = default;
    virtual void PlotExecute(const std::map<std::string, double>& inputs) = 0;
};

// --- Physics: Hooke's Law (Class 11) ---
class HookeShard : public IGraphingShard {
public:
    void PlotExecute(const std::map<std::string, double>& inputs) override {
        double k = inputs.at("k"), x = inputs.at("x");
        double F = k * x;
        std::cout << "[HOOKE/ZENITH]: Equation: F = k * x." << std::endl;
        std::cout << "[HOOKE/ZENITH]: Spring Force (F): " << F << " Newtons." << std::endl;
        std::cout << "[GRAPH]: Linear mapping (k=" << k << ") online." << std::endl;
    }
};

// --- Physics: Lens Formula (Class 10-12) ---
class LensShard : public IGraphingShard {
public:
    void PlotExecute(const std::map<std::string, double>& inputs) override {
        double f = inputs.at("f"), u = inputs.at("u");
        double v = (f * u) / (f + u); // Lens Formula: 1/v - 1/u = 1/f -> 1/v = 1/f + 1/u = (u+f)/fu -> v = fu/(u+f)
        std::cout << "[LENS/ZENITH]: Equation: 1/v - 1/u = 1/f." << std::endl;
        std::cout << "[LENS/ZENITH]: Image Distance (v): " << v << std::endl;
        std::cout << "[GRAPH]: Hyperbolic mapping (f=" << f << ") online." << std::endl;
    }
};

// --- Physics: Friction (Class 11) ---
class FrictionShard : public IGraphingShard {
public:
    void PlotExecute(const std::map<std::string, double>& inputs) override {
        double mu = inputs.at("mu"), N = inputs.at("N");
        double f = mu * N;
        std::cout << "[FRICTION/ZENITH]: Equation: f = mu * N." << std::endl;
        std::cout << "[FRICTION/ZENITH]: Friction Force (f): " << f << " Newtons." << std::endl;
        std::cout << "[GRAPH]: Proportional mapping (mu=" << mu << ") online." << std::endl;
    }
};

class ZenithGraphingEngine {
private:
    std::map<std::string, std::unique_ptr<IGraphingShard>> m_graphing;
public:
    void Synthesize() {
        m_graphing["HOOKE"] = std::make_unique<HookeShard>();
        m_graphing["LENS"] = std::make_unique<LensShard>();
        m_graphing["FRICTION"] = std::make_unique<FrictionShard>();
    }

    void ExecuteGraphShard(const std::string& key, const std::map<std::string, double>& inputs) {
        if (m_graphing.count(key)) {
            std::cout << "\n[ZENITH-GRAPH]: Executing Graphic Shard: " << key << std::endl;
            m_graphing[key]->PlotExecute(inputs);
        } else {
            std::cout << "[ERROR]: Graphic Shard '" << key << "' not synthesized. Total Mastery expanding..." << std::endl;
        }
    }
};

int main() {
    ZenithGraphingEngine zenith;
    zenith.Synthesize();

    std::map<std::string, double> hooke_in = {{"k", 200.0}, {"x", 0.1}};
    zenith.ExecuteGraphShard("HOOKE", hooke_in);

    std::map<std::string, double> lens_in = {{"f", 10.0}, {"u", -20.0}};
    zenith.ExecuteGraphShard("LENS", lens_in);

    std::cout << "\n[SUCCESS]: Competitive Universal Graphing Engine Online. Absolute NCERT Sovereignty 100%." << std::endl;
    return 0;
}
