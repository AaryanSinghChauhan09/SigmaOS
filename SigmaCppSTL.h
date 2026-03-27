/*
 * Σ SIGMA OS: SOVEREIGN C++ STL COMPATIBILITY LAYER (v7.1)
 * ==========================================================
 * This file provides aliases for the core SigmaOOP types 
 * for backward compatibility with legacy shards.
 */

#ifndef SIGMACPPS_H
#define SIGMACPPS_H

#include "SigmaOOP.hpp"

// Aliases for transition
template <typename T>
using SigmaVector = SigmaArray<T>;

// Standardize Push/Size to legacy capitalized names if needed, 
// or update the caller. 
// NOTE: SigmaArray already uses 'push' and 'size'.
// Adding wrapper for 'Push' and 'Size' to support SigmaFinalIntegration.cpp

template <typename T>
class SigmaVectorLegacy : public SigmaArray<T> {
public:
    template<typename U>
    void Push(U&& val) { this->push(static_cast<U&&>(val)); }
    sigma_usize Size() const { return this->size(); }
};

#define SigmaVector SigmaVectorLegacy

#endif // SIGMACPPS_H
