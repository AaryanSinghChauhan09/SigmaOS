#ifndef SVG_RENDERER_HPP
#define SVG_RENDERER_HPP

#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Graphics {

class SovereignSVGRenderer : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignSVGRenderer"; }

    void RasterizePath(const char* path_shard);
    void RenderWidget(const char* svg_id);
};

} // namespace Graphics
} // namespace SigmaOS

#endif

