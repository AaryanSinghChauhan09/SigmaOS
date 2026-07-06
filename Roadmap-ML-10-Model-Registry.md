# SigmaOS Roadmap: On-Device Model Registry
Version, store, and roll back AI/ML models using SovereignFS snapshots.
## Goals
- MLflow-inspired experiment metadata store in sigma_db
- Model artefact versioning with CoW snapshots
## Key Milestones
- [ ] Model metadata schema in sigma_db
- [ ] CoW snapshot on every fine-tune run
- [ ] CLI: sigma-model list | rollback | deploy
"@

"Roadmap-AI-11-Vision-Model.md" = @"
# SigmaOS Roadmap: On-Device Vision Model
Run quantised image classification and object detection locally.
## Goals
- MobileNetV3-Q8 inference for desktop screenshot analysis
- YOLO-nano for real-time webcam object detection
## Key Milestones
- [ ] NCHW tensor layout support in sigma_math
- [ ] JPEG/PNG decoder stub for image input
- [ ] Bounding box overlay in Zenith compositor