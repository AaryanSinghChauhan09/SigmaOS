#ifndef NCERT_BASE_HPP
#define NCERT_BASE_HPP

#include "../../../SigmaOOP.hpp"

class INCERTSim : public SigmaOS::SigmaObject {
public:
    virtual void Simulate() = 0;
    virtual const char* GetConcept() = 0;
};

#endif
