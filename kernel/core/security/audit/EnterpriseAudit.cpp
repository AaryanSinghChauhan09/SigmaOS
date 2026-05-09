#include "../crypto/FIPS140Lattice.hpp"

extern "C" void init_fips140_compliance() {
    SigmaOS::Kernel::Security::FIPS140Lattice::enforceComplianceMode();
}
