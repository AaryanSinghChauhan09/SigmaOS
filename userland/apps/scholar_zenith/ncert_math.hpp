#ifndef NCERT_MATH_HPP
#define NCERT_MATH_HPP

#include "SovereignLibC.h"

#include "ncert_base.hpp"

class MatrixSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "MatrixSim"; }
    void Simulate() override {
        sigma_printf("[MATH/NCERT]: Concept: Matrices & Determinants (Class 12).\n");
        sigma_printf("[MATH/NCERT]: Solving 2x2 Shard Matrix... [DET: 1.0]\n");
    }
    const char* GetConcept() override { return "Matrix_Shard"; }
};

class CalculusSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "CalculusSim"; }
    void Simulate() override {
        sigma_printf("[MATH/NCERT]: Concept: Continuity & Differentiability (Class 12).\n");
        sigma_printf("[MATH/NCERT]: d/dx (x^2) at x=5 = 10.0 (Calculus Shard Active).\n");
    }
    const char* GetConcept() override { return "Calculus_Shard"; }
};

#endif
