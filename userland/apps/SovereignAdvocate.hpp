#pragma once
#include <windows.h>
#include <string>
#include <vector>
#include <memory>

/**
 * SIGMA OS: SOVEREIGN ADVOCATE COMMAND CENTER (WIN32 NATIVE ZENITH)
 * ===============================================================
 * Principles: OOPS, SOLID, Raw Win32 (Zero Library Interference).
 * USP: Bare-Metal Performance crushing Python/Electron based SaaS.
 */

namespace SigmaOS::Legal {

    // --- Abstraction: IView ---
    class IView {
    public:
        virtual ~IView() = default;
        virtual void Render(HWND hwnd) = 0;
        virtual const std::string& GetTitle() const = 0;
    };

    // --- Concrete View: Dashboard ---
    class DashboardView : public IView {
    private:
        std::string m_title = "LITIGATION PULSE (SOVEREIGN)";
    public:
        void Render(HWND hwnd) override {
            // Draw stats, metrics, and case pulses via GDI/Direct2D later
            HDC hdc = GetDC(hwnd);
            TextOutA(hdc, 50, 50, "SOVEREIGN ADVOCATE: ZENITH ACTIVE", 33);
            ReleaseDC(hwnd, hdc);
        }
        const std::string& GetTitle() const override { return m_title; }
    };

    // --- The Advocate Application (OOPS / Encapsulation) ---
    class AdvocateApp {
    private:
        HWND m_hwnd;
        std::unique_ptr<IView> m_active_view;
        std::string m_user_name = "ADVOCATE SOVEREIGN_USER";

    public:
        AdvocateApp() : m_hwnd(nullptr) {
            m_active_view = std::make_unique<DashboardView>();
        }

        static LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam);

        void Run();
        void Initialize(HINSTANCE hInstance);
    };

} // namespace SigmaOS::Legal
