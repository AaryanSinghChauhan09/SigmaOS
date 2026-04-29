/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TELEPATHIC TYPING (S-TELEPATHTYPE)
 * =========================================================================
 * Mission: Leverage the S-Neural Engine to predict entire sentences and 
 * contextually complete logic blocks with a single keystroke.
 * =========================================================================
 */

#ifndef SIGMA_TELEPATHTYPE_H
#define SIGMA_TELEPATHTYPE_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- Telepathic Typing Primitives --- */
void telepathtype_init(void);
const char* telepathtype_predict_completion(const char* current_context);
void telepathtype_commit_prediction(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TELEPATHTYPE_H */
