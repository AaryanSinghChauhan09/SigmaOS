#include "sigma_core.h"
#include "sigma_libc.h"

extern "C" {

void media_load_codec(const char* codec) {
    sigma_kprint("[SigmaMedia] Loading atomic codec shard: ");
    sigma_kprint(codec);
    sigma_kprint("\n");
}

}
