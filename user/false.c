/* false — exit 1 */
#include "lib.h"
void _start(void) { sc3(SYS_EXIT, 1, 0, 0); __builtin_unreachable(); }
