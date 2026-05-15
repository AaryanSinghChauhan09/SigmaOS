#ifndef SOVEREIGN_CORE_UTILS_H
#define SOVEREIGN_CORE_UTILS_H

#include "../include/core/SigmaOOP.hpp"

namespace SigmaOS {
namespace CoreUtils {

class SovereignListDir : public SigmaObject {
public:
    const char* type_name() const noexcept override;
    void Execute(const char* path);
};

class SovereignConcatenate : public SigmaObject {
public:
    const char* type_name() const noexcept override;
    void Execute(const char* file);
};

class SovereignGrepSearch : public SigmaObject {
public:
    const char* type_name() const noexcept override;
    void Execute(const char* pattern, const char* file);
};

class SovereignProcessMonitor : public SigmaObject {
public:
    const char* type_name() const noexcept override;
    void Execute();
};

class SovereignPermissionMod : public SigmaObject {
public:
    const char* type_name() const noexcept override;
    void Execute(const char* permissions, const char* file);
};

class AutoAetherOrchestrator : public SigmaObject {
public:
    const char* type_name() const noexcept override;
    void DispatchCron();
};

class SovereignDataScienceForge : public SigmaObject {
public:
    const char* type_name() const noexcept override;
    void TrainModel(const char* dataSet);
    void PlotGraph(const char* metrics);
};

} // namespace CoreUtils
} // namespace SigmaOS

#endif
