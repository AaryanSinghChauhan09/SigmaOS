#ifndef COMPOSITOR_HPP
#define COMPOSITOR_HPP

#include "../../../SigmaOOP.hpp"

namespace SigmaOS {
namespace Graphics {

class SovereignGraphicsCompositor : public SigmaOS::SigmaObject {
public:
    SovereignGraphicsCompositor();
    const char* type_name() const noexcept override { return "SovereignGraphicsCompositor"; }

    void CommitFrameShard(const char* shard_id, const char* buffer_data);
    void ExecuteAlphaBlend(const char* overlay_shard);
};

} // namespace Graphics
} // namespace SigmaOS

#endif
