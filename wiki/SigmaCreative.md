# SigmaCreative

**SigmaCreative** is the SigmaOS alternative to the Adobe Creative Suite.

## Features

### SigmaVector (Vector Graphics)
Scalable vector drawing tools (Illustrator alternative):
- Pen tool with Bézier curves
- Shape tools (rectangle, ellipse, polygon, star)
- Path operations (union, intersection, difference, exclusion)
- Gradient and pattern fills
- Stroke and fill management
- Layers and groups
- Vector effects (blur, shadow, glow)
- SVG import/export
- AI-powered vector tracing
- Typography tools with OpenType features

### SigmaRaster (Raster Editing)
Photo manipulation and layer-based image editing (Photoshop alternative):
- Layer-based editing with blending modes
- Selection tools (lasso, magic wand, quick select)
- Adjustment layers (brightness, contrast, hue/saturation, curves)
- Filters (blur, sharpen, noise, artistic effects)
- Clone stamp and healing brush
- Content-aware fill
- RAW image processing
- HDR merging and tone mapping
- Batch processing
- AI-powered object selection and removal
- Export to PNG, JPEG, TIFF, WebP, PSD

### SigmaVideo (Video Editing)
Non-linear video editing (Premiere alternative):
- Multi-track timeline editing
- Video transitions and effects
- Audio mixing and effects
- Color grading with LUTs
- Text and title overlays
- Keyframe animation
- Proxy editing for 4K+ footage
- Export to H.264, H.265, ProRes, VP9
- AI-powered auto-captioning
- Motion tracking and stabilization

## Architecture

```
SigmaCreative Suite
   ├─ SigmaVector (vector graphics engine)
   │   ├─ Path renderer
   │   ├─ Bézier curve engine
   │   ├─ SVG parser/generator
   │   └─ AI vector tracing
   ├─ SigmaRaster (raster graphics engine)
   │   ├─ Layer compositor
   │   ├─ Image processor
   │   ├─ Filter pipeline
   │   └─ RAW processor
   └─ SigmaVideo (video engine)
       ├─ Timeline editor
       ├─ Video decoder/encoder
       ├─ Audio mixer
       └─ Color grading engine
```

## File Formats

| Application | Native Format | Import Formats | Export Formats |
|---|---|---|---|
| SigmaVector | .svector | .svg, .ai, .eps, .pdf | .svg, .pdf, .eps, .png |
| SigmaRaster | .sraster | .psd, .png, .jpg, .tiff, .webp, .raw | .psd, .png, .jpg, .tiff, .webp |
| SigmaVideo | .svideo | .mp4, .mov, .avi, .mkv | .mp4, .mov, .webm, .prores |

## API Interface

```c
// SigmaVector API
int sigma_vector_new_document(vector_doc_t *doc, int width, int height);
int sigma_vector_open(const char *path, vector_doc_t *doc);
int sigma_vector_save(const vector_doc_t *doc, const char *path);
int sigma_vector_add_path(vector_doc_t *doc, path_t *path);
int sigma_vector_apply_fill(vector_doc_t *doc, const char *object, fill_t fill);
int sigma_vector_apply_stroke(vector_doc_t *doc, const char *object, stroke_t stroke);
int sigma_vector_export_svg(const vector_doc_t *doc, const char *path);
int sigma_vector_trace_image(const char *image_path, vector_doc_t *doc);

// SigmaRaster API
int sigma_raster_new_document(raster_doc_t *doc, int width, int height);
int sigma_raster_open(const char *path, raster_doc_t *doc);
int sigma_raster_save(const raster_doc_t *doc, const char *path);
int sigma_raster_add_layer(raster_doc_t *doc, layer_t *layer);
int sigma_raster_apply_filter(raster_doc_t *doc, const char *layer, filter_t filter);
int sigma_raster_adjust_color(raster_doc_t *doc, const char *layer, adjustment_t adj);
int sigma_raster_select_content(raster_doc_t *doc, selection_t *sel);
int sigma_raster_content_aware_fill(raster_doc_t *doc, selection_t *sel);

// SigmaVideo API
int sigma_video_new_project(video_project_t *proj, int width, int height, int fps);
int sigma_video_open(const char *path, video_project_t *proj);
int sigma_video_save(const video_project_t *proj, const char *path);
int sigma_video_add_clip(video_project_t *proj, clip_t *clip, int track, int time);
int sigma_video_add_transition(video_project_t *proj, transition_t trans, int time);
int sigma_video_apply_effect(video_project_t *proj, const char *clip, effect_t effect);
int sigma_video_color_grade(video_project_t *proj, const char *clip, grade_t grade);
int sigma_video_export(const video_project_t *proj, const char *path, codec_t codec);
```

## Integration

- **SigmaFS Integration**: Auto-save to SovereignFS snapshots
- **SigmaAI Integration**: AI-powered vector tracing, object selection, auto-captioning
- **GPU Acceleration**: Vulkan-based rendering for real-time effects
- **Zenith Desktop Integration**: Native Zenith UI components with touch/gesture support

## Performance Characteristics

| Application | GPU Acceleration | Max Resolution | Real-time Preview |
|---|---|---|---|
| SigmaVector | ✅ Vulkan | Unlimited | ✅ Yes |
| SigmaRaster | ✅ Vulkan | 8K+ | ✅ Yes (with proxy) |
| SigmaVideo | ✅ Vulkan + NVENC | 8K+ | ✅ Yes (with proxy) |

## Roadmap

- [x] Architecture design and component specification
- [ ] SigmaVector path engine implementation
- [ ] SigmaRaster layer compositor implementation
- [ ] SigmaVideo timeline editor implementation
- [ ] File format import/export (PSD, AI, SVG)
- [ ] GPU acceleration pipeline
- [ ] AI-powered features (vector tracing, object selection, auto-captioning)
- [ ] Plugin system for third-party filters/effects
- [ ] Mobile versions (SigmaOS Mobile)
- [ ] Cloud collaboration (SigmaOS Cloud)

## Related Modules

- [`kernel/core/vulkan`](../../kernel/core/vulkan/README.md) — GPU acceleration
- [`modules/core/fs`](../../modules/core/fs/README.md) — Filesystem integration
- [`desktop/zenith_accessibility.rs`](../../desktop/zenith_accessibility.rs) — Accessibility features
