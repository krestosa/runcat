    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    // This renderer intentionally mirrors WinUI/Windows Community Toolkit metrics:
    // - SettingsCard: 68 epx min-height, 16 epx padding, 4 epx control radius.
    // - ToggleSwitch: 40x20 epx track with a 12 epx resting knob.
    // - Typography: Segoe UI Variable Text for UI copy and Display for page titles.
    const FL_WM_PAINT: Uint = 0x000F;
    const FL_WM_SHOWWINDOW: Uint = 0x0018;
    const FL_WM_DRAWITEM: Uint = 0x002B;
    const FL_WM_MOUSEMOVE: Uint = 0x0200;
    const FL_WM_LBUTTONDOWN: Uint = 0x0201;
    const FL_WM_MOUSELEAVE: Uint = 0x02A3;
    const FL_WM_KEYUP: Uint = 0x0101;
    const FL_WM_DPICHANGED: Uint = 0x02E0;
    const FL_WM_DWMCOMPOSITIONCHANGED: Uint = 0x031E;

    const FL_DWMWA_WINDOW_CORNER_PREFERENCE: Dword = 33;
    const FL_DWMWA_SYSTEMBACKDROP_TYPE: Dword = 38;
    const FL_DWMWCP_ROUND: Dword = 2;
    const FL_DWMSBT_MAINWINDOW: Dword = 2;

    const FL_PS_SOLID: i32 = 0;
    const FL_HOLLOW_BRUSH: i32 = 5;
    const FL_GW_CHILD: Uint = 5;
    const FL_GW_HWNDNEXT: Uint = 2;
    const FL_GWLP_USERDATA: i32 = -21;
    const FL_VK_SPACE: Wparam = 0x20;
    const FL_VK_RETURN: Wparam = 0x0D;
    const FL_VK_LEFT: Wparam = 0x25;
    const FL_VK_UP: Wparam = 0x26;
    const FL_VK_RIGHT: Wparam = 0x27;
    const FL_VK_DOWN: Wparam = 0x28;
    const FL_BS_OWNERDRAW: Dword = 0x0000_000B;
    const FL_SS_NOPREFIX: Dword = 0x0000_0080;
    const FL_SWP_NOMOVE: Uint = 0x0002;
    const FL_SWP_NOZORDER: Uint = 0x0004;
    const FL_SWP_NOACTIVATE: Uint = 0x0010;
    const FL_ODS_SELECTED: Uint = 0x0001;
    const FL_TRANSPARENT: i32 = 1;
    const FL_OPAQUE: i32 = 2;
    const FL_CBN_SELCHANGE: usize = 1;
    const FL_EN_KILLFOCUS: usize = 0x0200;
    const FL_TME_LEAVE: Dword = 0x0000_0002;

    const FL_DT_LEFT: Uint = 0x0000;
    const FL_DT_CENTER: Uint = 0x0001;
    const FL_DT_VCENTER: Uint = 0x0004;
    const FL_DT_SINGLELINE: Uint = 0x0020;
    const FL_DT_NOPREFIX: Uint = 0x0800;
    const FL_DT_END_ELLIPSIS: Uint = 0x8000;

    const FL_READY_MAGIC: isize = 0x4341_5453;
    const FL_SETTINGS_WIDTH: i32 = 920;
    const FL_SETTINGS_HEIGHT: i32 = 740;
    const FL_GUTTER: i32 = 24;
    const FL_NAV_WIDTH: i32 = 220;
    const FL_MAIN_X: i32 = 276;
    const FL_MAIN_WIDTH: i32 = 616;
    const FL_STATUS_TOP: i32 = 72;
    const FL_STATUS_HEIGHT: i32 = 68;
    const FL_SECTION_TOP: i32 = 154;
    const FL_ROW_TOP: i32 = 184;
    const FL_ROW_HEIGHT: i32 = 68;
    const FL_ROW_STEP: i32 = 72;
    const FL_CARD_RADIUS: i32 = 4;
    const FL_CARD_PADDING: i32 = 16;
    const FL_ACTION_MIN_WIDTH: i32 = 120;

    const FL_NAV_APPEARANCE: usize = 3101;
    const FL_NAV_ANIMATION: usize = 3102;
    const FL_NAV_BEHAVIOR: usize = 3103;
    const FL_RESET: usize = 3104;
    const FL_PAGE_APPEARANCE: usize = 0;
    const FL_PAGE_ANIMATION: usize = 1;
    const FL_PAGE_BEHAVIOR: usize = 2;

    const FL_STATE_CHECKED: isize = 1;
    const FL_STATE_HOVER: isize = 1 << 1;
    const FL_STATE_PRESSED: isize = 1 << 2;

    static FL_MICA_ACTIVE: AtomicBool = AtomicBool::new(false);
    static FL_TOGGLE_CLASS_READY: AtomicBool = AtomicBool::new(false);
    static FL_COMBO_CLASS_READY: AtomicBool = AtomicBool::new(false);
    static FL_ACTIVE_PAGE: AtomicUsize = AtomicUsize::new(FL_PAGE_APPEARANCE);

    #[repr(C)]
    struct FluentPaintStruct {
        dc: Hdc,
        erase: Bool,
        paint: WorkRect,
        restore: Bool,
        inc_update: Bool,
        reserved: [u8; 32],
    }

    #[repr(C)]
    struct FluentTrackMouseEvent {
        cb_size: Dword,
        flags: Dword,
        hwnd_track: Hwnd,
        hover_time: Dword,
    }

    #[repr(C)]
    struct FluentDrawItemStruct {
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

    #[derive(Clone, Copy)]
    struct FluentFonts {
        dpi: u32,
        body: Hfont,
        secondary: Hfont,
        section: Hfont,
        title: Hfont,
    }

    #[derive(Clone, Copy)]
    struct FluentPalette {
        base: ColorRef,
        card: ColorRef,
        card_border: ColorRef,
        text: ColorRef,
        secondary: ColorRef,
        control: ColorRef,
        control_border: ColorRef,
        nav_selected: ColorRef,
        toggle_off_fill: ColorRef,
        toggle_off_stroke: ColorRef,
        toggle_knob_off: ColorRef,
        toggle_knob_on: ColorRef,
    }

    #[derive(Clone, Copy)]
    struct FluentBrushes {
        base: Hbrush,
        card: Hbrush,
        control: Hbrush,
    }

    static FL_FONTS: OnceLock<Mutex<FluentFonts>> = OnceLock::new();
    static FL_DARK_BRUSHES: OnceLock<FluentBrushes> = OnceLock::new();
    static FL_LIGHT_BRUSHES: OnceLock<FluentBrushes> = OnceLock::new();

    #[link(name = "dwmapi")]
    extern "system" {
        fn DwmGetColorizationColor(color: *mut Dword, opaque: *mut Bool) -> i32;
    }

    #[link(name = "user32")]
    extern "system" {
        fn BeginPaint(hwnd: Hwnd, paint: *mut FluentPaintStruct) -> Hdc;
        fn EndPaint(hwnd: Hwnd, paint: *const FluentPaintStruct) -> Bool;
        fn GetWindow(hwnd: Hwnd, command: Uint) -> Hwnd;
        fn GetWindowLongPtrW(hwnd: Hwnd, index: i32) -> isize;
        fn SetWindowLongPtrW(hwnd: Hwnd, index: i32, value: isize) -> isize;
        fn GetFocus() -> Hwnd;
        fn SetFocus(hwnd: Hwnd) -> Hwnd;
        fn GetParent(hwnd: Hwnd) -> Hwnd;
        fn GetDlgCtrlID(hwnd: Hwnd) -> i32;
        fn GetWindowRect(hwnd: Hwnd, rect: *mut WorkRect) -> Bool;
        fn DrawTextW(dc: Hdc, text: *const u16, count: i32, rect: *mut WorkRect, format: Uint) -> i32;
        fn TrackMouseEvent(event: *mut FluentTrackMouseEvent) -> Bool;
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

    fn fl_palette(light: bool) -> FluentPalette {
        if light {
            // Pre-composited equivalents of WinUI Light theme resources over
            // SolidBackgroundFillColorBase (#F3F3F3).
            FluentPalette {
                base: rgb(243, 243, 243),
                card: rgb(251, 251, 251),
                card_border: rgb(236, 236, 236),
                text: rgb(27, 27, 27),
                secondary: rgb(96, 96, 96),
                control: rgb(251, 251, 251),
                control_border: rgb(236, 236, 236),
                nav_selected: rgb(229, 229, 229),
                toggle_off_fill: rgb(245, 245, 245),
                toggle_off_stroke: rgb(145, 145, 145),
                toggle_knob_off: rgb(96, 96, 96),
                toggle_knob_on: rgb(255, 255, 255),
            }
        } else {
            // Pre-composited equivalents of WinUI Dark theme resources over
            // SolidBackgroundFillColorBase (#202020).
            FluentPalette {
                base: rgb(32, 32, 32),
                card: rgb(43, 43, 43),
                card_border: rgb(39, 39, 39),
                text: rgb(255, 255, 255),
                secondary: rgb(207, 207, 207),
                control: rgb(56, 56, 56),
                control_border: rgb(70, 70, 70),
                nav_selected: rgb(45, 45, 45),
                toggle_off_fill: rgb(39, 39, 39),
                toggle_off_stroke: rgb(159, 159, 159),
                toggle_knob_off: rgb(207, 207, 207),
                toggle_knob_on: rgb(0, 0, 0),
            }
        }
    }

    fn fl_brushes(light: bool) -> FluentBrushes {
        let target = if light { &FL_LIGHT_BRUSHES } else { &FL_DARK_BRUSHES };
        *target.get_or_init(|| {
            let palette = fl_palette(light);
            unsafe {
                FluentBrushes {
                    base: CreateSolidBrush(palette.base),
                    card: CreateSolidBrush(palette.card),
                    control: CreateSolidBrush(palette.control),
                }
            }
        })
    }

    fn fl_create_font(face_name: &str, dpi: u32, weight: i32, base_px: i32) -> Hfont {
        let face = wide(face_name);
        let fallback = wide("Segoe UI");
        unsafe {
            let font = CreateFontW(
                -scale_px(base_px, dpi), 0, 0, 0, weight, 0, 0, 0,
                DEFAULT_CHARSET, 0, 0, CLEARTYPE_QUALITY, 0, face.as_ptr(),
            );
            if font != 0 {
                font
            } else {
                CreateFontW(
                    -scale_px(base_px, dpi), 0, 0, 0, weight, 0, 0, 0,
                    DEFAULT_CHARSET, 0, 0, CLEARTYPE_QUALITY, 0, fallback.as_ptr(),
                )
            }
        }
    }

    fn fl_fonts(dpi: u32) -> FluentFonts {
        let lock = FL_FONTS.get_or_init(|| {
            Mutex::new(FluentFonts { dpi: 0, body: 0, secondary: 0, section: 0, title: 0 })
        });
        let Ok(mut fonts) = lock.lock() else {
            return FluentFonts { dpi, body: 0, secondary: 0, section: 0, title: 0 };
        };
        if fonts.dpi != dpi || fonts.body == 0 {
            unsafe {
                for object in [fonts.body, fonts.secondary, fonts.section, fonts.title] {
                    if object != 0 { DeleteObject(object); }
                }
            }
            *fonts = FluentFonts {
                dpi,
                body: fl_create_font("Segoe UI Variable Text", dpi, FW_NORMAL, 14),
                secondary: fl_create_font("Segoe UI Variable Text", dpi, FW_NORMAL, 12),
                section: fl_create_font("Segoe UI Variable Text", dpi, FW_SEMIBOLD, 14),
                title: fl_create_font("Segoe UI Variable Display", dpi, FW_SEMIBOLD, 28),
            };
        }
        *fonts
    }

    fn fl_accent_color() -> ColorRef {
        let mut value = 0u32;
        let mut opaque = 0;
        if unsafe { DwmGetColorizationColor(&mut value, &mut opaque) } == 0 {
            let red = ((value >> 16) & 0xff) as u8;
            let green = ((value >> 8) & 0xff) as u8;
            let blue = (value & 0xff) as u8;
            if red != 0 || green != 0 || blue != 0 { return rgb(red, green, blue); }
        }
        rgb(0, 120, 212)
    }

    unsafe fn fl_draw_text(
        dc: Hdc,
        text: &str,
        mut rect: WorkRect,
        font: Hfont,
        color: ColorRef,
        background: ColorRef,
        flags: Uint,
    ) {
        let old_font = if font != 0 { SelectObject(dc, font) } else { 0 };
        SetTextColor(dc, color);
        SetBkMode(dc, FL_OPAQUE);
        SetBkColor(dc, background);
        let text = wide(text);
        DrawTextW(dc, text.as_ptr(), -1, &mut rect, flags | FL_DT_NOPREFIX);
        if old_font != 0 { SelectObject(dc, old_font); }
    }

    unsafe fn fl_round_rect(
        dc: Hdc,
        rect: WorkRect,
        radius: i32,
        fill: ColorRef,
        border: ColorRef,
    ) {
        let pen = CreatePen(FL_PS_SOLID, 1, border);
        let brush = CreateSolidBrush(fill);
        let old_pen = if pen != 0 { SelectObject(dc, pen) } else { 0 };
        let old_brush = if brush != 0 { SelectObject(dc, brush) } else { 0 };
        RoundRect(dc, rect.left, rect.top, rect.right, rect.bottom, radius, radius);
        if old_brush != 0 { SelectObject(dc, old_brush); }
        if old_pen != 0 { SelectObject(dc, old_pen); }
        if brush != 0 { DeleteObject(brush); }
        if pen != 0 { DeleteObject(pen); }
    }

    fn fl_apply_mica(hwnd: Hwnd) {
        let corners = FL_DWMWCP_ROUND;
        let backdrop = FL_DWMSBT_MAINWINDOW;
        let result = unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd,
                FL_DWMWA_WINDOW_CORNER_PREFERENCE,
                &corners as *const Dword as *const c_void,
                size_of::<Dword>() as Dword,
            );
            DwmSetWindowAttribute(
                hwnd,
                FL_DWMWA_SYSTEMBACKDROP_TYPE,
                &backdrop as *const Dword as *const c_void,
                size_of::<Dword>() as Dword,
            )
        };
        FL_MICA_ACTIVE.store(result == 0, Ordering::Relaxed);
        unsafe { InvalidateRect(hwnd, null(), 1); }
    }

    unsafe fn fl_destroy_children(parent: Hwnd) {
        let mut child = GetWindow(parent, FL_GW_CHILD);
        while child != 0 {
            let next = GetWindow(child, FL_GW_HWNDNEXT);
            DestroyWindow(child);
            child = next;
        }
    }

    unsafe fn fl_invalidate_children(parent: Hwnd) {
        let mut child = GetWindow(parent, FL_GW_CHILD);
        while child != 0 {
            InvalidateRect(child, null(), 1);
            child = GetWindow(child, FL_GW_HWNDNEXT);
        }
    }

    unsafe fn fl_notify_parent(hwnd: Hwnd, code: usize) {
        let parent = GetParent(hwnd);
        if parent == 0 { return; }
        let id = GetDlgCtrlID(hwnd).max(0) as usize;
        SendMessageW(parent, WM_COMMAND, id | (code << 16), hwnd as Lparam);
    }

    unsafe fn fl_state(hwnd: Hwnd) -> isize {
        GetWindowLongPtrW(hwnd, FL_GWLP_USERDATA)
    }

    unsafe fn fl_set_state(hwnd: Hwnd, value: isize) {
        SetWindowLongPtrW(hwnd, FL_GWLP_USERDATA, value);
        InvalidateRect(hwnd, null(), 1);
    }

    unsafe fn fl_toggle_checked(hwnd: Hwnd) -> bool {
        fl_state(hwnd) & FL_STATE_CHECKED != 0
    }

    unsafe fn fl_set_toggle_checked(hwnd: Hwnd, checked: bool) {
        let mut state = fl_state(hwnd);
        if checked { state |= FL_STATE_CHECKED; } else { state &= !FL_STATE_CHECKED; }
        fl_set_state(hwnd, state);
    }

    unsafe fn fl_track_leave(hwnd: Hwnd) {
        let mut event = FluentTrackMouseEvent {
            cb_size: size_of::<FluentTrackMouseEvent>() as Dword,
            flags: FL_TME_LEAVE,
            hwnd_track: hwnd,
            hover_time: 0,
        };
        TrackMouseEvent(&mut event);
    }

    unsafe fn fl_toggle_paint(hwnd: Hwnd) {
        let mut paint: FluentPaintStruct = zeroed();
        let dc = BeginPaint(hwnd, &mut paint);
        if dc == 0 { return; }
        let mut rect: WorkRect = zeroed();
        GetClientRect(hwnd, &mut rect);
        let dpi = GetDpiForWindow(hwnd).max(96);
        let light = system_uses_light_apps();
        let palette = fl_palette(light);
        let brushes = fl_brushes(light);
        FillRect(dc, &rect, brushes.card);

        let state = fl_state(hwnd);
        let checked = state & FL_STATE_CHECKED != 0;
        let hover = state & FL_STATE_HOVER != 0;
        let pressed = state & FL_STATE_PRESSED != 0;

        let track_w = scale_px(40, dpi);
        let track_h = scale_px(20, dpi);
        let track_left = (rect.right - track_w) / 2;
        let track_top = (rect.bottom - track_h) / 2;
        let track = WorkRect {
            left: track_left,
            top: track_top,
            right: track_left + track_w,
            bottom: track_top + track_h,
        };
        let accent = fl_accent_color();
        let track_fill = if checked { accent } else { palette.toggle_off_fill };
        let track_border = if checked { accent } else { palette.toggle_off_stroke };
        fl_round_rect(dc, track, track_h, track_fill, track_border);

        let knob_base = if pressed { 14 } else if hover { 14 } else { 12 };
        let knob_w = if pressed { 17 } else { knob_base };
        let knob_h = knob_base;
        let knob_w = scale_px(knob_w, dpi);
        let knob_h = scale_px(knob_h, dpi);
        let margin = scale_px(if pressed { 2 } else { 4 }, dpi);
        let knob_left = if checked {
            track.right - knob_w - margin
        } else {
            track.left + margin
        };
        let knob_top = track.top + (track_h - knob_h) / 2;
        let knob_color = if checked { palette.toggle_knob_on } else { palette.toggle_knob_off };
        let brush = CreateSolidBrush(knob_color);
        let pen = CreatePen(FL_PS_SOLID, 1, knob_color);
        let old_brush = if brush != 0 { SelectObject(dc, brush) } else { 0 };
        let old_pen = if pen != 0 { SelectObject(dc, pen) } else { 0 };
        Ellipse(dc, knob_left, knob_top, knob_left + knob_w, knob_top + knob_h);
        if old_brush != 0 { SelectObject(dc, old_brush); }
        if old_pen != 0 { SelectObject(dc, old_pen); }
        if brush != 0 { DeleteObject(brush); }
        if pen != 0 { DeleteObject(pen); }

        if GetFocus() == hwnd {
            let focus = WorkRect { left: 1, top: 1, right: rect.right - 1, bottom: rect.bottom - 1 };
            let pen = CreatePen(FL_PS_SOLID, 1, fl_accent_color());
            let old_pen = if pen != 0 { SelectObject(dc, pen) } else { 0 };
            let old_brush = SelectObject(dc, GetStockObject(FL_HOLLOW_BRUSH));
            RoundRect(dc, focus.left, focus.top, focus.right, focus.bottom, scale_px(4, dpi), scale_px(4, dpi));
            if old_brush != 0 { SelectObject(dc, old_brush); }
            if old_pen != 0 { SelectObject(dc, old_pen); }
            if pen != 0 { DeleteObject(pen); }
        }
        EndPaint(hwnd, &paint);
    }

    unsafe extern "system" fn fl_toggle_wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        match msg {
            BM_GETCHECK => return if fl_toggle_checked(hwnd) { BST_CHECKED as Lresult } else { 0 },
            BM_SETCHECK => {
                fl_set_toggle_checked(hwnd, w_param as usize == BST_CHECKED);
                return 0;
            }
            FL_WM_MOUSEMOVE => {
                let state = fl_state(hwnd);
                if state & FL_STATE_HOVER == 0 {
                    fl_track_leave(hwnd);
                    fl_set_state(hwnd, state | FL_STATE_HOVER);
                }
                return 0;
            }
            FL_WM_MOUSELEAVE => {
                let state = fl_state(hwnd) & !FL_STATE_HOVER & !FL_STATE_PRESSED;
                fl_set_state(hwnd, state);
                return 0;
            }
            FL_WM_LBUTTONDOWN => {
                SetFocus(hwnd);
                fl_set_state(hwnd, fl_state(hwnd) | FL_STATE_PRESSED);
                return 0;
            }
            WM_LBUTTONUP => {
                let was_pressed = fl_state(hwnd) & FL_STATE_PRESSED != 0;
                fl_set_state(hwnd, fl_state(hwnd) & !FL_STATE_PRESSED);
                if was_pressed {
                    fl_set_toggle_checked(hwnd, !fl_toggle_checked(hwnd));
                    fl_notify_parent(hwnd, 0);
                }
                return 0;
            }
            FL_WM_KEYUP if w_param == FL_VK_SPACE => {
                fl_set_toggle_checked(hwnd, !fl_toggle_checked(hwnd));
                fl_notify_parent(hwnd, 0);
                return 0;
            }
            FL_WM_PAINT => {
                fl_toggle_paint(hwnd);
                return 0;
            }
            WM_ERASEBKGND => return 1,
            _ => {}
        }
        DefWindowProcW(hwnd, msg, w_param, l_param)
    }

    unsafe fn fl_register_toggle() -> bool {
        if FL_TOGGLE_CLASS_READY.load(Ordering::Acquire) { return true; }
        let name = wide("CatCPU.FluentToggle");
        let class = WndClassW {
            style: 0,
            wnd_proc: Some(fl_toggle_wnd_proc),
            cls_extra: 0,
            wnd_extra: 0,
            instance: GetModuleHandleW(null()),
            icon: 0,
            cursor: 0,
            background: 0,
            menu_name: null(),
            class_name: name.as_ptr(),
        };
        if RegisterClassW(&class) == 0 { return false; }
        FL_TOGGLE_CLASS_READY.store(true, Ordering::Release);
        true
    }

    unsafe fn fl_create_toggle(parent: Hwnd, id: usize, x: i32, y: i32, dpi: u32) -> Hwnd {
        if !fl_register_toggle() { return 0; }
        let name = wide("CatCPU.FluentToggle");
        CreateWindowExW(
            0,
            name.as_ptr(),
            wide("").as_ptr(),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            scale_px(x, dpi), scale_px(y, dpi), scale_px(48, dpi), scale_px(32, dpi),
            parent, id as Hmenu, GetModuleHandleW(null()), null_mut(),
        )
    }

    fn fl_combo_values(id: usize) -> &'static [&'static str] {
        match id {
            CFG_THEME => &["Use system setting", "Light", "Dark"],
            CFG_CURVE => &["Smooth", "Linear", "Reactive"],
            _ => &[""],
        }
    }

    unsafe fn fl_combo_selection(hwnd: Hwnd) -> usize {
        GetWindowLongPtrW(hwnd, FL_GWLP_USERDATA).max(0) as usize
    }

    unsafe fn fl_set_combo_selection(hwnd: Hwnd, value: usize) {
        let id = GetDlgCtrlID(hwnd).max(0) as usize;
        let max_index = fl_combo_values(id).len().saturating_sub(1);
        SetWindowLongPtrW(hwnd, FL_GWLP_USERDATA, value.min(max_index) as isize);
        InvalidateRect(hwnd, null(), 1);
    }

    unsafe fn fl_combo_paint(hwnd: Hwnd) {
        let mut paint: FluentPaintStruct = zeroed();
        let dc = BeginPaint(hwnd, &mut paint);
        if dc == 0 { return; }
        let mut rect: WorkRect = zeroed();
        GetClientRect(hwnd, &mut rect);
        let dpi = GetDpiForWindow(hwnd).max(96);
        let fonts = fl_fonts(dpi);
        let light = system_uses_light_apps();
        let palette = fl_palette(light);
        fl_round_rect(dc, rect, scale_px(FL_CARD_RADIUS, dpi), palette.control, palette.control_border);

        let id = GetDlgCtrlID(hwnd).max(0) as usize;
        let values = fl_combo_values(id);
        let index = fl_combo_selection(hwnd).min(values.len().saturating_sub(1));
        let label_rect = WorkRect {
            left: scale_px(12, dpi), top: 0,
            right: rect.right - scale_px(32, dpi), bottom: rect.bottom,
        };
        fl_draw_text(
            dc, values[index], label_rect, fonts.body, palette.text, palette.control,
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE | FL_DT_END_ELLIPSIS,
        );
        let arrow_rect = WorkRect {
            left: rect.right - scale_px(28, dpi), top: 0,
            right: rect.right - scale_px(8, dpi), bottom: rect.bottom,
        };
        fl_draw_text(
            dc, "⌄", arrow_rect, fonts.body, palette.secondary, palette.control,
            FL_DT_CENTER | FL_DT_VCENTER | FL_DT_SINGLELINE,
        );
        EndPaint(hwnd, &paint);
    }

    unsafe fn fl_combo_open(hwnd: Hwnd) {
        let id = GetDlgCtrlID(hwnd).max(0) as usize;
        let values = fl_combo_values(id);
        let current = fl_combo_selection(hwnd);
        let menu = CreatePopupMenu();
        if menu == 0 { return; }
        for (index, value) in values.iter().enumerate() {
            let mut flags = MF_STRING;
            if index == current { flags |= MF_CHECKED; }
            let text = wide(value);
            AppendMenuW(menu, flags, index + 1, text.as_ptr());
        }
        let mut rect: WorkRect = zeroed();
        GetWindowRect(hwnd, &mut rect);
        let parent = GetParent(hwnd);
        let picked = TrackPopupMenu(
            menu,
            TPM_RETURNCMD | TPM_RIGHTBUTTON | TPM_NONOTIFY,
            rect.left, rect.bottom, 0, parent, null(),
        );
        DestroyMenu(menu);
        if picked > 0 {
            fl_set_combo_selection(hwnd, picked as usize - 1);
            fl_notify_parent(hwnd, FL_CBN_SELCHANGE);
        }
    }

    unsafe extern "system" fn fl_combo_wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        match msg {
            CB_GETCURSEL => return fl_combo_selection(hwnd) as Lresult,
            CB_SETCURSEL => {
                fl_set_combo_selection(hwnd, w_param);
                return w_param as Lresult;
            }
            CB_ADDSTRING => return 0,
            WM_LBUTTONUP => {
                SetFocus(hwnd);
                fl_combo_open(hwnd);
                return 0;
            }
            FL_WM_KEYUP if matches!(w_param, FL_VK_SPACE | FL_VK_RETURN) => {
                fl_combo_open(hwnd);
                return 0;
            }
            FL_WM_KEYUP if matches!(w_param, FL_VK_LEFT | FL_VK_UP | FL_VK_RIGHT | FL_VK_DOWN) => {
                let id = GetDlgCtrlID(hwnd).max(0) as usize;
                let count = fl_combo_values(id).len();
                let current = fl_combo_selection(hwnd);
                let next = if matches!(w_param, FL_VK_LEFT | FL_VK_UP) {
                    current.saturating_sub(1)
                } else {
                    (current + 1).min(count.saturating_sub(1))
                };
                if next != current {
                    fl_set_combo_selection(hwnd, next);
                    fl_notify_parent(hwnd, FL_CBN_SELCHANGE);
                }
                return 0;
            }
            FL_WM_PAINT => {
                fl_combo_paint(hwnd);
                return 0;
            }
            WM_ERASEBKGND => return 1,
            _ => {}
        }
        DefWindowProcW(hwnd, msg, w_param, l_param)
    }

    unsafe fn fl_register_combo() -> bool {
        if FL_COMBO_CLASS_READY.load(Ordering::Acquire) { return true; }
        let name = wide("CatCPU.FluentCombo");
        let class = WndClassW {
            style: 0,
            wnd_proc: Some(fl_combo_wnd_proc),
            cls_extra: 0,
            wnd_extra: 0,
            instance: GetModuleHandleW(null()),
            icon: 0,
            cursor: 0,
            background: 0,
            menu_name: null(),
            class_name: name.as_ptr(),
        };
        if RegisterClassW(&class) == 0 { return false; }
        FL_COMBO_CLASS_READY.store(true, Ordering::Release);
        true
    }

    unsafe fn fl_create_combo(parent: Hwnd, id: usize, x: i32, y: i32, width: i32, dpi: u32) -> Hwnd {
        if !fl_register_combo() { return 0; }
        let name = wide("CatCPU.FluentCombo");
        CreateWindowExW(
            0,
            name.as_ptr(),
            wide("").as_ptr(),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            scale_px(x, dpi), scale_px(y, dpi), scale_px(width, dpi), scale_px(32, dpi),
            parent, id as Hmenu, GetModuleHandleW(null()), null_mut(),
        )
    }

    fn fl_page_ids(page: usize) -> &'static [usize] {
        match page {
            FL_PAGE_ANIMATION => &[
                CFG_SPEED, CFG_CURVE, CFG_SMOOTH, CFG_INVERT, CFG_PAUSE,
            ],
            FL_PAGE_BEHAVIOR => &[
                CFG_THRESHOLD, CFG_HYSTERESIS, CFG_SAMPLE, CFG_SLEEP,
                CFG_BATTERY_PAUSE, CFG_TOOLTIP_CPU, CFG_TOOLTIP_RAM,
                CFG_TOOLTIP_BATTERY, CFG_OVERLAY,
            ],
            _ => &[CFG_THEME, CFG_STARTUP, CFG_SIZE],
        }
    }

    unsafe fn fl_show_page(hwnd: Hwnd) {
        let active = FL_ACTIVE_PAGE.load(Ordering::Relaxed);
        for page in [FL_PAGE_APPEARANCE, FL_PAGE_ANIMATION, FL_PAGE_BEHAVIOR] {
            let show = if page == active { SW_SHOW } else { SW_HIDE };
            for id in fl_page_ids(page) {
                let control = GetDlgItem(hwnd, *id as i32);
                if control != 0 { ShowWindow(control, show); }
            }
        }
        InvalidateRect(hwnd, null(), 1);
    }

    fn fl_row_y(index: i32, control_height: i32) -> i32 {
        FL_ROW_TOP + index * FL_ROW_STEP + (FL_ROW_HEIGHT - control_height) / 2
    }

    unsafe fn fl_create_edit(
        hwnd: Hwnd,
        id: usize,
        row: i32,
        dpi: u32,
        font: Hfont,
    ) {
        let field_x = FL_MAIN_X + FL_MAIN_WIDTH - FL_CARD_PADDING - FL_ACTION_MIN_WIDTH;
        create_control(
            hwnd,
            "EDIT",
            "",
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | ES_AUTOHSCROLL,
            field_x + 8,
            fl_row_y(row, 32) + 4,
            FL_ACTION_MIN_WIDTH - 16,
            24,
            id,
            font,
            dpi,
        );
    }

    unsafe fn fl_create_controls(hwnd: Hwnd, resize_window: bool) {
        if GetWindowLongPtrW(hwnd, FL_GWLP_USERDATA) == FL_READY_MAGIC { return; }
        fl_destroy_children(hwnd);
        let dpi = GetDpiForWindow(hwnd).max(96);
        let fonts = fl_fonts(dpi);
        if resize_window {
            SetWindowPos(
                hwnd, 0, 0, 0,
                scale_px(FL_SETTINGS_WIDTH, dpi), scale_px(FL_SETTINGS_HEIGHT, dpi),
                FL_SWP_NOMOVE | FL_SWP_NOZORDER | FL_SWP_NOACTIVATE,
            );
        }

        let status = create_control(
            hwnd, "STATIC", "", WS_CHILD | WS_VISIBLE | FL_SS_NOPREFIX | SS_CENTERIMAGE,
            FL_MAIN_X + FL_CARD_PADDING, FL_STATUS_TOP,
            FL_MAIN_WIDTH - FL_CARD_PADDING * 2, FL_STATUS_HEIGHT,
            CFG_STATUS, fonts.body, dpi,
        );
        if status != 0 { SendMessageW(status, WM_SETFONT, fonts.body as Wparam, 1); }

        let nav_style = WS_CHILD | WS_VISIBLE | WS_TABSTOP | FL_BS_OWNERDRAW;
        create_control(hwnd, "BUTTON", "Appearance", nav_style, FL_GUTTER, 104, FL_NAV_WIDTH, 40, FL_NAV_APPEARANCE, fonts.body, dpi);
        create_control(hwnd, "BUTTON", "Animation", nav_style, FL_GUTTER, 152, FL_NAV_WIDTH, 40, FL_NAV_ANIMATION, fonts.body, dpi);
        create_control(hwnd, "BUTTON", "Behavior", nav_style, FL_GUTTER, 200, FL_NAV_WIDTH, 40, FL_NAV_BEHAVIOR, fonts.body, dpi);
        create_control(hwnd, "BUTTON", "Reset defaults", nav_style, FL_GUTTER, 640, FL_NAV_WIDTH, 36, FL_RESET, fonts.body, dpi);

        let action_right = FL_MAIN_X + FL_MAIN_WIDTH - FL_CARD_PADDING;
        let combo_width = 196;
        let combo_x = action_right - combo_width;
        let toggle_x = action_right - 48;

        // Appearance
        fl_create_combo(hwnd, CFG_THEME, combo_x, fl_row_y(0, 32), combo_width, dpi);
        fl_create_toggle(hwnd, CFG_STARTUP, toggle_x, fl_row_y(1, 32), dpi);
        fl_create_edit(hwnd, CFG_SIZE, 2, dpi, fonts.body);

        // Animation
        fl_create_edit(hwnd, CFG_SPEED, 0, dpi, fonts.body);
        fl_create_combo(hwnd, CFG_CURVE, combo_x, fl_row_y(1, 32), combo_width, dpi);
        fl_create_toggle(hwnd, CFG_SMOOTH, toggle_x, fl_row_y(2, 32), dpi);
        fl_create_toggle(hwnd, CFG_INVERT, toggle_x, fl_row_y(3, 32), dpi);
        fl_create_toggle(hwnd, CFG_PAUSE, toggle_x, fl_row_y(4, 32), dpi);

        // Behavior
        fl_create_edit(hwnd, CFG_THRESHOLD, 0, dpi, fonts.body);
        fl_create_edit(hwnd, CFG_HYSTERESIS, 1, dpi, fonts.body);
        fl_create_edit(hwnd, CFG_SAMPLE, 2, dpi, fonts.body);
        fl_create_toggle(hwnd, CFG_SLEEP, toggle_x, fl_row_y(3, 32), dpi);
        fl_create_toggle(hwnd, CFG_BATTERY_PAUSE, toggle_x, fl_row_y(4, 32), dpi);

        let tooltip_action_left = action_right - 300;
        fl_create_toggle(hwnd, CFG_TOOLTIP_CPU, tooltip_action_left + 28, fl_row_y(5, 32), dpi);
        fl_create_toggle(hwnd, CFG_TOOLTIP_RAM, tooltip_action_left + 124, fl_row_y(5, 32), dpi);
        fl_create_toggle(hwnd, CFG_TOOLTIP_BATTERY, tooltip_action_left + 252, fl_row_y(5, 32), dpi);
        fl_create_toggle(hwnd, CFG_OVERLAY, toggle_x, fl_row_y(6, 32), dpi);

        SetWindowLongPtrW(hwnd, FL_GWLP_USERDATA, FL_READY_MAGIC);
        apply_settings_theme(hwnd);
        sync_settings_window();
        fl_show_page(hwnd);
    }

    unsafe fn fl_draw_card(
        dc: Hdc,
        dpi: u32,
        index: i32,
        label: &str,
        description: &str,
        fonts: FluentFonts,
        palette: FluentPalette,
    ) {
        let top = FL_ROW_TOP + index * FL_ROW_STEP;
        let rect = WorkRect {
            left: scale_px(FL_MAIN_X, dpi),
            top: scale_px(top, dpi),
            right: scale_px(FL_MAIN_X + FL_MAIN_WIDTH, dpi),
            bottom: scale_px(top + FL_ROW_HEIGHT, dpi),
        };
        fl_round_rect(dc, rect, scale_px(FL_CARD_RADIUS, dpi), palette.card, palette.card_border);

        let text_right = FL_MAIN_X + FL_MAIN_WIDTH - FL_ACTION_MIN_WIDTH - 40;
        fl_draw_text(
            dc,
            label,
            WorkRect {
                left: scale_px(FL_MAIN_X + FL_CARD_PADDING, dpi),
                top: scale_px(top + 14, dpi),
                right: scale_px(text_right, dpi),
                bottom: scale_px(top + 34, dpi),
            },
            fonts.body, palette.text, palette.card,
            FL_DT_LEFT | FL_DT_SINGLELINE | FL_DT_END_ELLIPSIS,
        );
        fl_draw_text(
            dc,
            description,
            WorkRect {
                left: scale_px(FL_MAIN_X + FL_CARD_PADDING, dpi),
                top: scale_px(top + 36, dpi),
                right: scale_px(text_right, dpi),
                bottom: scale_px(top + 55, dpi),
            },
            fonts.secondary, palette.secondary, palette.card,
            FL_DT_LEFT | FL_DT_SINGLELINE | FL_DT_END_ELLIPSIS,
        );
    }

    unsafe fn fl_draw_edit_backplate(
        dc: Hdc,
        dpi: u32,
        row: i32,
        palette: FluentPalette,
    ) {
        let left = FL_MAIN_X + FL_MAIN_WIDTH - FL_CARD_PADDING - FL_ACTION_MIN_WIDTH;
        let top = fl_row_y(row, 32);
        let rect = WorkRect {
            left: scale_px(left, dpi), top: scale_px(top, dpi),
            right: scale_px(left + FL_ACTION_MIN_WIDTH, dpi), bottom: scale_px(top + 32, dpi),
        };
        fl_round_rect(dc, rect, scale_px(FL_CARD_RADIUS, dpi), palette.control, palette.control_border);
    }

    unsafe fn fl_draw_tooltip_actions(
        dc: Hdc,
        dpi: u32,
        fonts: FluentFonts,
        palette: FluentPalette,
    ) {
        let action_right = FL_MAIN_X + FL_MAIN_WIDTH - FL_CARD_PADDING;
        let left = action_right - 300;
        let top = FL_ROW_TOP + 5 * FL_ROW_STEP;
        for (x, label) in [(left, "CPU"), (left + 92, "RAM"), (left + 196, "Battery")] {
            fl_draw_text(
                dc, label,
                WorkRect {
                    left: scale_px(x, dpi), top: scale_px(top + 24, dpi),
                    right: scale_px(x + 64, dpi), bottom: scale_px(top + 45, dpi),
                },
                fonts.secondary, palette.secondary, palette.card,
                FL_DT_LEFT | FL_DT_SINGLELINE,
            );
        }
    }

    unsafe fn fl_fill_client(hwnd: Hwnd, dc: Hdc, palette: FluentPalette) {
        let mut client: WorkRect = zeroed();
        if GetClientRect(hwnd, &mut client) != 0 {
            let brush = fl_brushes(system_uses_light_apps()).base;
            if brush != 0 { FillRect(dc, &client, brush); }
        }
        let _ = palette;
    }

    unsafe fn fl_paint_page(hwnd: Hwnd) {
        let mut paint: FluentPaintStruct = zeroed();
        let dc = BeginPaint(hwnd, &mut paint);
        if dc == 0 { return; }
        let dpi = GetDpiForWindow(hwnd).max(96);
        let fonts = fl_fonts(dpi);
        let light = system_uses_light_apps();
        let palette = fl_palette(light);
        let page = FL_ACTIVE_PAGE.load(Ordering::Relaxed);
        fl_fill_client(hwnd, dc, palette);

        // Left navigation/identity.
        fl_draw_text(
            dc, "CatCPU",
            WorkRect {
                left: scale_px(FL_GUTTER, dpi), top: scale_px(24, dpi),
                right: scale_px(FL_GUTTER + FL_NAV_WIDTH, dpi), bottom: scale_px(48, dpi),
            },
            fonts.section, palette.text, palette.base,
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE,
        );
        fl_draw_text(
            dc, "Settings",
            WorkRect {
                left: scale_px(FL_GUTTER, dpi), top: scale_px(50, dpi),
                right: scale_px(FL_GUTTER + FL_NAV_WIDTH, dpi), bottom: scale_px(72, dpi),
            },
            fonts.secondary, palette.secondary, palette.base,
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE,
        );

        let (page_title, section_title) = match page {
            FL_PAGE_ANIMATION => ("Animation", "CPU response"),
            FL_PAGE_BEHAVIOR => ("Behavior", "Idle, power & tray"),
            _ => ("Appearance", "Cat"),
        };
        fl_draw_text(
            dc, page_title,
            WorkRect {
                left: scale_px(FL_MAIN_X, dpi), top: scale_px(18, dpi),
                right: scale_px(FL_MAIN_X + FL_MAIN_WIDTH, dpi), bottom: scale_px(58, dpi),
            },
            fonts.title, palette.text, palette.base,
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE,
        );

        let status_rect = WorkRect {
            left: scale_px(FL_MAIN_X, dpi), top: scale_px(FL_STATUS_TOP, dpi),
            right: scale_px(FL_MAIN_X + FL_MAIN_WIDTH, dpi),
            bottom: scale_px(FL_STATUS_TOP + FL_STATUS_HEIGHT, dpi),
        };
        fl_round_rect(dc, status_rect, scale_px(FL_CARD_RADIUS, dpi), palette.card, palette.card_border);

        fl_draw_text(
            dc, section_title,
            WorkRect {
                left: scale_px(FL_MAIN_X, dpi), top: scale_px(FL_SECTION_TOP, dpi),
                right: scale_px(FL_MAIN_X + FL_MAIN_WIDTH, dpi), bottom: scale_px(FL_SECTION_TOP + 22, dpi),
            },
            fonts.section, palette.text, palette.base,
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE,
        );

        match page {
            FL_PAGE_ANIMATION => {
                let rows = [
                    ("Speed multiplier", "Animation speed from 0.10× to 5.00×"),
                    ("Speed curve", "Choose how strongly CPU usage affects animation"),
                    ("Smooth speed transitions", "Blend changes instead of jumping between speeds"),
                    ("Invert CPU / speed", "Reverse the CPU-to-animation relationship"),
                    ("Pause animation", "Keep the cat visible without animation"),
                ];
                for (index, (label, description)) in rows.iter().enumerate() {
                    fl_draw_card(dc, dpi, index as i32, label, description, fonts, palette);
                }
                fl_draw_edit_backplate(dc, dpi, 0, palette);
            }
            FL_PAGE_BEHAVIOR => {
                let rows = [
                    ("Sleep threshold", "CPU level at or below which the cat can sleep"),
                    ("Wake hysteresis", "Extra CPU required before waking from idle"),
                    ("CPU sampling", "How often CatCPU refreshes total CPU usage"),
                    ("Sleeping cat when idle", "Show the sleeping sprite below the threshold"),
                    ("Pause animation on battery", "Reduce animation activity while unplugged"),
                    ("Tray tooltip", "Choose which live metrics appear on hover"),
                    ("Large overlay", "Use the overlay when the cat is larger than 32 px"),
                ];
                for (index, (label, description)) in rows.iter().enumerate() {
                    fl_draw_card(dc, dpi, index as i32, label, description, fonts, palette);
                }
                fl_draw_edit_backplate(dc, dpi, 0, palette);
                fl_draw_edit_backplate(dc, dpi, 1, palette);
                fl_draw_edit_backplate(dc, dpi, 2, palette);
                fl_draw_tooltip_actions(dc, dpi, fonts, palette);
            }
            _ => {
                let rows = [
                    ("Cat theme", "Follow Windows or override the cat contrast"),
                    ("Start with Windows", "Launch CatCPU when the current user signs in"),
                    ("Cat size", "Visual size from 12 to 64 px"),
                ];
                for (index, (label, description)) in rows.iter().enumerate() {
                    fl_draw_card(dc, dpi, index as i32, label, description, fonts, palette);
                }
                fl_draw_edit_backplate(dc, dpi, 2, palette);
            }
        }

        let (nav_y, nav_id) = match page {
            FL_PAGE_ANIMATION => (152, FL_NAV_ANIMATION),
            FL_PAGE_BEHAVIOR => (200, FL_NAV_BEHAVIOR),
            _ => (104, FL_NAV_APPEARANCE),
        };
        let selected_rect = WorkRect {
            left: scale_px(FL_GUTTER, dpi), top: scale_px(nav_y, dpi),
            right: scale_px(FL_GUTTER + FL_NAV_WIDTH, dpi), bottom: scale_px(nav_y + 40, dpi),
        };
        fl_round_rect(
            dc, selected_rect, scale_px(FL_CARD_RADIUS, dpi), palette.nav_selected, palette.nav_selected,
        );
        let accent = fl_accent_color();
        let accent_rect = WorkRect {
            left: scale_px(FL_GUTTER, dpi), top: scale_px(nav_y + 10, dpi),
            right: scale_px(FL_GUTTER + 3, dpi), bottom: scale_px(nav_y + 30, dpi),
        };
        fl_round_rect(dc, accent_rect, scale_px(2, dpi), accent, accent);

        // Owner-drawn buttons repaint after the parent, but invalidate the active nav
        // so the button surface and the parent selection backplate stay in sync.
        let active_nav = GetDlgItem(hwnd, nav_id as i32);
        if active_nav != 0 { InvalidateRect(active_nav, null(), 0); }
        EndPaint(hwnd, &paint);
    }

    unsafe fn fl_draw_button(draw: &FluentDrawItemStruct) {
        if draw.dc == 0 || draw.hwnd_item == 0 { return; }
        let dpi = GetDpiForWindow(draw.hwnd_item).max(96);
        let fonts = fl_fonts(dpi);
        let light = system_uses_light_apps();
        let palette = fl_palette(light);
        let id = draw.control_id as usize;
        let page = FL_ACTIVE_PAGE.load(Ordering::Relaxed);
        let is_nav = matches!(id, FL_NAV_APPEARANCE | FL_NAV_ANIMATION | FL_NAV_BEHAVIOR);
        let active = (id == FL_NAV_APPEARANCE && page == FL_PAGE_APPEARANCE)
            || (id == FL_NAV_ANIMATION && page == FL_PAGE_ANIMATION)
            || (id == FL_NAV_BEHAVIOR && page == FL_PAGE_BEHAVIOR);
        let pressed = draw.item_state & FL_ODS_SELECTED != 0;

        let _ = DrawThemeParentBackground(draw.hwnd_item, draw.dc, &draw.rect);
        let fill = if is_nav {
            if active || pressed { palette.nav_selected } else { palette.base }
        } else if pressed {
            palette.nav_selected
        } else {
            palette.card
        };
        let border = if is_nav { fill } else { palette.card_border };
        fl_round_rect(draw.dc, draw.rect, scale_px(FL_CARD_RADIUS, dpi), fill, border);

        let label = if id == FL_NAV_APPEARANCE {
            "Appearance"
        } else if id == FL_NAV_ANIMATION {
            "Animation"
        } else if id == FL_NAV_BEHAVIOR {
            "Behavior"
        } else {
            "Reset defaults"
        };
        let mut text_rect = draw.rect;
        text_rect.left += scale_px(if is_nav { 16 } else { 8 }, dpi);
        text_rect.right -= scale_px(8, dpi);
        fl_draw_text(
            draw.dc, label, text_rect, fonts.body, palette.text, fill,
            if is_nav {
                FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE
            } else {
                FL_DT_CENTER | FL_DT_VCENTER | FL_DT_SINGLELINE
            },
        );
    }

    unsafe fn fl_control_color(parent: Hwnd, w_param: Wparam, l_param: Lparam) -> Lresult {
        let light = system_uses_light_apps();
        let palette = fl_palette(light);
        let brushes = fl_brushes(light);
        let dc = w_param as Hdc;
        let child = l_param as Hwnd;
        let id = GetDlgCtrlID(child).max(0) as usize;
        SetTextColor(dc, palette.text);
        SetBkMode(dc, FL_OPAQUE);

        if matches!(id, CFG_SPEED | CFG_SIZE | CFG_THRESHOLD | CFG_HYSTERESIS | CFG_SAMPLE) {
            SetBkColor(dc, palette.control);
            brushes.control as Lresult
        } else if child == GetDlgItem(parent, CFG_STATUS as i32) {
            SetBkColor(dc, palette.card);
            brushes.card as Lresult
        } else {
            SetBkColor(dc, palette.base);
            brushes.base as Lresult
        }
    }

    fn fl_is_toggle(id: usize) -> bool {
        matches!(
            id,
            CFG_STARTUP | CFG_SMOOTH | CFG_INVERT | CFG_PAUSE | CFG_SLEEP
                | CFG_BATTERY_PAUSE | CFG_TOOLTIP_CPU | CFG_TOOLTIP_RAM
                | CFG_TOOLTIP_BATTERY | CFG_OVERLAY
        )
    }

    unsafe extern "system" fn mica_settings_wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        match msg {
            FL_WM_SHOWWINDOW => {
                if w_param != 0 {
                    fl_create_controls(hwnd, true);
                    apply_settings_theme(hwnd);
                    fl_apply_mica(hwnd);
                    sync_settings_window();
                    fl_show_page(hwnd);
                }
                return 0;
            }
            WM_SETTINGCHANGE | WM_THEMECHANGED | WM_SYSCOLORCHANGE | FL_WM_DWMCOMPOSITIONCHANGED => {
                apply_settings_theme(hwnd);
                fl_apply_mica(hwnd);
                fl_invalidate_children(hwnd);
                InvalidateRect(hwnd, null(), 1);
                return 0;
            }
            FL_WM_DPICHANGED => {
                let suggested = l_param as *const WorkRect;
                if !suggested.is_null() {
                    let rect = *suggested;
                    SetWindowPos(
                        hwnd, 0, rect.left, rect.top,
                        rect.right - rect.left, rect.bottom - rect.top,
                        FL_SWP_NOZORDER | FL_SWP_NOACTIVATE,
                    );
                }
                SetWindowLongPtrW(hwnd, FL_GWLP_USERDATA, 0);
                fl_create_controls(hwnd, false);
                sync_settings_window();
                fl_show_page(hwnd);
                return 0;
            }
            WM_COMMAND => {
                let id = w_param & 0xffff;
                let code = (w_param >> 16) & 0xffff;
                if id == FL_NAV_APPEARANCE {
                    FL_ACTIVE_PAGE.store(FL_PAGE_APPEARANCE, Ordering::Relaxed);
                    fl_show_page(hwnd);
                    return 0;
                }
                if id == FL_NAV_ANIMATION {
                    FL_ACTIVE_PAGE.store(FL_PAGE_ANIMATION, Ordering::Relaxed);
                    fl_show_page(hwnd);
                    return 0;
                }
                if id == FL_NAV_BEHAVIOR {
                    FL_ACTIVE_PAGE.store(FL_PAGE_BEHAVIOR, Ordering::Relaxed);
                    fl_show_page(hwnd);
                    return 0;
                }
                if id == FL_RESET {
                    reset_app_settings();
                    InvalidateRect(hwnd, null(), 1);
                    return 0;
                }
                if fl_is_toggle(id)
                    || ((id == CFG_THEME || id == CFG_CURVE) && code == FL_CBN_SELCHANGE)
                    || (matches!(id, CFG_SPEED | CFG_SIZE | CFG_THRESHOLD | CFG_HYSTERESIS | CFG_SAMPLE)
                        && code == FL_EN_KILLFOCUS)
                {
                    apply_settings_from_window(hwnd);
                    InvalidateRect(hwnd, null(), 1);
                    return 0;
                }
            }
            WM_ERASEBKGND => return 1,
            FL_WM_PAINT => {
                fl_paint_page(hwnd);
                return 0;
            }
            FL_WM_DRAWITEM => {
                let draw = l_param as *const FluentDrawItemStruct;
                if !draw.is_null() {
                    fl_draw_button(&*draw);
                    return 1;
                }
            }
            WM_CTLCOLOREDIT | WM_CTLCOLORSTATIC | WM_CTLCOLORBTN => {
                return fl_control_color(hwnd, w_param, l_param);
            }
            WM_DESTROY => {
                SetWindowLongPtrW(hwnd, FL_GWLP_USERDATA, 0);
            }
            _ => {}
        }
        settings_wnd_proc(hwnd, msg, w_param, l_param)
    }
