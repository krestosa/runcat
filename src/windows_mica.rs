    use std::sync::atomic::{AtomicBool, Ordering};

    const MICA_WM_PAINT: Uint = 0x000F;
    const MICA_WM_SHOWWINDOW: Uint = 0x0018;
    const MICA_WM_DRAWITEM: Uint = 0x002B;
    const MICA_WM_SETFOCUS: Uint = 0x0007;
    const MICA_WM_KILLFOCUS: Uint = 0x0008;
    const MICA_WM_KEYUP: Uint = 0x0101;
    const MICA_WM_DWMCOMPOSITIONCHANGED: Uint = 0x031E;

    const MICA_DWMWA_WINDOW_CORNER_PREFERENCE: Dword = 33;
    const MICA_DWMWA_SYSTEMBACKDROP_TYPE: Dword = 38;
    const MICA_DWMWCP_ROUND: Dword = 2;
    const MICA_DWMSBT_MAINWINDOW: Dword = 2;

    const MICA_PS_SOLID: i32 = 0;
    const MICA_BLACK_BRUSH: i32 = 4;
    const MICA_HOLLOW_BRUSH: i32 = 5;
    const MICA_GW_CHILD: Uint = 5;
    const MICA_GW_HWNDNEXT: Uint = 2;
    const MICA_GWLP_USERDATA: i32 = -21;
    const MICA_VK_SPACE: Wparam = 0x20;
    const MICA_BS_OWNERDRAW: Dword = 0x0000_000B;
    const MICA_SS_NOPREFIX: Dword = 0x0000_0080;
    const MICA_SWP_NOMOVE: Uint = 0x0002;
    const MICA_SWP_NOZORDER: Uint = 0x0004;
    const MICA_SWP_NOACTIVATE: Uint = 0x0010;
    const MICA_ODS_SELECTED: Uint = 0x0001;
    const MICA_TRANSPARENT: i32 = 1;
    const MICA_OPAQUE: i32 = 2;

    const MICA_DT_LEFT: Uint = 0x0000;
    const MICA_DT_CENTER: Uint = 0x0001;
    const MICA_DT_VCENTER: Uint = 0x0004;
    const MICA_DT_SINGLELINE: Uint = 0x0020;
    const MICA_DT_NOPREFIX: Uint = 0x0800;
    const MICA_DT_END_ELLIPSIS: Uint = 0x8000;

    const MICA_MODERN_MAGIC: isize = 0x4341_5443;
    const MICA_SETTINGS_WIDTH: i32 = 780;
    const MICA_SETTINGS_HEIGHT: i32 = 690;
    const MICA_LEFT_X: i32 = 24;
    const MICA_RIGHT_X: i32 = 400;
    const MICA_COLUMN_WIDTH: i32 = 356;
    const MICA_ROW_TOP: i32 = 190;
    const MICA_ROW_HEIGHT: i32 = 40;
    const MICA_ROW_STEP: i32 = 46;

    static MICA_ACTIVE: AtomicBool = AtomicBool::new(false);
    static MICA_TOGGLE_CLASS_READY: AtomicBool = AtomicBool::new(false);

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

    #[repr(C)]
    struct MicaDrawItemStruct {
        control_type: Uint,
        control_id: Uint,
        item_id: Uint,
        item_action: Uint,
        item_state: Uint,
        hwnd_item: Hwnd,
        dc: Hdc,
        rect: WorkRect,
        item_data: UlongPtr,
    }

    #[link(name = "dwmapi")]
    extern "system" {
        fn DwmExtendFrameIntoClientArea(hwnd: Hwnd, margins: *const MicaMargins) -> i32;
        fn DwmGetColorizationColor(color: *mut Dword, opaque: *mut Bool) -> i32;
    }

    #[link(name = "user32")]
    extern "system" {
        fn BeginPaint(hwnd: Hwnd, paint: *mut MicaPaintStruct) -> Hdc;
        fn EndPaint(hwnd: Hwnd, paint: *const MicaPaintStruct) -> Bool;
        fn GetWindow(hwnd: Hwnd, command: Uint) -> Hwnd;
        fn GetWindowLongPtrW(hwnd: Hwnd, index: i32) -> isize;
        fn SetWindowLongPtrW(hwnd: Hwnd, index: i32, value: isize) -> isize;
        fn GetFocus() -> Hwnd;
        fn SetFocus(hwnd: Hwnd) -> Hwnd;
        fn DrawTextW(dc: Hdc, text: *const u16, count: i32, rect: *mut WorkRect, format: Uint) -> i32;
    }

    #[link(name = "uxtheme")]
    extern "system" {
        fn DrawThemeParentBackground(hwnd: Hwnd, dc: Hdc, rect: *const WorkRect) -> i32;
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
        fn Ellipse(dc: Hdc, left: i32, top: i32, right: i32, bottom: i32) -> Bool;
        fn GetStockObject(index: i32) -> isize;
    }

    fn mica_accent_color() -> ColorRef {
        let mut value = 0u32;
        let mut opaque = 0;
        if unsafe { DwmGetColorizationColor(&mut value, &mut opaque) } == 0 {
            let red = ((value >> 16) & 0xff) as u8;
            let green = ((value >> 8) & 0xff) as u8;
            let blue = (value & 0xff) as u8;
            if red != 0 || green != 0 || blue != 0 {
                return rgb(red, green, blue);
            }
        }
        rgb(0, 120, 212)
    }

    fn mica_text_colors(light: bool) -> (ColorRef, ColorRef) {
        if light {
            (rgb(28, 28, 28), rgb(96, 96, 96))
        } else {
            (rgb(250, 250, 250), rgb(186, 186, 186))
        }
    }

    fn mica_border_color(light: bool) -> ColorRef {
        if light {
            rgb(210, 210, 210)
        } else {
            rgb(72, 72, 72)
        }
    }

    fn mica_button_surface(light: bool, pressed: bool) -> ColorRef {
        match (light, pressed) {
            (true, false) => rgb(250, 250, 250),
            (true, true) => rgb(232, 232, 232),
            (false, false) => rgb(52, 52, 52),
            (false, true) => rgb(68, 68, 68),
        }
    }

    fn mica_get_text(hwnd: Hwnd) -> String {
        unsafe {
            let len = GetWindowTextLengthW(hwnd);
            if len <= 0 {
                return String::new();
            }
            let mut buffer = vec![0u16; len as usize + 1];
            let count = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
            String::from_utf16_lossy(&buffer[..count.max(0) as usize])
        }
    }

    unsafe fn mica_draw_text(
        dc: Hdc,
        text: &str,
        mut rect: WorkRect,
        font: Hfont,
        color: ColorRef,
        flags: Uint,
    ) {
        let old_font = if font != 0 {
            SelectObject(dc, font)
        } else {
            0
        };
        SetTextColor(dc, color);
        SetBkMode(dc, MICA_TRANSPARENT);
        let text = wide(text);
        DrawTextW(dc, text.as_ptr(), -1, &mut rect, flags | MICA_DT_NOPREFIX);
        if old_font != 0 {
            SelectObject(dc, old_font);
        }
    }

    unsafe fn mica_round_rect(
        dc: Hdc,
        rect: WorkRect,
        radius: i32,
        fill: Option<ColorRef>,
        border: ColorRef,
    ) {
        let pen = CreatePen(MICA_PS_SOLID, 1, border);
        let brush = match fill {
            Some(color) => CreateSolidBrush(color),
            None => 0,
        };
        let old_pen = if pen != 0 { SelectObject(dc, pen) } else { 0 };
        let old_brush = if brush != 0 {
            SelectObject(dc, brush)
        } else {
            SelectObject(dc, GetStockObject(MICA_HOLLOW_BRUSH))
        };

        RoundRect(
            dc,
            rect.left,
            rect.top,
            rect.right,
            rect.bottom,
            radius,
            radius,
        );

        if old_brush != 0 {
            SelectObject(dc, old_brush);
        }
        if old_pen != 0 {
            SelectObject(dc, old_pen);
        }
        if brush != 0 {
            DeleteObject(brush);
        }
        if pen != 0 {
            DeleteObject(pen);
        }
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

    unsafe fn mica_toggle_checked(hwnd: Hwnd) -> bool {
        GetWindowLongPtrW(hwnd, MICA_GWLP_USERDATA) != 0
    }

    unsafe fn mica_set_toggle_checked(hwnd: Hwnd, checked: bool) {
        SetWindowLongPtrW(hwnd, MICA_GWLP_USERDATA, if checked { 1 } else { 0 });
        InvalidateRect(hwnd, null(), 1);
    }

    unsafe fn mica_toggle_paint(hwnd: Hwnd) {
        let mut paint: MicaPaintStruct = zeroed();
        let dc = BeginPaint(hwnd, &mut paint);
        if dc == 0 {
            return;
        }

        let mut rect: WorkRect = zeroed();
        GetClientRect(hwnd, &mut rect);

        let (light, font, surface_brush) = if let Some(lock) = STATE.get() {
            if let Ok(state) = lock.lock() {
                (state.ui_light_theme, state.ui_font, state.ui_surface_brush)
            } else {
                (system_uses_light_apps(), 0, 0)
            }
        } else {
            (system_uses_light_apps(), 0, 0)
        };
        if surface_brush != 0 {
            FillRect(dc, &rect, surface_brush);
        }
        let (text_color, _) = mica_text_colors(light);
        let checked = mica_toggle_checked(hwnd);
        let accent = mica_accent_color();

        let label = mica_get_text(hwnd);
        let label_rect = WorkRect {
            left: 2,
            top: 0,
            right: rect.right - 58,
            bottom: rect.bottom,
        };
        mica_draw_text(
            dc,
            &label,
            label_rect,
            font,
            text_color,
            MICA_DT_LEFT | MICA_DT_VCENTER | MICA_DT_SINGLELINE | MICA_DT_END_ELLIPSIS,
        );

        let track_width = 40;
        let track_height = 20;
        let track_left = rect.right - track_width - 2;
        let track_top = (rect.bottom - track_height) / 2;
        let track_rect = WorkRect {
            left: track_left,
            top: track_top,
            right: track_left + track_width,
            bottom: track_top + track_height,
        };
        let track_color = if checked {
            accent
        } else if light {
            rgb(118, 118, 118)
        } else {
            rgb(105, 105, 105)
        };
        mica_round_rect(dc, track_rect, track_height, Some(track_color), track_color);

        let thumb_size = 16;
        let thumb_left = if checked {
            track_rect.right - thumb_size - 2
        } else {
            track_rect.left + 2
        };
        let thumb_top = track_rect.top + 2;
        let thumb_color = if checked || !light {
            rgb(255, 255, 255)
        } else {
            rgb(245, 245, 245)
        };
        let thumb_brush = CreateSolidBrush(thumb_color);
        let thumb_pen = CreatePen(MICA_PS_SOLID, 1, thumb_color);
        let old_brush = if thumb_brush != 0 { SelectObject(dc, thumb_brush) } else { 0 };
        let old_pen = if thumb_pen != 0 { SelectObject(dc, thumb_pen) } else { 0 };
        Ellipse(
            dc,
            thumb_left,
            thumb_top,
            thumb_left + thumb_size,
            thumb_top + thumb_size,
        );
        if old_brush != 0 { SelectObject(dc, old_brush); }
        if old_pen != 0 { SelectObject(dc, old_pen); }
        if thumb_brush != 0 { DeleteObject(thumb_brush); }
        if thumb_pen != 0 { DeleteObject(thumb_pen); }

        if GetFocus() == hwnd {
            let focus = WorkRect {
                left: 0,
                top: 0,
                right: rect.right,
                bottom: rect.bottom,
            };
            mica_round_rect(dc, focus, scale_px(6, 96), None, accent);
        }

        EndPaint(hwnd, &paint);
    }

    unsafe extern "system" fn mica_toggle_wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        match msg {
            BM_GETCHECK => {
                return if mica_toggle_checked(hwnd) {
                    BST_CHECKED as Lresult
                } else {
                    0
                };
            }
            BM_SETCHECK => {
                mica_set_toggle_checked(hwnd, w_param as usize == BST_CHECKED);
                return 0;
            }
            WM_LBUTTONUP => {
                SetFocus(hwnd);
                mica_set_toggle_checked(hwnd, !mica_toggle_checked(hwnd));
                return 0;
            }
            MICA_WM_KEYUP if w_param == MICA_VK_SPACE => {
                mica_set_toggle_checked(hwnd, !mica_toggle_checked(hwnd));
                return 0;
            }
            MICA_WM_SETFOCUS | MICA_WM_KILLFOCUS => {
                InvalidateRect(hwnd, null(), 0);
                return 0;
            }
            MICA_WM_PAINT => {
                mica_toggle_paint(hwnd);
                return 0;
            }
            WM_ERASEBKGND => return 1,
            _ => {}
        }
        DefWindowProcW(hwnd, msg, w_param, l_param)
    }

    unsafe fn mica_register_toggle_class() -> bool {
        if MICA_TOGGLE_CLASS_READY.load(Ordering::Acquire) {
            return true;
        }
        let class_name = wide("CatCPU.WinUIToggle");
        let class = WndClassW {
            style: 0,
            wnd_proc: Some(mica_toggle_wnd_proc),
            cls_extra: 0,
            wnd_extra: 0,
            instance: GetModuleHandleW(null()),
            icon: 0,
            cursor: 0,
            background: 0,
            menu_name: null(),
            class_name: class_name.as_ptr(),
        };
        if RegisterClassW(&class) == 0 {
            return false;
        }
        MICA_TOGGLE_CLASS_READY.store(true, Ordering::Release);
        true
    }

    unsafe fn mica_create_toggle(
        parent: Hwnd,
        text: &str,
        id: usize,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        font: Hfont,
        dpi: u32,
    ) -> Hwnd {
        if !mica_register_toggle_class() {
            return 0;
        }
        let class_name = wide("CatCPU.WinUIToggle");
        let text = wide(text);
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            text.as_ptr(),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            scale_px(x, dpi),
            scale_px(y, dpi),
            scale_px(width, dpi),
            scale_px(height, dpi),
            parent,
            id as Hmenu,
            GetModuleHandleW(null()),
            null_mut(),
        );
        if hwnd != 0 && font != 0 {
            SendMessageW(hwnd, WM_SETFONT, font as Wparam, 1);
        }
        hwnd
    }

    unsafe fn mica_destroy_children(parent: Hwnd) {
        let mut child = GetWindow(parent, MICA_GW_CHILD);
        while child != 0 {
            let next = GetWindow(child, MICA_GW_HWNDNEXT);
            DestroyWindow(child);
            child = next;
        }
    }

    unsafe fn mica_create_modern_controls(hwnd: Hwnd) {
        if GetWindowLongPtrW(hwnd, MICA_GWLP_USERDATA) == MICA_MODERN_MAGIC {
            return;
        }

        mica_destroy_children(hwnd);
        let (dpi, font, _header) = prepare_ui_resources(hwnd);
        SetWindowPos(
            hwnd,
            0,
            0,
            0,
            scale_px(MICA_SETTINGS_WIDTH, dpi),
            scale_px(MICA_SETTINGS_HEIGHT, dpi),
            MICA_SWP_NOMOVE | MICA_SWP_NOZORDER | MICA_SWP_NOACTIVATE,
        );

        let label = WS_CHILD | WS_VISIBLE | MICA_SS_NOPREFIX;
        let edit = WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_BORDER | ES_AUTOHSCROLL;

        create_control(
            hwnd,
            "STATIC",
            "",
            label | SS_CENTERIMAGE,
            38,
            94,
            704,
            36,
            CFG_STATUS,
            font,
            dpi,
        );

        let theme = create_control(
            hwnd,
            "COMBOBOX",
            "",
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | CBS_DROPDOWNLIST,
            202,
            196,
            146,
            132,
            CFG_THEME,
            font,
            dpi,
        );
        add_combo_items(theme, &["Automatic", "Light / black cat", "Dark / white cat"]);
        mica_create_toggle(hwnd, "Start with Windows", CFG_STARTUP, 36, 242, 320, 28, font, dpi);

        create_control(hwnd, "EDIT", "", edit, 278, 288, 70, 26, CFG_SPEED, font, dpi);

        let curve = create_control(
            hwnd,
            "COMBOBOX",
            "",
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | CBS_DROPDOWNLIST,
            202,
            334,
            146,
            120,
            CFG_CURVE,
            font,
            dpi,
        );
        add_combo_items(curve, &["Smooth", "Linear", "Reactive"]);

        create_control(hwnd, "EDIT", "", edit, 278, 380, 70, 26, CFG_SIZE, font, dpi);
        mica_create_toggle(hwnd, "Smooth speed transitions", CFG_SMOOTH, 36, 426, 320, 28, font, dpi);
        mica_create_toggle(hwnd, "Invert CPU / speed", CFG_INVERT, 36, 472, 320, 28, font, dpi);
        mica_create_toggle(hwnd, "Pause animation", CFG_PAUSE, 36, 518, 320, 28, font, dpi);

        create_control(hwnd, "EDIT", "", edit, 654, 196, 70, 26, CFG_THRESHOLD, font, dpi);
        create_control(hwnd, "EDIT", "", edit, 654, 242, 70, 26, CFG_HYSTERESIS, font, dpi);
        create_control(hwnd, "EDIT", "", edit, 654, 288, 70, 26, CFG_SAMPLE, font, dpi);
        mica_create_toggle(hwnd, "Sleeping cat when idle", CFG_SLEEP, 412, 334, 320, 28, font, dpi);
        mica_create_toggle(hwnd, "Pause animation on battery", CFG_BATTERY_PAUSE, 412, 380, 320, 28, font, dpi);
        mica_create_toggle(hwnd, "Tooltip: CPU", CFG_TOOLTIP_CPU, 412, 426, 320, 28, font, dpi);
        mica_create_toggle(hwnd, "Tooltip: RAM", CFG_TOOLTIP_RAM, 412, 472, 320, 28, font, dpi);
        mica_create_toggle(hwnd, "Tooltip: battery", CFG_TOOLTIP_BATTERY, 412, 518, 320, 28, font, dpi);
        mica_create_toggle(hwnd, "Large overlay for >32 px", CFG_OVERLAY, 412, 564, 320, 28, font, dpi);

        create_control(
            hwnd,
            "BUTTON",
            "Apply",
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | MICA_BS_OWNERDRAW,
            476,
            622,
            86,
            36,
            CFG_APPLY,
            font,
            dpi,
        );
        create_control(
            hwnd,
            "BUTTON",
            "Reset",
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | MICA_BS_OWNERDRAW,
            572,
            622,
            80,
            36,
            CFG_RESET,
            font,
            dpi,
        );
        create_control(
            hwnd,
            "BUTTON",
            "Close",
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | MICA_BS_OWNERDRAW,
            662,
            622,
            82,
            36,
            CFG_CLOSE,
            font,
            dpi,
        );

        SetWindowLongPtrW(hwnd, MICA_GWLP_USERDATA, MICA_MODERN_MAGIC);
        apply_settings_theme(hwnd);
        sync_settings_window();
        InvalidateRect(hwnd, null(), 1);
    }

    unsafe fn mica_draw_action_button(draw: &MicaDrawItemStruct) {
        if draw.dc == 0 || draw.hwnd_item == 0 {
            return;
        }

        let _ = DrawThemeParentBackground(draw.hwnd_item, draw.dc, &draw.rect);
        let (light, font) = if let Some(lock) = STATE.get() {
            if let Ok(state) = lock.lock() {
                (state.ui_light_theme, state.ui_font)
            } else {
                (system_uses_light_apps(), 0)
            }
        } else {
            (system_uses_light_apps(), 0)
        };

        let pressed = draw.item_state & MICA_ODS_SELECTED != 0;
        let accent = mica_accent_color();
        let is_primary = draw.control_id as usize == CFG_APPLY;
        let fill = if is_primary {
            accent
        } else {
            mica_button_surface(light, pressed)
        };
        let border = if is_primary { accent } else { mica_border_color(light) };
        mica_round_rect(draw.dc, draw.rect, scale_px(8, 96), Some(fill), border);

        let text_color = if is_primary {
            rgb(255, 255, 255)
        } else {
            mica_text_colors(light).0
        };
        let label = mica_get_text(draw.hwnd_item);
        mica_draw_text(
            draw.dc,
            &label,
            draw.rect,
            font,
            text_color,
            MICA_DT_CENTER | MICA_DT_VCENTER | MICA_DT_SINGLELINE,
        );
    }

    unsafe fn mica_control_color(
        parent: Hwnd,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        let Some(lock) = STATE.get() else {
            return 0;
        };
        let Ok(state) = lock.lock() else {
            return 0;
        };
        let light = state.ui_light_theme;
        let surface = state.ui_surface_brush;
        let text = mica_text_colors(light).0;
        drop(state);

        let dc = w_param as Hdc;
        let child = l_param as Hwnd;
        let status = GetDlgItem(parent, CFG_STATUS as i32);
        SetTextColor(dc, text);
        if child == status && surface != 0 {
            let color = if light { rgb(255, 255, 255) } else { rgb(45, 45, 45) };
            SetBkMode(dc, MICA_OPAQUE);
            SetBkColor(dc, color);
            surface as Lresult
        } else {
            SetBkMode(dc, MICA_TRANSPARENT);
            GetStockObject(MICA_HOLLOW_BRUSH) as Lresult
        }
    }

    unsafe fn mica_paint_rows(
        dc: Hdc,
        dpi: u32,
        light: bool,
        font: Hfont,
        header: Hfont,
    ) {
        let border = mica_border_color(light);
        let (text, secondary) = mica_text_colors(light);
        let radius = scale_px(8, dpi);

        let title_font = create_ui_font(dpi, FW_SEMIBOLD, 24);
        let small_font = create_ui_font(dpi, FW_NORMAL, 11);

        mica_draw_text(
            dc,
            "CatCPU",
            WorkRect {
                left: scale_px(24, dpi),
                top: scale_px(16, dpi),
                right: scale_px(420, dpi),
                bottom: scale_px(48, dpi),
            },
            title_font,
            text,
            MICA_DT_LEFT | MICA_DT_VCENTER | MICA_DT_SINGLELINE,
        );
        mica_draw_text(
            dc,
            "Settings",
            WorkRect {
                left: scale_px(24, dpi),
                top: scale_px(48, dpi),
                right: scale_px(420, dpi),
                bottom: scale_px(70, dpi),
            },
            font,
            secondary,
            MICA_DT_LEFT | MICA_DT_VCENTER | MICA_DT_SINGLELINE,
        );

        mica_draw_text(
            dc,
            "Appearance & animation",
            WorkRect {
                left: scale_px(MICA_LEFT_X, dpi),
                top: scale_px(154, dpi),
                right: scale_px(MICA_LEFT_X + MICA_COLUMN_WIDTH, dpi),
                bottom: scale_px(180, dpi),
            },
            header,
            text,
            MICA_DT_LEFT | MICA_DT_VCENTER | MICA_DT_SINGLELINE,
        );
        mica_draw_text(
            dc,
            "Idle, power & tray",
            WorkRect {
                left: scale_px(MICA_RIGHT_X, dpi),
                top: scale_px(154, dpi),
                right: scale_px(MICA_RIGHT_X + MICA_COLUMN_WIDTH, dpi),
                bottom: scale_px(180, dpi),
            },
            header,
            text,
            MICA_DT_LEFT | MICA_DT_VCENTER | MICA_DT_SINGLELINE,
        );

        for index in 0..8 {
            let top = MICA_ROW_TOP + index * MICA_ROW_STEP;
            let rect = WorkRect {
                left: scale_px(MICA_LEFT_X, dpi),
                top: scale_px(top, dpi),
                right: scale_px(MICA_LEFT_X + MICA_COLUMN_WIDTH, dpi),
                bottom: scale_px(top + MICA_ROW_HEIGHT, dpi),
            };
            let fill = if light { rgb(255, 255, 255) } else { rgb(45, 45, 45) };
            mica_round_rect(dc, rect, radius, Some(fill), border);
        }
        for index in 0..9 {
            let top = MICA_ROW_TOP + index * MICA_ROW_STEP;
            let rect = WorkRect {
                left: scale_px(MICA_RIGHT_X, dpi),
                top: scale_px(top, dpi),
                right: scale_px(MICA_RIGHT_X + MICA_COLUMN_WIDTH, dpi),
                bottom: scale_px(top + MICA_ROW_HEIGHT, dpi),
            };
            let fill = if light { rgb(255, 255, 255) } else { rgb(45, 45, 45) };
            mica_round_rect(dc, rect, radius, Some(fill), border);
        }

        let labels = [
            (MICA_LEFT_X, MICA_ROW_TOP, "Cat theme", "Follow Windows or override cat contrast"),
            (MICA_LEFT_X, MICA_ROW_TOP + 2 * MICA_ROW_STEP, "Speed multiplier", "0.10–5.00×"),
            (MICA_LEFT_X, MICA_ROW_TOP + 3 * MICA_ROW_STEP, "Speed curve", "CPU response profile"),
            (MICA_LEFT_X, MICA_ROW_TOP + 4 * MICA_ROW_STEP, "Cat size", "12–64 px"),
            (MICA_RIGHT_X, MICA_ROW_TOP, "Sleep threshold", "0–100% CPU"),
            (MICA_RIGHT_X, MICA_ROW_TOP + MICA_ROW_STEP, "Wake hysteresis", "0–25 percentage points"),
            (MICA_RIGHT_X, MICA_ROW_TOP + 2 * MICA_ROW_STEP, "CPU sampling", "250–5000 ms"),
        ];
        for (x, y, label, hint) in labels {
            mica_draw_text(
                dc,
                label,
                WorkRect {
                    left: scale_px(x + 14, dpi),
                    top: scale_px(y + 3, dpi),
                    right: scale_px(x + 190, dpi),
                    bottom: scale_px(y + 22, dpi),
                },
                font,
                text,
                MICA_DT_LEFT | MICA_DT_SINGLELINE | MICA_DT_END_ELLIPSIS,
            );
            mica_draw_text(
                dc,
                hint,
                WorkRect {
                    left: scale_px(x + 14, dpi),
                    top: scale_px(y + 20, dpi),
                    right: scale_px(x + 210, dpi),
                    bottom: scale_px(y + 37, dpi),
                },
                small_font,
                secondary,
                MICA_DT_LEFT | MICA_DT_SINGLELINE | MICA_DT_END_ELLIPSIS,
            );
        }

        if title_font != 0 { DeleteObject(title_font); }
        if small_font != 0 { DeleteObject(small_font); }
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
        let background_brush = state.ui_bg_brush;
        let font = state.ui_font;
        let header = state.ui_header_font;
        drop(state);

        let mut paint: MicaPaintStruct = unsafe { zeroed() };
        let dc = unsafe { BeginPaint(hwnd, &mut paint) };
        if dc == 0 {
            return;
        }

        unsafe {
            let mut client: WorkRect = zeroed();
            if GetClientRect(hwnd, &mut client) != 0 {
                if MICA_ACTIVE.load(Ordering::Relaxed) {
                    let black = GetStockObject(MICA_BLACK_BRUSH) as Hbrush;
                    if black != 0 {
                        FillRect(dc, &client, black);
                    }
                } else if background_brush != 0 {
                    FillRect(dc, &client, background_brush);
                }
            }

            let status_rect = WorkRect {
                left: scale_px(24, dpi),
                top: scale_px(88, dpi),
                right: scale_px(756, dpi),
                bottom: scale_px(136, dpi),
            };
            let border = mica_border_color(light);
            let fill = if light { rgb(255, 255, 255) } else { rgb(45, 45, 45) };
            mica_round_rect(dc, status_rect, scale_px(10, dpi), Some(fill), border);

            mica_paint_rows(dc, dpi, light, font, header);
            EndPaint(hwnd, &paint);
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
                    mica_create_modern_controls(hwnd);
                    apply_settings_theme(hwnd);
                    apply_mica_backdrop(hwnd);
                }
                return 0;
            }
            WM_SETTINGCHANGE
            | WM_THEMECHANGED
            | WM_SYSCOLORCHANGE
            | MICA_WM_DWMCOMPOSITIONCHANGED => {
                apply_settings_theme(hwnd);
                apply_mica_backdrop(hwnd);
                return 0;
            }
            WM_ERASEBKGND if MICA_ACTIVE.load(Ordering::Relaxed) => return 1,
            MICA_WM_PAINT => {
                paint_winui_settings(hwnd);
                return 0;
            }
            MICA_WM_DRAWITEM => {
                let draw = l_param as *const MicaDrawItemStruct;
                if !draw.is_null() {
                    mica_draw_action_button(&*draw);
                    return 1;
                }
            }
            WM_CTLCOLORSTATIC | WM_CTLCOLORBTN => {
                return mica_control_color(hwnd, w_param, l_param);
            }
            WM_DESTROY => {
                SetWindowLongPtrW(hwnd, MICA_GWLP_USERDATA, 0);
            }
            _ => {}
        }
        settings_wnd_proc(hwnd, msg, w_param, l_param)
    }
