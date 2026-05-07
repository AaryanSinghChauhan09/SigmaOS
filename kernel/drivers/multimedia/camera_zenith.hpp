#ifndef CAMERA_ZENITH_HPP
#define CAMERA_ZENITH_HPP

#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Multimedia {

class RawHardwareSensor : public SigmaObject {
protected:
    volatile unsigned int* mmio_register_base;
public:
    RawHardwareSensor(unsigned int* base) : mmio_register_base(base) {}
    const char* type_name() const noexcept override { return "RawHardwareSensor"; }
    virtual void TriggerHardwareInterrupt() = 0;
};

class WaitFreeCameraFeed : public RawHardwareSensor {
public:
    WaitFreeCameraFeed(unsigned int* base) : RawHardwareSensor(base) {}
    void TriggerHardwareInterrupt() override;
    void FetchFrameToCache();
};

class INeuralFilter {
public:
    virtual void MapTensors() = 0;
    virtual ~INeuralFilter() = default;
};

class VisualBlockLogicEngine : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "VisualBlockLogicEngine"; }
    void CompileScratchBlocksToASM(const char* logicTreeRoot);
};

class SnapchatNeuralMesh : public INeuralFilter {
public:
    void MapTensors() override;
};

class PhotographicOrchestrator {
private:
    WaitFreeCameraFeed* activeSensor;
    INeuralFilter* activeFilter;
public:
    PhotographicOrchestrator(WaitFreeCameraFeed* s, INeuralFilter* f) : activeSensor(s), activeFilter(f) {}
    void Ignite();
};

} // namespace Multimedia
} // namespace SigmaOS

#endif
