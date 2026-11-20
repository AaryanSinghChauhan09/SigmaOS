/* true — exit 0 */
#include "lib.h"
void _start(void) { sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable(); }
