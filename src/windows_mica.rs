    use std::sync::atomic::{AtomicBool, Ordering};

    const MICA_WM_PAINT: Uint = 0x000F;
    const MICA_WM_SHOWWINDOW: Uint = 0x0018;
    const MICA_WM_DWMCOMPOSITIONCHANGED: Uint = 0x031E;

    const MICA_DWMWA_WINDOW_CORNER_PREFERENCE: Dword = 33;
    const MICA_DWMWA_SYSTEMBACKDROP_TYPE: Dword = 38;
    const MICA_DWMWCP_ROUND: Dword = 2;
    const MICA_DWMSBT_MAINWINDOW: Dword = 2;

    const MICA_PS_SOLID: i32 = 0;
    const MICA_HOLLOW_BRUSH: i32 = 5;

    static MICA_ACTIVE: AtomicBool = AtomicBool::new(false);

    #[repr(C)]
    struct MicaMargins {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    #[repr(C)]
    struct MicaPaintStruct {
        dc: Hdc,
        erase: Bool,
        paint: WorkRect,
        restore: Bool,
        inc_update: Bool,
        reserved: [u8; 32],
    }

    #[link(name = "dwmapi")]
    extern "system" {
        fn DwmExtendFrameIntoClientArea(hwnd: Hwnd, margins: *const MicaMargins) -> i32;
    }

    #[link(name = "user32")]
    extern "system" {
        fn BeginPaint(hwnd: Hwnd, paint: *mut MicaPaintStruct) -> Hdc;
        fn EndPaint(hwnd: Hwnd, paint: *const MicaPaintStruct) -> Bool;
    }

    #[link(name = "gdi32")]
    extern "system" {
        fn CreatePen(style: i32, width: i32, color: ColorRef) -> isize;
        fn SelectObject(dc: Hdc, object: isize) -> isize;
        fn RoundRect(
            dc: Hdc,
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
            width: i32,
            height: i32,
        ) -> Bool;
        fn GetStockObject(index: i32) -> isize;
    }

    fn apply_mica_backdrop(hwnd: Hwnd) {
        let corners = MICA_DWMWCP_ROUND;
        let backdrop = MICA_DWMSBT_MAINWINDOW;
        let backdrop_result = unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd,
                MICA_DWMWA_WINDOW_CORNER_PREFERENCE,
                &corners as *const Dword as *const c_void,
                size_of::<Dword>() as Dword,
            );
            DwmSetWindowAttribute(
                hwnd,
                MICA_DWMWA_SYSTEMBACKDROP_TYPE,
                &backdrop as *const Dword as *const c_void,
                size_of::<Dword>() as Dword,
            )
        };

        let active = backdrop_result == 0;
        let margins = if active {
            MicaMargins {
                left: -1,
                right: -1,
                top: -1,
                bottom: -1,
            }
        } else {
            MicaMargins {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            }
        };
        unsafe {
            let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
            InvalidateRect(hwnd, null(), 1);
        }
        MICA_ACTIVE.store(active, Ordering::Relaxed);
    }

    fn paint_winui_settings(hwnd: Hwnd) {
        let Some(lock) = STATE.get() else {
            return;
        };
        let Ok(state) = lock.lock() else {
            return;
        };
        let dpi = state.ui_dpi.max(96);
        let light = state.ui_light_theme;
        let surface_brush = state.ui_surface_brush;
        let background_brush = state.ui_bg_brush;
        drop(state);

        let mut paint: MicaPaintStruct = unsafe { zeroed() };
        let dc = unsafe { BeginPaint(hwnd, &mut paint) };
        if dc == 0 {
            return;
        }

        unsafe {
            if !MICA_ACTIVE.load(Ordering::Relaxed) && background_brush != 0 {
                let mut client: WorkRect = zeroed();
                if GetClientRect(hwnd, &mut client) != 0 {
                    FillRect(dc, &client, background_brush);
                }
            }

            let border = if light {
                rgb(216, 216, 216)
            } else {
                rgb(66, 66, 66)
            };
            let pen = CreatePen(MICA_PS_SOLID, 1, border);
            let old_pen = if pen != 0 { SelectObject(dc, pen) } else { 0 };
            let old_brush = if surface_brush != 0 {
                SelectObject(dc, surface_brush)
            } else {
                0
            };
            let radius = scale_px(10, dpi);

            for (left, top, right, bottom) in [
                (20, 76, 688, 118),
                (18, 124, 346, 486),
                (362, 124, 690, 486),
            ] {
                RoundRect(
                    dc,
                    scale_px(left, dpi),
                    scale_px(top, dpi),
                    scale_px(right, dpi),
                    scale_px(bottom, dpi),
                    radius,
                    radius,
                );
            }

            if old_brush != 0 {
                SelectObject(dc, old_brush);
            }
            if old_pen != 0 {
                SelectObject(dc, old_pen);
            }
            if pen != 0 {
                DeleteObject(pen);
            }
            EndPaint(hwnd, &paint);
        }
    }

    fn transparent_winui_control(w_param: Wparam) -> Lresult {
        let Some(lock) = STATE.get() else {
            return 0;
        };
        let Ok(state) = lock.lock() else {
            return 0;
        };
        let text = if state.ui_light_theme {
            rgb(31, 31, 31)
        } else {
            rgb(247, 247, 247)
        };
        drop(state);

        unsafe {
            let dc = w_param as Hdc;
            SetTextColor(dc, text);
            SetBkMode(dc, TRANSPARENT);
            GetStockObject(MICA_HOLLOW_BRUSH) as Lresult
        }
    }

    unsafe extern "system" fn mica_settings_wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        match msg {
            MICA_WM_SHOWWINDOW => {
                if w_param != 0 {
                    apply_settings_theme(hwnd);
                    apply_mica_backdrop(hwnd);
                }
                0
            }
            WM_SETTINGCHANGE | WM_THEMECHANGED | WM_SYSCOLORCHANGE | MICA_WM_DWMCOMPOSITIONCHANGED => {
                apply_settings_theme(hwnd);
                apply_mica_backdrop(hwnd);
                0
            }
            WM_ERASEBKGND if MICA_ACTIVE.load(Ordering::Relaxed) => 1,
            MICA_WM_PAINT => {
                paint_winui_settings(hwnd);
                0
            }
            WM_CTLCOLORSTATIC | WM_CTLCOLORBTN => transparent_winui_control(w_param),
            _ => settings_wnd_proc(hwnd, msg, w_param, l_param),
        }
    }
