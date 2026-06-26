#ifndef SIGMA_PKG_RECIPE_H
#define SIGMA_PKG_RECIPE_H

#ifdef __cplusplus
extern "C" {
#endif

int sigma_pkg_recipe_load_buffer(const char* text);
const char* sigma_pkg_recipe_last_name(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PKG_RECIPE_H */
