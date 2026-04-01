"use strict";

/**
 * Σ SOVEREIGN MATH UNIT (SMU)
 * Low-level, User-defined math functions to reduce HLL dependency (Math.*).
 */
export const SMU = {
    abs: (x) => (x < 0) ? -x : x,
    pow: (base, exp) => {
        let res = 1;
        for (let i = 0; i < exp; i++) res *= base;
        return res;
    },
    // Custom random generator to avoid Math.random() dependency
    seed: 12345,
    random: () => {
        SMU.seed = (SMU.seed * 16807) % 2147483647;
        return (SMU.seed - 1) / 2147483646;
    }
};
