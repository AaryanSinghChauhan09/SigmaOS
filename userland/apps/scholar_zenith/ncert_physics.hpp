#ifndef NCERT_PHYSICS_HPP
#define NCERT_PHYSICS_HPP

#include "../../../include/libc/SovereignLibC.h"

#include "../../../include/ncert_base.hpp"

class GravitationSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "GravitationSim"; }
    void Simulate() override {
        sigma_printf("[PHYSICS/NCERT]: Concept: Universal Gravitation (Class 11).\n");
        sigma_printf("[PHYSICS/NCERT]: G*m1*m2/r^2 = 1.98e20 N (Earth-Moon).\n");
    }
    const char* GetConcept() override { return "Gravitation_Shard"; }
};

class ProjectileSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "ProjectileSim"; }
    void Simulate() override {
        sigma_printf("[PHYSICS/NCERT]: Concept: Motion in a Plane (Class 11).\n");
        sigma_printf("[PHYSICS/NCERT]: Range (45 deg, 20m/s) = 40.8 Meters.\n");
    }
    const char* GetConcept() override { return "Kinematics_Shard"; }
};

class OpticsSim : public INCERTSim {
public:
    const char* type_name() const noexcept override { return "OpticsSim"; }
    void Simulate() override {
        sigma_printf("[PHYSICS/NCERT]: Concept: Reflection & Refraction (Class 10).\n");
        sigma_printf("[PHYSICS/NCERT]: Snell's Law (n1 sin i = n2 sin r) Verified.\n");
    }
    const char* GetConcept() override { return "Optics_Shard"; }
};

#endif
