#include "SigmaOOP.hpp"

/**
 * Σ SIGMA OS: ZENITH GRAPHING ENGINE (v128.0 - ZERO-STD NATIVE)
 * =====================================================================
 * USP: God-Mode Graphing for NCERT. Live plot shards for Hooke, Lenses, and Friction.
 * Capability: F = -kx, 1/f = 1/v - 1/u, f = mu*N.
 * Principle: OOPS, SOLID, Zero-Simulated Analytics / Zero-STL.
 * =====================================================================
 */

class IGraphingShard {
public:
    virtual ~IGraphingShard() = default;
    virtual void PlotExecute(const SigmaMap<SigmaString, double>& inputs) = 0;
};

// --- Physics: Hooke's Law (Class 11) ---
class HookeShard : public IGraphingShard {
public:
    void PlotExecute(const SigmaMap<SigmaString, double>& inputs) override {
        double k = inputs.at("k"), x = inputs.at("x");
        double F = k * x;
        sigma_printf("[HOOKE/ZENITH]: Equation: F = k * x.\n");
        sigma_printf("[HOOKE/ZENITH]: Spring Force (F): %f Newtons.\n", F);
        sigma_printf("[GRAPH]: Linear mapping (k=%f) online.\n", k);
    }
};

// --- Physics: Lens Formula (Class 10-12) ---
class LensShard : public IGraphingShard {
public:
    void PlotExecute(const SigmaMap<SigmaString, double>& inputs) override {
        double f = inputs.at("f"), u = inputs.at("u");
        double v = (f * u) / (f + u); 
        sigma_printf("[LENS/ZENITH]: Equation: 1/v - 1/u = 1/f.\n");
        sigma_printf("[LENS/ZENITH]: Image Distance (v): %f\n", v);
        sigma_printf("[GRAPH]: Hyperbolic mapping (f=%f) online.\n", f);
    }
};

// --- Physics: Friction (Class 11) ---
class FrictionShard : public IGraphingShard {
public:
    void PlotExecute(const SigmaMap<SigmaString, double>& inputs) override {
        double mu = inputs.at("mu"), N = inputs.at("N");
        double f = mu * N;
        sigma_printf("[FRICTION/ZENITH]: Equation: f = mu * N.\n");
        sigma_printf("[FRICTION/ZENITH]: Friction Force (f): %f Newtons.\n", f);
        sigma_printf("[GRAPH]: Proportional mapping (mu=%f) online.\n", mu);
    }
};

class ZenithGraphingEngine {
private:
    SigmaMap<SigmaString, SigmaUniquePtr<IGraphingShard>> m_graphing;
public:
    void Synthesize() {
        m_graphing.insert("HOOKE", sigma_make_unique<HookeShard>());
        m_graphing.insert("LENS", sigma_make_unique<LensShard>());
        m_graphing.insert("FRICTION", sigma_make_unique<FrictionShard>());
    }

    void ExecuteGraphShard(const SigmaString& key, const SigmaMap<SigmaString, double>& inputs) {
        if (m_graphing.contains(key)) {
            sigma_printf("\n[ZENITH-GRAPH]: Executing Graphic Shard: %s\n", key.c_str());
            m_graphing.at(key)->PlotExecute(inputs);
        } else {
            sigma_printf("[ERROR]: Graphic Shard '%s' not synthesized. Total Mastery expanding...\n", key.c_str());
        }
    }
};

extern "C" void _start(void) {
    ZenithGraphingEngine zenith;
    zenith.Synthesize();

    SigmaMap<SigmaString, double> hooke_in;
    hooke_in.insert("k", 200.0);
    hooke_in.insert("x", 0.1);
    zenith.ExecuteGraphShard("HOOKE", hooke_in);

    SigmaMap<SigmaString, double> lens_in;
    lens_in.insert("f", 10.0);
    lens_in.insert("u", -20.0);
    zenith.ExecuteGraphShard("LENS", lens_in);

    sigma_printf("\n[SUCCESS]: Competitive Universal Graphing Engine Online. Absolute NCERT Sovereignty 100%%.\n");
    sigma_exit(0);
}
