#include "../../../include/sigma_kernel.h"

typedef struct {
    sigma_f64 m[3][3];
} sigma_matrix3x3_t;

void sigma_math_matrix_mult(sigma_matrix3x3_t* a, sigma_matrix3x3_t* b, sigma_matrix3x3_t* res) {
    // 3x3 matrix multiplication logic
    for(int i=0; i<3; i++){
        for(int j=0; j<3; j++){
            res->m[i][j] = 0;
            for(int k=0; k<3; k++) res->m[i][j] += a->m[i][k] * b->m[k][j];
        }
    }
}

void SovereignEdu_Math_Init() {
    sigma_printf("Σ [EDU]: Discrete Mathematics Shard Online.\n");
    sigma_printf("Σ [LOGIC]: Propositional Truth Values & Predicate Logic initialized.\n");
}


