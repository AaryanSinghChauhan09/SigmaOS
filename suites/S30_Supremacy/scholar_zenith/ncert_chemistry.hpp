#ifndef NCERT_CHEMISTRY_HPP
#define NCERT_CHEMISTRY_HPP

#include "../../../include/libc/SovereignLibC.h"

#include "../../../include/ncert_base.hpp"

class IdealGasSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "IdealGasSim"; }
    void Simulate() override {
        sigma_printf("[CHEMISTRY/NCERT]: Concept: States of Matter (Class 11).\n");
        sigma_printf("[CHEMISTRY/NCERT]: 1 mole at STP = 22.4 Liters (Verified).\n");
    }
    const char* GetConcept() override { return "Ideal_Gas_Shard"; }
};

class BohrModelSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "BohrModelSim"; }
    void Simulate() override {
        sigma_printf("[CHEMISTRY/NCERT]: Concept: Structure of Atom (Class 11).\n");
        sigma_printf("[CHEMISTRY/NCERT]: Energy in State n=1 = -13.6 eV.\n");
    }
    const char* GetConcept() override { return "Atomic_Shard"; }
};

class OrganicSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "OrganicSim"; }
    void Simulate() override {
        sigma_printf("[CHEMISTRY/NCERT]: Concept: Alcohols, Phenols & Ethers (Class 12).\n");
        sigma_printf("[CHEMISTRY/NCERT]: Functional Shard Identified: -OH (Hydroxyl).\n");
    }
    const char* GetConcept() override { return "Organic_Shard"; }
};

#endif
