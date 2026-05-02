#ifndef CLOUD_MAESTRO_HPP
#define CLOUD_MAESTRO_HPP

#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Net {

struct CloudShard {
    SigmaString region;
    SigmaString status;
    SigmaString ip;
};

class ICloudOrchestrator {
public:
    virtual void DeployToCloud(const SigmaString& shardName) = 0;
    virtual void ShowCloudMatrix() const = 0;
    virtual ~ICloudOrchestrator() = default;
};

class CloudMaestro : public ICloudOrchestrator, public SigmaOS::SigmaObject {
private:
    SigmaString m_regions[3];
    SigmaMap<SigmaString, CloudShard> m_active_shards;

public:
    CloudMaestro();
    const char* type_name() const noexcept override { return "CloudMaestro"; }

    void DeployToCloud(const SigmaString& shardName) override;
    void ShowCloudMatrix() const override;
};

} // namespace Net
} // namespace SigmaOS

#endif
