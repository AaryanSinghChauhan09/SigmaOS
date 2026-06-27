// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
// sigma_welcome.h — First-boot onboarding wizard interface
#include <sigma_kernel_types.h>

namespace sigma::installer {

struct WelcomeState;

class WelcomeWizard {
public:
    static void render_screen_1(WelcomeState *s, char *html_out, size_t max);
    static void render_screen_2(WelcomeState *s, char *html_out, size_t max);
    static void render_screen_3(WelcomeState *s, char *html_out, size_t max);
    static void render_screen_5(WelcomeState *s, char *html_out, size_t max);
    static void render_screen_6(WelcomeState *s, char *html_out, size_t max);
    static int  install_profession_bundle(const char *profession_id);
};

} // namespace sigma::installer
