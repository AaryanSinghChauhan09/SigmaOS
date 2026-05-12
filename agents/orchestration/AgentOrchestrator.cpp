#include "../quota/AINativeAgent.hpp"

void init_autonomous_agents() {
    SigmaOS::Agents::Quota::AINativeAgent::enforceDynamicQuotas();
}

} // extern "C"
