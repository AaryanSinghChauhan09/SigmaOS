// ============================================
// MACHINE LEARNING COMMANDS
// ============================================
void sigma_load_ml_commands(void) {
    if (!g_command_library) return;
    
    // Model Training
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_ml", "Machine Learning Framework",
        "sigma_ml [train|predict|evaluate|deploy] [model] [data]",
        "sigma_ml train random_forest data.csv --target=label --test_size=0.2",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Complete ML training and deployment"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_train", "Model Training",
        "sigma_train [algorithm] [dataset] [hyperparameters]",
        "sigma_train neural_network images/ --epochs=100 --batch=32 --gpu=true",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Train ML models with various algorithms"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_predict", "Model Prediction",
        "sigma_predict [model] [input] [output]",
        "sigma_predict model.pkl new_data.csv predictions.csv",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Make predictions with trained models"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_cv", "Cross Validation",
        "sigma_cv [model] [data] [folds]",
        "sigma_cv classifier.csv data.csv 5 --stratified=true",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Perform cross-validation on models"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_tune", "Hyperparameter Tuning",
        "sigma_tune [model] [search_space] [method]",
        "sigma_tune xgboost params.json bayesian --iterations=100",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Optimize model hyperparameters"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_feature", "Feature Engineering",
        "sigma_feature [extract|select|transform] [data] [method]",
        "sigma_feature select data.csv correlation --top=20",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Feature extraction and selection"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_preprocess", "Data Preprocessing",
        "sigma_preprocess [normalize|encode|impute] [data]",
        "sigma_preprocess normalize data.csv --method=standard",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Preprocess data for ML"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_nlp", "NLP Processing",
        "sigma_nlp [tokenize|embed|classify] [text] [model]",
        "sigma_nlp classify reviews.txt sentiment --model=bert",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Natural language processing"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_cv2", "Computer Vision",
        "sigma_cv2 [detect|recognize|segment] [images] [model]",
        "sigma_cv2 detect photos/ objects --confidence=0.8",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Computer vision tasks"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_time", "Time Series Analysis",
        "sigma_time [forecast|anomaly|pattern] [data] [model]",
        "sigma_time forecast sales.csv lstm --horizon=30",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Time series forecasting and analysis"
    };
    
    // Deep Learning
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_dl", "Deep Learning Framework",
        "sigma_dl [architecture] [dataset] [training]",
        "sigma_dl cifar10 resnet50 --epochs=200 --augment=true",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Deep learning model training"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_gan", "GAN Training",
        "sigma_gan [type] [dataset] [generator] [discriminator]",
        "sigma_gan dcgan faces/ --latent=100 --epochs=1000",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Train Generative Adversarial Networks"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_rl", "Reinforcement Learning",
        "sigma_rl [environment] [agent] [training]",
        "sigma_rl cartpole dqn --episodes=10000 --render=true",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Reinforcement learning training"
    };
    
    // Model Management
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_model", "Model Management",
        "sigma_model [save|load|export|compare] [model]",
        "sigma_model export model.pkl --format=onnx --optimize=true",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Manage ML models"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_deploy_ml", "Deploy ML Model",
        "sigma_deploy_ml [model] [endpoint] [scaling]",
        "sigma_deploy_ml model.pkl api/ --auto_scale=true --quantum=true",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Deploy models to production"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_explain", "Model Explainability",
        "sigma_explain [model] [data] [method]",
        "sigma_explain model.pkl sample.csv shap --visualize=true",
        "SigmaOS", true, false, SIGMA_CMD_ML,
        "Explain model predictions"
    };
    
    printf("[Command Library] Loaded %d ML commands\n", 16);
}

// ============================================
// VISUALIZATION & GRAPH PLOTTING COMMANDS
// ============================================
void sigma_load_visualization_commands(void) {
    if (!g_command_library) return;
    
    // Basic Plotting
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_plot", "Data Plotting",
        "sigma_plot [data] [chart_type] [options]",
        "sigma_plot data.csv line --x=time --y=value --title='Sales Trend'",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Create data visualizations"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_chart", "Chart Generator",
        "sigma_chart [type] [data] [output] [styling]",
        "sigma_chart pie sales.csv chart.png --colors=auto --3d=true",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Generate various chart types"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_graph", "Graph Plotting",
        "sigma_graph [function] [range] [render]",
        "sigma_graph 'sin(x)*exp(-x/10)' 0:20 plot.png --grid=true",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Plot mathematical functions"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_3d", "3D Visualization",
        "sigma_3d [data] [plot_type] [interactive]",
        "sigma_3d surface.csv surface --interactive=true --rotate=auto",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Create 3D visualizations"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_dashboard", "Dashboard Builder",
        "sigma_dashboard [create|update] [widgets] [data]",
        "sigma_dashboard create 'kpi,chart,table' realtime_data --layout=grid",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Build interactive dashboards"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_map", "Geographic Visualization",
        "sigma_map [data] [type] [region]",
        "sigma_map locations.csv heatmap world --cluster=true",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Create maps and geospatial plots"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_network", "Network Visualization",
        "sigma_network [nodes] [edges] [layout]",
        "sigma_network nodes.csv edges.csv force --labels=true",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Visualize network graphs"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_tree", "Tree/Hierarchy Plot",
        "sigma_tree [data] [orientation] [collapse]",
        "sigma_tree hierarchy.json horizontal --interactive=true",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Visualize hierarchical data"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_heatmap", "Heatmap Generator",
        "sigma_heatmap [matrix] [colors] [labels]",
        "sigma_heatmap correlation.csv diverging --annotate=true",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Create heatmaps from matrices"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_animation", "Animated Plots",
        "sigma_animation [data] [frames] [type]",
        "sigma_animation timeseries.csv 100 line --save=video.mp4",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Create animated visualizations"
    };
    
    // Advanced Visualization
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_vr", "VR Visualization",
        "sigma_vr [data] [environment] [interaction]",
        "sigma_vr molecules.json lab --hands=true --collaborate=true",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Virtual reality data visualization"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_ar", "AR Visualization",
        "sigma_ar [data] [overlay] [device]",
        "sigma_ar analytics.json hologram --device=glasses",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Augmented reality data overlay"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_hologram", "Holographic Display",
        "sigma_hologram [data] [type] [rotation]",
        "sigma_hologram structure.pdb molecule --auto_rotate=true",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Display 3D holograms"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_live", "Live Data Stream",
        "sigma_live [source] [chart] [refresh]",
        "sigma_live kafka:metrics real_time --refresh=1s --alerts=threshold",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Real-time streaming visualizations"
    };
    
    // Export & Share
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_export", "Export Visualization",
        "sigma_export [chart] [format] [quality]",
        "sigma_export plot.svg pdf --resolution=300dpi",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Export plots in various formats"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_share", "Share Visualization",
        "sigma_share [chart] [platform] [permissions]",
        "sigma_share dashboard.png web --public=true --embed=true",
        "SigmaOS", true, false, SIGMA_CMD_VISUALIZATION,
        "Share visualizations online"
    };
    
    printf("[Command Library] Loaded %d visualization commands\n", 16);
}

// ============================================
// CAMERA & IMAGE COMMANDS (MIT Scratch + Snapchat USP)
// ============================================
void sigma_load_camera_commands(void) {
    if (!g_command_library) return;
    
    // Basic Camera
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_cam", "SigmaOS Camera",
        "sigma_cam [capture|record|stream] [options]",
        "sigma_cam capture --filter=beauty --ar=1:1 --share=instagram",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Advanced camera with AI filters and effects"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_snap", "Quick Capture (Snapchat Style)",
        "sigma_snap [mode] [filter] [duration]",
        "sigma_snap selfie dog_ears --timer=3s --send=friends",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Quick photo/video with fun filters"
    };
    
    // Scratch-like Programming
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_scratch_cam", "Visual Camera Programming",
        "sigma_scratch_cam [project] [blocks] [run]",
        "sigma_scratch_cam project1 'when_motion_detect,play_sound,flash'",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "MIT Scratch-style visual camera programming"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_block_cam", "Block-Based Camera Control",
        "sigma_block_cam [drag_and_drop_blocks]",
        "sigma_block_cam --blocks='start,capture,if_motion,save,else,stream'",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Drag-and-drop camera automation blocks"
    };
    
    // AI Filters & Effects
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_filter", "AI Camera Filters",
        "sigma_filter [type] [intensity] [realtime]",
        "sigma_filter face_retouch 80% --realtime=true --smooth_skin=true",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "AI-powered beauty and style filters"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_lens", "AR Camera Lenses",
        "sigma_lens [effect] [tracking] [share]",
        "sigma_lens rainbow_vomit face_tracking --save_to=memories",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Snapchat-style AR lenses and effects"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_mask", "Face Masks & Avatars",
        "sigma_mask [style] [expressions] [animate]",
        "sigma_mask cartoon_avatar expressions=mimic --animate=true",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Animated face masks and avatars"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_bg", "Background Effects",
        "sigma_bg [effect] [blur] [replace]",
        "sigma_bg replace --image=beach.jpg --edge_smooth=true",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Background blur, replacement, effects"
    };
    
    // Advanced Camera Features
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_studio", "Camera Studio Mode",
        "sigma_studio [setup] [lights] [greenscreen]",
        "sigma_studio portrait --key_light=80% --fill_light=40% --green_screen=true",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Professional studio setup control"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_detect", "Object/Person Detection",
        "sigma_detect [target] [action] [alert]",
        "sigma_detect 'person,cat' record --alert=notification --zone=kitchen",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Smart detection and alerts"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_track", "Motion Tracking",
        "sigma_track [object] [smooth] [predict]",
        "sigma_track face smooth=high predict=3frames",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Advanced motion and face tracking"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_time_lapse", "Time Lapse",
        "sigma_time_lapse [interval] [duration] [ramp]",
        "sigma_time_lapse 5s 2h --exposure_ramp --deflicker=true",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Create professional time-lapse videos"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_slow_mo", "Slow Motion",
        "sigma_slow_mo [fps] [resolution] [ramp]",
        "sigma_slow_mo 240 1080p --ramp_speed=variable --ai_interpolate",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "High FPS slow motion with AI interpolation"
    };
    
    // Image Processing
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_edit", "Image Editor",
        "sigma_edit [image] [tools] [effects]",
        "sigma_edit photo.jpg 'crop,adjust,filter' --ai_enhance=true",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "AI-powered image editing"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_collage", "Photo Collage",
        "sigma_collage [photos] [layout] [style]",
        "sigma_collage '*.jpg' mosaic --ai_arrange=true --theme=vacation",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "AI-generated photo collages"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_panorama", "Panorama Stitcher",
        "sigma_panorama [photos] [projection] [hdr]",
        "sigma_panorama '*.jpg' spherical --hdr_merge --ghost_remove=true",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Create seamless panoramas"
    };
    
    // Sharing & Social
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_stories", "Create Stories",
        "sigma_stories [media] [stickers] [music]",
        "sigma_stories video.mp4 stickers=auto music=suggest --platform=all",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Create social media stories"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_live", "Live Stream",
        "sigma_live [platform] [quality] [interact]",
        "sigma_live youtube 4k --chat_overlay=true --donation_alert=true",
        "SigmaOS", true, false, SIGMA_CMD_CAMERA,
        "Multi-platform live streaming"
    };
    
    printf("[Command Library] Loaded %d camera commands\n", 20);
}

// ============================================
// SETUP & INSTALLATION COMMANDS
// ============================================
void sigma_load_setup_commands(void) {
    if (!g_command_library) return;
    
    // System Setup
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_setup", "System Setup Wizard",
        "sigma_setup [profile] [options] [automation]",
        "sigma_setup developer --ide='vscode,intellij' --languages='python,js' --ai=true",
        "SigmaOS", true, true, SIGMA_CMD_SETUP,
        "Interactive system setup wizard"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_install", "Package Installation",
        "sigma_install [package] [version] [source]",
        "sigma_install nodejs 18 lts --with_npm=true --global_tools='yarn,pnpm'",
        "SigmaOS", true, false, SIGMA_CMD_SETUP,
        "Install packages with dependencies"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_env", "Environment Setup",
        "sigma_env [language] [version] [tools]",
        "sigma_env python 3.11 --venv=default --packages='numpy,pandas,jupyter'",
        "SigmaOS", true, false, SIGMA_CMD_SETUP,
        "Setup development environments"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_config", "System Configuration",
        "sigma_config [component] [setting] [value]",
        "sigma_config display resolution=4k --scale=150% --night_light=auto",
        "SigmaOS", true, false, SIGMA_CMD_SETUP,
        "Configure system components"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_restore", "System Restore",
        "sigma_restore [point] [selective] [verify]",
        "sigma_restore 'clean_install' --selective='settings,apps' --verify=true",
        "SigmaOS", true, true, SIGMA_CMD_SETUP,
        "Restore from backup points"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_migrate", "Migration Assistant",
        "sigma_migrate [from_os] [what] [how]",
        "sigma_migrate macos all --method=one_click --apps='native'",
        "SigmaOS", true, false, SIGMA_CMD_SETUP,
        "Migrate from other operating systems"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_update", "System Update",
        "sigma_update [type] [schedule] [rollback]",
        "sigma_update full --schedule=weekly --rollback_point=true",
        "SigmaOS", true, true, SIGMA_CMD_SETUP,
        "Update system with safety features"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_clone", "System Cloning",
        "sigma_clone [source] [destination] [method]",
        "sigma_clone current_machine new_disk --method=quantum_sync --verify=true",
        "SigmaOS", true, true, SIGMA_CMD_SETUP,
        "Clone system to new hardware"
    };
    
    printf("[Command Library] Loaded %d setup commands\n", 8);
}

// ============================================
// SECURITY COMMANDS
// ============================================
void sigma_load_security_commands(void) {
    if (!g_command_library) return;
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_encrypt", "File Encryption",
        "sigma_encrypt [file] [method] [key]",
        "sigma_encrypt secrets.txt aes256 --quantum_resistant=true --biometric=true",
        "SigmaOS", true, false, SIGMA_CMD_SECURITY,
        "Encrypt files with quantum-resistant algorithms"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_vault", "Password Vault",
        "sigma_vault [add|get|generate] [entry]",
        "sigma_vault generate github --length=32 --symbols=true --save=true",
        "SigmaOS", true, false, SIGMA_CMD_SECURITY,
        "Secure password management"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_scan", "Security Scanner",
        "sigma_scan [target] [type] [report]",
        "sigma_scan system full --realtime=true --auto_remediate=true",
        "SigmaOS", true, true, SIGMA_CMD_SECURITY,
        "Scan for security vulnerabilities"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_firewall", "Firewall Manager",
        "sigma_firewall [allow|deny|monitor] [service|port]",
        "sigma_firewall allow ssh --from=home_network --log=true",
        "SigmaOS", true, true, SIGMA_CMD_SECURITY,
        "Manage firewall rules"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_auth", "Authentication",
        "sigma_auth [method] [setup|verify]",
        "sigma_auth biometric setup --types='face,fingerprint' --backup=true",
        "SigmaOS", true, false, SIGMA_CMD_SECURITY,
        "Multi-factor authentication"
    };
    
    printf("[Command Library] Loaded %d security commands\n", 5);
}

// ============================================
// QUANTUM COMMANDS
// ============================================
void sigma_load_quantum_commands(void) {
    if (!g_command_library) return;
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_quantum", "Quantum Operations",
        "sigma_quantum [operation] [target] [acceleration]",
        "sigma_quantum enable computation --algorithm=shor --qubits=1024",
        "SigmaOS", true, false, SIGMA_CMD_QUANTUM,
        "Enable quantum computing features"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_q_encrypt", "Quantum Encryption",
        "sigma_q_encrypt [data] [protocol] [key]",
        "sigma_q_encrypt message.txt bb84 --key_distribution=quantum_channel",
        "SigmaOS", true, false, SIGMA_CMD_QUANTUM,
        "Quantum key distribution encryption"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_sim", "Quantum Simulator",
        "sigma_sim [circuit] [shots] [backend]",
        "sigma_sim grover 10000 --backend=local_qpu --optimize=true",
        "SigmaOS", true, false, SIGMA_CMD_QUANTUM,
        "Run quantum circuit simulations"
    };
    
    g_command_library->commands[g_command_library->command_count++] = (SigmaCommand){
        "sigma_optimize", "Quantum Optimization",
        "sigma_optimize [problem] [algorithm] [constraints]",
        "sigma_optimize logistics qaoa --nodes=1000 --edges=5000 --annealing=true",
        "SigmaOS", true, false, SIGMA_CMD_QUANTUM,
        "Quantum optimization algorithms"
    };
    
    printf("[Command Library] Loaded %d quantum commands\n", 4);
}

// Helper function to print loaded commands summary
void sigma_print_command_summary(void) {
    if (!g_command_library) return;
    
    printf("\n========================================\n");
    printf("SigmaOS Command Library Summary\n");
    printf("========================================\n");
    printf("Total Commands: %d\n", g_command_library->command_count);
    printf("\nCategories:\n");
    printf("  - System: Package, file, network, process management\n");
    printf("  - Automation: Workflows, tasks, triggers, pipelines\n");
    printf("  - Customization: Themes, appearance, desktop environment\n");
    printf("  - Personalization: AI assistant, learning, context\n");
    printf("  - Data Science: Processing, ETL, statistics, big data\n");
    printf("  - Machine Learning: Training, prediction, deep learning\n");
    printf("  - Visualization: Charts, 3D, VR/AR, dashboards\n");
    printf("  - Camera: Photo, video, filters, effects, streaming\n");
    printf("  - Setup: Installation, configuration, migration\n");
    printf("  - Security: Encryption, vault, scanning, firewall\n");
    printf("  - Quantum: Quantum computing, encryption, simulation\n");
    printf("\nLinux Distros Covered:\n");
    printf("  - Ubuntu, Debian, Fedora, RHEL, CentOS\n");
    printf("  - Arch, Gentoo, openSUSE, Alpine, Void, NixOS\n");
    printf("========================================\n\n");
}
