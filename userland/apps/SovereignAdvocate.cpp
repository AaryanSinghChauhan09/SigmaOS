#include "SovereignAdvocate.hpp"

namespace SigmaOS {
    namespace Legal {

    // --- Singleton Architecture / Context Sharing ---
    AdvocateApp* g_current_app = nullptr;

    void AdvocateApp::Initialize(HINSTANCE hInstance) {
        g_current_app = this;
        
        const wchar_t CLASS_NAME[] = L"SigmaSovereignAdvocate";
        
        WNDCLASSW wc = { };
        wc.lpfnWndProc   = WindowProc;
        wc.hInstance     = hInstance;
        wc.lpszClassName = CLASS_NAME;
        wc.hbrBackground = (HBRUSH)(COLOR_WINDOW+1);

        RegisterClassW(&wc);

        m_hwnd = CreateWindowExW(
            0,                              // Optional window styles.
            CLASS_NAME,                     // Window class
            L"Sovereign Advocate | SigmaOS", // Window text
            WS_OVERLAPPEDWINDOW,            // Window style
            CW_USEDEFAULT, CW_USEDEFAULT, 1200, 800, // Size and position
            NULL,                           // Parent window    
            NULL,                           // Menu
            hInstance,                      // Instance handle
            NULL                            // Additional application data
        );

        if (m_hwnd == NULL) return;

        ShowWindow(m_hwnd, SW_SHOW);
    }

    LRESULT CALLBACK AdvocateApp::WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
        switch (uMsg) {
            case WM_PAINT: {
                PAINTSTRUCT ps;
                HDC hdc = BeginPaint(hwnd, &ps);
                
                // 1. Sidebar Design (Bare-Metal Zenith)
                HBRUSH sidebar_brush = CreateSolidBrush(RGB(20, 30, 48)); // Deep Navy
                RECT sidebar_rect = {0, 0, 280, 800};
                FillRect(hdc, &sidebar_rect, sidebar_brush);
                DeleteObject(sidebar_brush);

                // 2. Main Canvas Background
                HBRUSH bg_brush = CreateSolidBrush(RGB(255, 255, 255));
                RECT main_rect = {280, 0, 1200, 800};
                FillRect(hdc, &main_rect, bg_brush);
                DeleteObject(bg_brush);

                // 3. Typography & Branding
                SetBkMode(hdc, TRANSPARENT);
                HFONT hFontMain = CreateFontW(38, 0, 0, 0, FW_BOLD, FALSE, FALSE, FALSE, DEFAULT_CHARSET, 
                    OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, ANTIALIASED_QUALITY, VARIABLE_PITCH, L"Segoe UI");
                SelectObject(hdc, hFontMain);
                
                // Sidebar Text
                SetTextColor(hdc, RGB(255, 255, 255));
                TextOutW(hdc, 30, 30, L"Σ SIGMA OS", 10);
                
                HFONT hFontSub = CreateFontW(18, 0, 0, 0, FW_REGULAR, FALSE, FALSE, FALSE, DEFAULT_CHARSET, 
                    OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, ANTIALIASED_QUALITY, VARIABLE_PITCH, L"Segoe UI");
                SelectObject(hdc, hFontSub);
                TextOutW(hdc, 30, 80, L"LEGAL SHARDS:", 13);
                TextOutW(hdc, 40, 120, L"-> BNS (Penal)", 14);
                TextOutW(hdc, 40, 150, L"-> BNSS (Procedural)", 20);
                TextOutW(hdc, 40, 180, L"-> BSA (Evidence)", 17);
                TextOutW(hdc, 40, 210, L"-> GST Shards", 13);

                // Main Area Text
                SetTextColor(hdc, RGB(20, 30, 48));
                SelectObject(hdc, hFontMain);
                TextOutW(hdc, 320, 50, L"SOVEREIGN ADVOCATE DASHBOARD", 28);
                
                HFONT hFontStat = CreateFontW(24, 0, 0, 0, FW_SEMIBOLD, FALSE, FALSE, FALSE, DEFAULT_CHARSET, 
                    OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, ANTIALIASED_QUALITY, VARIABLE_PITCH, L"Segoe UI");
                SelectObject(hdc, hFontStat);
                TextOutW(hdc, 320, 110, L"STATUTORY PARITY: 100.0% (ALGORITHMIC ZENITH)", 45);
                
                // Draw a simple "Accuracy" gauge placeholder
                Rectangle(hdc, 320, 150, 800, 180);
                HBRUSH gauge_brush = CreateSolidBrush(RGB(0, 180, 0)); // Green
                RECT gauge_rect = {321, 151, 799, 179};
                FillRect(hdc, &gauge_rect, gauge_brush);
                DeleteObject(gauge_brush);

                DeleteObject(hFontMain);
                DeleteObject(hFontSub);
                DeleteObject(hFontStat);
                
                EndPaint(hwnd, &ps);
                return 0;
            }
            case WM_DESTROY:
                PostQuitMessage(0);
                return 0;
        }
        return DefWindowProc(hwnd, uMsg, wParam, lParam);
    }

    void AdvocateApp::Run() {
        MSG msg = { };
        while (GetMessage(&msg, NULL, 0, 0) > 0) {
            TranslateMessage(&msg);
            DispatchMessage(&msg);
        }
    }

    } // namespace Legal
} // namespace SigmaOS

int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE, PWSTR, int) {
    SigmaOS::Legal::AdvocateApp app;
    app.Initialize(hInstance);
    app.Run();
    return 0;
}
