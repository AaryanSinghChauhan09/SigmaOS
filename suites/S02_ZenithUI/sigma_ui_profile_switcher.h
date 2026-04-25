// SigmaOS — sigma-ui-profile-switcher: Work/Gaming/VR Profiles
// Module: sigma-ui-profile-switcher
// USP: Instantly adjusts system resource allocation and UI compositor settings based on active profile.

#ifndef SIGMA_UI_PROFILE_SWITCHER_H
#define SIGMA_UI_PROFILE_SWITCHER_H

namespace sigma {
namespace ui {

enum class UIProfile {
    WORK,
    GAMING,
    VR,
    MINIMAL
};

struct ProfileSettings {
    bool enable_vsync;
    bool enable_blur;
    bool prioritize_input_latency;
    unsigned int target_fps;
};

class ProfileSwitcher {
private:
    UIProfile active_profile;
    ProfileSettings current_settings;

    void apply_settings() {
        switch (active_profile) {
            case UIProfile::WORK:
                current_settings = { true, true, false, 60 };
                break;
            case UIProfile::GAMING:
                current_settings = { false, false, true, 240 };
                break;
            case UIProfile::VR:
                current_settings = { true, false, true, 120 };
                break;
            case UIProfile::MINIMAL:
                current_settings = { true, false, false, 30 };
                break;
        }
    }

public:
    ProfileSwitcher() : active_profile(UIProfile::WORK) {
        apply_settings();
    }

    void switch_profile(UIProfile new_profile) {
        active_profile = new_profile;
        apply_settings();
    }

    const ProfileSettings& get_settings() const { return current_settings; }
    UIProfile get_active() const { return active_profile; }
};

} // namespace ui
} // namespace sigma

#endif /* SIGMA_UI_PROFILE_SWITCHER_H */
