/*
 * =========================================================================
 * Σ SIGMAOS: RECIPE FORMAT HEADER
 * =========================================================================
 */
#ifndef SIGMA_RECIPE_FORMAT_H
#define SIGMA_RECIPE_FORMAT_H

#include "../../include/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

#define MAX_RECIPE_NAME 64
#define MAX_RECIPE_VERSION 32
#define MAX_DEPS 16
#define MAX_SOURCES 16
#define MAX_BUILD_STEPS 16
#define MAX_LINE_LEN 256

typedef struct {
    char name[MAX_RECIPE_NAME];
    char version[MAX_RECIPE_VERSION];
    
    char build_depends[MAX_DEPS][MAX_RECIPE_NAME];
    sigma_u32 build_deps_count;
    
    char sources[MAX_SOURCES][MAX_LINE_LEN];
    sigma_u32 sources_count;
    
    char build_steps[MAX_BUILD_STEPS][MAX_LINE_LEN];
    sigma_u32 build_steps_count;
    
    char install_steps[MAX_BUILD_STEPS][MAX_LINE_LEN];
    sigma_u32 install_steps_count;
} sigma_recipe_t;

int sigma_recipe_parse(const char* filepath, sigma_recipe_t* recipe_out);
int sigma_recipe_build(const sigma_recipe_t* recipe);

#ifdef __cplusplus
}
#endif

#endif
