#include "../quota/AINativeAgent.hpp"

extern "C" void init_autonomous_agents() {
    SigmaOS::Agents::Quota::AINativeAgent::enforceDynamicQuotas();
}
