#ifndef NCERT_BIOLOGY_HPP
#define NCERT_BIOLOGY_HPP

#include "../../../include/SovereignLibC.h"

#include "ncert_base.hpp"

class GeneticsSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "GeneticsSim"; }
    void Simulate() override {
        sigma_printf("[BIOLOGY/NCERT]: Concept: Molecular Basis of Inheritance (Class 12).\n");
        sigma_printf("[BIOLOGY/NCERT]: Complementary DNA Shard: TACG (Silicon-Direct).\n");
    }
    const char* GetConcept() override { return "Genetics_Shard"; }
};

class PlantSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "PlantSim"; }
    void Simulate() override {
        sigma_printf("[BIOLOGY/NCERT]: Concept: Photosynthesis in Higher Plants (Class 11).\n");
        sigma_printf("[BIOLOGY/NCERT]: CO2 + H2O + Light -> Glucose + O2 (Active).\n");
    }
    const char* GetConcept() override { return "Botany_Shard"; }
};

#endif
