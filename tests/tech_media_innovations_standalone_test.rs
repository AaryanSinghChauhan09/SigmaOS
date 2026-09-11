use sigmaos::ai::marktechpost_kdnuggets_data_science::*;
use sigmaos::distro::tech_media_distro_innovations::*;
use sigmaos::hardware::tech_powerup_hardware_monitors::*;

fn main() {
    println!("=== Testing Tech Media Inspired Innovations ===");

    let mut distro_suite = SovereignTechMediaDistroInnovationsSuite::new();
    assert!(distro_suite.verify_suite(), "Distro suite verification failed");
    println!("DistroWatch top rank: {}", distro_suite.rank_tracker.get_top_ranked_distro());

    let hw_suite = SovereignTechPowerUpHardwareMonitorsSuite::new();
    assert!(hw_suite.verify_suite(), "Hardware suite verification failed");
    println!("GPU Profiler status optimal: {}", hw_suite.gpu_profiler.is_optimal_performance());

    let mut ai_suite = SovereignAiDataSciencePipelineSuite::new();
    assert!(ai_suite.verify_suite(), "AI suite verification failed");
    println!("AutoML best model: {}", ai_suite.automl.best_model_type);

    println!("All tech media innovations tests passed successfully!");
}
