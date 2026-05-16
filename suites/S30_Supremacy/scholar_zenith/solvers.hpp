#ifndef SOLVERS_HPP
#define SOLVERS_HPP

#include "../../../include/libc/SovereignLibC.h"

#include "../../../include/SigmaOOP.hpp"

// Simple math primitives for Zero-Dependency environment
static inline double sigma_sqrt(double x) {
    if (x < 0) return 0;
    double z = 1.0;
    for (int i = 0; i < 10; i++) {
        z -= (z * z - x) / (2 * z);
    }
    return z;
}

static inline double sigma_pow(double base, double exp) {
    double res = 1.0;
    int iexp = (int)exp;
    for (int i = 0; i < iexp; i++) res *= base;
    return res;
}

class ISolverShard : public SigmaOS::SigmaObject {
public:
    virtual void Solve() = 0;
};

class KinematicsSolver : public ISolverShard {
public:
    const char* type_name() const noexcept override { return "KinematicsSolver"; }
    void Solve() override {
        double u = 0.0, a = 9.8, t = 5.0;
        double v = u + a * t;
        double s = u * t + 0.5 * a * t * t;
        sigma_printf("[PHYSICS/SOLVE]: Kinematics (u=0, a=9.8, t=5)\n");
        sigma_printf("[PHYSICS/SOLVE]: Final Velocity (v): %f m/s\n", v);
        sigma_printf("[PHYSICS/SOLVE]: Displacement (s): %f m\n", s);
    }
};

class MolaritySolver : public ISolverShard {
public:
    const char* type_name() const noexcept override { return "MolaritySolver"; }
    void Solve() override {
        double moles = 0.5, volume_litres = 2.0;
        double molarity = moles / volume_litres;
        sigma_printf("[CHEMISTRY/SOLVE]: Molarity (n=0.5, V=2.0L)\n");
        sigma_printf("[CHEMISTRY/SOLVE]: Result: %f M (mol/L)\n", molarity);
    }
};

class HeronsSolver : public ISolverShard {
public:
    const char* type_name() const noexcept override { return "HeronsSolver"; }
    void Solve() override {
        double a = 3, b = 4, c = 5;
        double s = (a + b + c) / 2.0;
        double area = sigma_sqrt(s * (s - a) * (s - b) * (s - c));
        sigma_printf("[MATH/SOLVE]: Heron's Formula (sides 3, 4, 5)\n");
        sigma_printf("[MATH/SOLVE]: Area Shard: %f sq units (Verified)\n", area);
    }
};

class HalfLifeSolver : public ISolverShard {
public:
    const char* type_name() const noexcept override { return "HalfLifeSolver"; }
    void Solve() override {
        double N0 = 100.0, t = 10.0, T = 3.3; 
        double N = N0 * sigma_pow(0.5, t / T);
        sigma_printf("[PHYSICS/SOLVE]: Radioactivity (N0=100, t=10, T=3.3)\n");
        sigma_printf("[PHYSICS/SOLVE]: Remaining Shard (N): %f units.\n", N);
    }
};

#endif
