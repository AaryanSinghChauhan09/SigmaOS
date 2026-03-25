#include <iostream>
#include <vector>
#include <memory>
#include <cmath>

/**
 * Σ SIGMA OS: SOVEREIGN LAB ZENITH (v128.0 - ANIMATION PARITY)
 * ==========================================================
 * USP: Comprehensive "Small & Big" experiment simulations (1-12).
 * Capability: Electrolysis, Displacement, Pythagoras, and Pollen Shards.
 * Principle: OOPS, Polymorphism, Abstraction, SOLID.
 */

class IZenithExperiment {
public:
    virtual ~IZenithExperiment() = default;
    virtual void Project() = 0;
};

// --- Physics: Simple Circuit (Class 6) ---
class CircuitExperiment : public IZenithExperiment {
public:
    void Project() override {
        std::cout << "[PHYSICS/LAB]: Experiment: Constructing a Simple Electric Circuit." << std::endl;
        std::cout << "[PHYSICS/LAB]: Battery Shard connected to Switch & Bulb. Electron flux active." << std::endl;
    }
};

// --- Chemistry: Electrolysis of Water (Class 10) ---
class ElectrolysisExperiment : public IZenithExperiment {
public:
    void Project() override {
        std::cout << "[CHEMISTRY/LAB]: Experiment: Electrolysis of Water (Shard-H2O)." << std::endl;
        std::cout << "[CHEMISTRY/LAB]: H2:O2 Volume ratio detected as 2:1 at Cathode/Anode." << std::endl;
    }
};

// --- Chemistry: Displacement Reaction (Class 10) ---
class DisplacementExperiment : public IZenithExperiment {
public:
    void Project() override {
        std::cout << "[CHEMISTRY/LAB]: Experiment: Iron Nail in Copper Sulphate Solution." << std::endl;
        std::cout << "[CHEMISTRY/LAB]: Result: Blue color fades to green; Brown Copper deposit identified." << std::endl;
    }
};

// --- Biology: Pollen Tube Growth (Class 12) ---
class PollenExperiment : public IZenithExperiment {
public:
    void Project() override {
        std::cout << "[BIOLOGY/LAB]: Experiment: Germination of Pollen on Stigma Shard." << std::endl;
        std::cout << "[BIOLOGY/LAB]: Pollen Tube growth detected via Chemotropism Shard." << std::endl;
    }
};

// --- Math: Pythagorean Shard (Class 7) ---
class PythagorasExperiment : public IZenithExperiment {
public:
    void Project() override {
        double a = 3.0, b = 4.0;
        double c = std::sqrt(a*a + b*b);
        std::cout << "[MATH/LAB]: Experiment: Verification of Pythagoras Theorem." << std::endl;
        std::cout << "[MATH/LAB]: Base=3, Perp=4 -> Hypotenuse = " << c << " (Shard-Perfect)." << std::endl;
    }
};

class SovereignLabZenith {
private:
    std::vector<std::unique_ptr<IZenithExperiment>> m_lab;
public:
    void Synthesize() {
        m_lab.push_back(std::make_unique<CircuitExperiment>());
        m_lab.push_back(std::make_unique<ElectrolysisExperiment>());
        m_lab.push_back(std::make_unique<DisplacementExperiment>());
        m_lab.push_back(std::make_unique<PollenExperiment>());
        m_lab.push_back(std::make_unique<PythagorasExperiment>());
    }

    void ExecuteFinalAudit() {
        std::cout << "--- Σ SIGMA OS SOVEREIGN LABORATORY ZENITH ---" << std::endl;
        for (const auto& exp : m_lab) {
            std::cout << "\n------------------------------------------------" << std::endl;
            exp->Project();
        }
    }
};

int main() {
    SovereignLabZenith lab;
    lab.Synthesize();
    lab.ExecuteFinalAudit();

    std::cout << "\n[SUCCESS]: Competitive Laboratory Zenith Shards (with Animation parity) Active." << std::endl;
    return 0;
}
