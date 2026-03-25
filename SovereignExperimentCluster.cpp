#include <iostream>
#include <vector>
#include <memory>

/**
 * Σ SIGMA OS: SOVEREIGN EXPERIMENT CLUSTER (v128.0 - LAB ZENITH)
 * ============================================================
 * USP: Comprehensive "Small & Big" experiment simulations (1-12).
 * Capability: Ohm's Law, Acids/Metals, Transpiration, and Geometry.
 * Principle: OOPS, Polymorphism, Abstraction, SOLID.
 */

class IExperiment {
public:
    virtual ~IExperiment() = default;
    virtual void Execute() = 0;
};

// --- Physics Shard: Ohm's Law (Class 10) ---
class OhmsLawExperiment : public IExperiment {
public:
    void Execute() override {
        double R = 10.0; // Ohms
        double V = 2.0;
        double I = V / R;
        std::cout << "[PHYSICS/EXP]: Experiment: Verification of Ohm's Law." << std::endl;
        std::cout << "[PHYSICS/EXP]: V=2V, R=10-ohm -> I = " << I << " Amperes (Verified)." << std::endl;
    }
};

// --- Physics Shard: Refraction (Class 10) ---
class RefractionExperiment : public IExperiment {
public:
    void Execute() override {
        std::cout << "[PHYSICS/EXP]: Experiment: Refraction through Glass Shard." << std::endl;
        std::cout << "[PHYSICS/EXP]: Displacement measured at various angles. Shard-Ref Index: 1.5." << std::endl;
    }
};

// --- Chemistry Shard: Acid-Metal Reactor (Class 10) ---
class AcidMetalExperiment : public IExperiment {
public:
    void Execute() override {
        std::cout << "[CHEMISTRY/EXP]: Experiment: Zinc + Sulphuric Acid Reaction." << std::endl;
        std::cout << "[CHEMISTRY/EXP]: Observation: Hydrogen Gas Evolution (Brum-Sound popping)." << std::endl;
    }
};

// --- Chemistry Shard: Boiling Point (Class 9) ---
class BoilingPointExperiment : public IExperiment {
public:
    void Execute() override {
        std::cout << "[CHEMISTRY/EXP]: Experiment: Determination of Boiling Point of Water." << std::endl;
        std::cout << "[CHEMISTRY/EXP]: Latent Heat of Vaporization Shard Stabilized at 100-deg C." << std::endl;
    }
};

// --- Biology Shard: Transpiration (Class 11) ---
class TranspirationExperiment : public IExperiment {
public:
    void Execute() override {
        std::cout << "[BIOLOGY/EXP]: Experiment: Measuring Rate of Transpiration (Potometer)." << std::endl;
        std::cout << "[BIOLOGY/EXP]: Leaf Stomata open @ 298.15K. Shard-Water flux identified." << std::endl;
    }
};

// --- Biology Shard: Starch Test (Class 7) ---
class StarchTestExperiment : public IExperiment {
public:
    void Execute() override {
        std::cout << "[BIOLOGY/EXP]: Experiment: Test for Presence of Starch (Iodine)." << std::endl;
        std::cout << "[BIOLOGY/EXP]: Leaf Shard color change: Blue-Black confirmed." << std::endl;
    }
};

// --- Math Shard: Tangents (Class 10) ---
class TangentExperiment : public IExperiment {
public:
    void Execute() override {
        std::cout << "[MATH/EXP]: Experiment: Drawing Tangents to a Circle from External Point." << std::endl;
        std::cout << "[MATH/EXP]: Intersection Shard identifies exactly 2 Tangent vectors." << std::endl;
    }
};

class SovereignExperimentCluster {
private:
    std::vector<std::unique_ptr<IExperiment>> m_cluster;
public:
    void Synthesize() {
        m_cluster.push_back(std::make_unique<OhmsLawExperiment>());
        m_cluster.push_back(std::make_unique<RefractionExperiment>());
        m_cluster.push_back(std::make_unique<AcidMetalExperiment>());
        m_cluster.push_back(std::make_unique<BoilingPointExperiment>());
        m_cluster.push_back(std::make_unique<TranspirationExperiment>());
        m_cluster.push_back(std::make_unique<StarchTestExperiment>());
        m_cluster.push_back(std::make_unique<TangentExperiment>());
    }

    void ExecuteFullLaboratoryAudit() {
        std::cout << "--- Σ SIGMA OS SOVEREIGN EXPERIMENT CLUSTER ---" << std::endl;
        for (const auto& exp : m_cluster) {
            std::cout << "\n------------------------------------------------" << std::endl;
            exp->Execute();
        }
    }
};

int main() {
    SovereignExperimentCluster cluster;
    cluster.Synthesize();
    cluster.ExecuteFullLaboratoryAudit();

    std::cout << "\n[SUCCESS]: Competitive 'Small & Big' Experiment Cluster Synthesized." << std::endl;
    return 0;
}
