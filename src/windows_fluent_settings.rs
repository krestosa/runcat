    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    const FL_WM_PAINT: Uint = 0x000F;
    const FL_WM_SHOWWINDOW: Uint = 0x0018;
    const FL_WM_DRAWITEM: Uint = 0x002B;
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

    const FL_DT_LEFT: Uint = 0x0000;
    const FL_DT_CENTER: Uint = 0x0001;
    const FL_DT_VCENTER: Uint = 0x0004;
    const FL_DT_SINGLELINE: Uint = 0x0020;
    const FL_DT_NOPREFIX: Uint = 0x0800;
    const FL_DT_END_ELLIPSIS: Uint = 0x8000;

    const FL_READY_MAGIC: isize = 0x4341_5453;
    const FL_SETTINGS_WIDTH: i32 = 840;
    const FL_SETTINGS_HEIGHT: i32 = 720;
    const FL_GUTTER: i32 = 24;
    const FL_NAV_WIDTH: i32 = 180;
    const FL_MAIN_X: i32 = 228;
    const FL_MAIN_WIDTH: i32 = 588;
    const FL_STATUS_TOP: i32 = 80;
    const FL_STATUS_HEIGHT: i32 = 48;
    const FL_SECTION_TOP: i32 = 140;
    const FL_ROW_TOP: i32 = 176;
    const FL_ROW_HEIGHT: i32 = 48;
    const FL_ROW_STEP: i32 = 56;
    const FL_CARD_RADIUS: i32 = 4;

    const FL_NAV_APPEARANCE: usize = 3101;
    const FL_NAV_BEHAVIOR: usize = 3102;
    const FL_RESET: usize = 3103;
    const FL_PAGE_APPEARANCE: usize = 0;
    const FL_PAGE_BEHAVIOR: usize = 1;

    static FL_MICA_ACTIVE: AtomicBool = AtomicBool::new(false);
    static FL_TOGGLE_CLASS_READY: AtomicBool = AtomicBool::new(false);
    static FL_COMBO_CLASS_READY: AtomicBool = AtomicBool::new(false);
    static FL_ACTIVE_PAGE: AtomicUsize = AtomicUsize::new(FL_PAGE_APPEARANCE);

    #[repr(C)]
    struct FluentMargins {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

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

    static FL_FONTS: OnceLock<Mutex<FluentFonts>> = OnceLock::new();

    #[link(name = "dwmapi")]
    extern "system" {
        fn DwmExtendFrameIntoClientArea(hwnd: Hwnd, margins: *const FluentMargins) -> i32;
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

    fn fl_create_font(dpi: u32, weight: i32, base_px: i32) -> Hfont {
        let variable = wide("Segoe UI Variable");
        let fallback = wide("Segoe UI");
        unsafe {
            let font = CreateFontW(
                -scale_px(base_px, dpi),
                0,
                0,
                0,
                weight,
                0,
                0,
                0,
                DEFAULT_CHARSET,
                0,
                0,
                CLEARTYPE_QUALITY,
                0,
                variable.as_ptr(),
            );
            if font != 0 {
                font
            } else {
                CreateFontW(
                    -scale_px(base_px, dpi),
                    0,
                    0,
                    0,
                    weight,
                    0,
                    0,
                    0,
                    DEFAULT_CHARSET,
                    0,
                    0,
                    CLEARTYPE_QUALITY,
                    0,
                    fallback.as_ptr(),
                )
            }
        }
    }

    fn fl_fonts(dpi: u32) -> FluentFonts {
        let lock = FL_FONTS.get_or_init(|| {
            Mutex::new(FluentFonts {
                dpi: 0,
                body: 0,
                secondary: 0,
                section: 0,
                title: 0,
            })
        });
        let Ok(mut fonts) = lock.lock() else {
            return FluentFonts { dpi, body: 0, secondary: 0, section: 0, title: 0 };
        };
        if fonts.dpi != dpi || fonts.body == 0 {
            unsafe {
                for object in [fonts.body, fonts.secondary, fonts.section, fonts.title] {
                    if object != 0 {
                        DeleteObject(object);
                    }
                }
            }
            *fonts = FluentFonts {
                dpi,
                body: fl_create_font(dpi, FW_NORMAL, 14),
                secondary: fl_create_font(dpi, FW_NORMAL, 12),
                section: fl_create_font(dpi, FW_SEMIBOLD, 14),
                title: fl_create_font(dpi, FW_SEMIBOLD, 24),
            };
        }
        *fonts
    }

    fn fl_colors(light: bool) -> (ColorRef, ColorRef, ColorRef, ColorRef, ColorRef) {
        if light {
            (
                rgb(255, 255, 255),
                rgb(31, 31, 31),
                rgb(96, 96, 96),
                rgb(224, 224, 224),
                rgb(238, 238, 238),
            )
        } else {
            (
                rgb(45, 45, 45),
                rgb(245, 245, 245),
                rgb(190, 190, 190),
                rgb(64, 64, 64),
                rgb(50, 50, 50),
            )
        }
    }

    fn fl_accent_color() -> ColorRef {
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

    unsafe fn fl_draw_text(
        dc: Hdc,
        text: &str,
        mut rect: WorkRect,
        font: Hfont,
        color: ColorRef,
        background: Option<ColorRef>,
        flags: Uint,
    ) {
        let old_font = if font != 0 { SelectObject(dc, font) } else { 0 };
        SetTextColor(dc, color);
        if let Some(fill) = background {
            SetBkMode(dc, FL_OPAQUE);
            SetBkColor(dc, fill);
        } else {
            SetBkMode(dc, FL_TRANSPARENT);
        }
        let text = wide(text);
        DrawTextW(dc, text.as_ptr(), -1, &mut rect, flags | FL_DT_NOPREFIX);
        if old_font != 0 {
            SelectObject(dc, old_font);
        }
    }

    unsafe fn fl_round_rect(
        dc: Hdc,
        rect: WorkRect,
        radius: i32,
        fill: Option<ColorRef>,
        border: ColorRef,
    ) {
        let pen = CreatePen(FL_PS_SOLID, 1, border);
        let brush = match fill {
            Some(color) => CreateSolidBrush(color),
            None => 0,
        };
        let old_pen = if pen != 0 { SelectObject(dc, pen) } else { 0 };
        let old_brush = if brush != 0 {
            SelectObject(dc, brush)
        } else {
            SelectObject(dc, GetStockObject(FL_HOLLOW_BRUSH))
        };
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
        let active = result == 0;
        let margins = if active {
            FluentMargins { left: -1, right: -1, top: -1, bottom: -1 }
        } else {
            FluentMargins { left: 0, right: 0, top: 0, bottom: 0 }
        };
        unsafe {
            let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
            InvalidateRect(hwnd, null(), 1);
        }
        FL_MICA_ACTIVE.store(active, Ordering::Relaxed);
    }

    unsafe fn fl_destroy_children(parent: Hwnd) {
        let mut child = GetWindow(parent, FL_GW_CHILD);
        while child != 0 {
            let next = GetWindow(child, FL_GW_HWNDNEXT);
            DestroyWindow(child);
            child = next;
        }
    }

    unsafe fn fl_notify_parent(hwnd: Hwnd, code: usize) {
        let parent = GetParent(hwnd);
        if parent == 0 { return; }
        let id = GetDlgCtrlID(hwnd).max(0) as usize;
        SendMessageW(parent, WM_COMMAND, id | (code << 16), hwnd as Lparam);
    }

    unsafe fn fl_toggle_checked(hwnd: Hwnd) -> bool {
        GetWindowLongPtrW(hwnd, FL_GWLP_USERDATA) != 0
    }

    unsafe fn fl_set_toggle_checked(hwnd: Hwnd, checked: bool) {
        SetWindowLongPtrW(hwnd, FL_GWLP_USERDATA, if checked { 1 } else { 0 });
        InvalidateRect(hwnd, null(), 1);
    }

    unsafe fn fl_toggle_paint(hwnd: Hwnd) {
        let mut paint: FluentPaintStruct = zeroed();
        let dc = BeginPaint(hwnd, &mut paint);
        if dc == 0 { return; }
        let mut rect: WorkRect = zeroed();
        GetClientRect(hwnd, &mut rect);
        let dpi = GetDpiForWindow(hwnd).max(96);
        let light = system_uses_light_apps();
        let (surface, _, _, border, _) = fl_colors(light);
        let bg = CreateSolidBrush(surface);
        if bg != 0 {
            FillRect(dc, &rect, bg);
            DeleteObject(bg);
        }

        let checked = fl_toggle_checked(hwnd);
        let track_w = scale_px(40, dpi);
        let track_h = scale_px(20, dpi);
        let track_left = rect.right - track_w;
        let track_top = (rect.bottom - track_h) / 2;
        let track = WorkRect {
            left: track_left,
            top: track_top,
            right: track_left + track_w,
            bottom: track_top + track_h,
        };
        let track_color = if checked {
            fl_accent_color()
        } else if light {
            rgb(118, 118, 118)
        } else {
            rgb(105, 105, 105)
        };
        fl_round_rect(dc, track, track_h, Some(track_color), track_color);

        let thumb = scale_px(16, dpi);
        let inset = scale_px(2, dpi);
        let thumb_left = if checked {
            track.right - thumb - inset
        } else {
            track.left + inset
        };
        let thumb_top = track.top + inset;
        let thumb_color = rgb(255, 255, 255);
        let brush = CreateSolidBrush(thumb_color);
        let pen = CreatePen(FL_PS_SOLID, 1, thumb_color);
        let old_brush = if brush != 0 { SelectObject(dc, brush) } else { 0 };
        let old_pen = if pen != 0 { SelectObject(dc, pen) } else { 0 };
        Ellipse(dc, thumb_left, thumb_top, thumb_left + thumb, thumb_top + thumb);
        if old_brush != 0 { SelectObject(dc, old_brush); }
        if old_pen != 0 { SelectObject(dc, old_pen); }
        if brush != 0 { DeleteObject(brush); }
        if pen != 0 { DeleteObject(pen); }

        if GetFocus() == hwnd {
            let focus = WorkRect { left: 0, top: 0, right: rect.right, bottom: rect.bottom };
            fl_round_rect(dc, focus, scale_px(4, dpi), None, border);
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
            WM_LBUTTONUP => {
                SetFocus(hwnd);
                fl_set_toggle_checked(hwnd, !fl_toggle_checked(hwnd));
                fl_notify_parent(hwnd, 0);
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
            scale_px(x, dpi),
            scale_px(y, dpi),
            scale_px(44, dpi),
            scale_px(28, dpi),
            parent,
            id as Hmenu,
            GetModuleHandleW(null()),
            null_mut(),
        )
    }

    fn fl_combo_values(id: usize) -> &'static [&'static str] {
        match id {
            CFG_THEME => &["Automatic", "Light / black cat", "Dark / white cat"],
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
        let (surface, text, _, border, _) = fl_colors(light);
        fl_round_rect(dc, rect, scale_px(FL_CARD_RADIUS, dpi), Some(surface), border);

        let id = GetDlgCtrlID(hwnd).max(0) as usize;
        let values = fl_combo_values(id);
        let index = fl_combo_selection(hwnd).min(values.len().saturating_sub(1));
        let label_rect = WorkRect {
            left: scale_px(12, dpi),
            top: 0,
            right: rect.right - scale_px(32, dpi),
            bottom: rect.bottom,
        };
        fl_draw_text(
            dc,
            values[index],
            label_rect,
            fonts.body,
            text,
            Some(surface),
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE | FL_DT_END_ELLIPSIS,
        );
        let arrow_rect = WorkRect {
            left: rect.right - scale_px(28, dpi),
            top: 0,
            right: rect.right - scale_px(8, dpi),
            bottom: rect.bottom,
        };
        fl_draw_text(
            dc,
            "⌄",
            arrow_rect,
            fonts.body,
            text,
            Some(surface),
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
            rect.left,
            rect.bottom,
            0,
            parent,
            null(),
        );
        DestroyMenu(menu);
        if picked > 0 {
            let next = picked as usize - 1;
            fl_set_combo_selection(hwnd, next);
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
            scale_px(x, dpi),
            scale_px(y, dpi),
            scale_px(width, dpi),
            scale_px(32, dpi),
            parent,
            id as Hmenu,
            GetModuleHandleW(null()),
            null_mut(),
        )
    }

    fn fl_page_ids(page: usize) -> &'static [usize] {
        if page == FL_PAGE_BEHAVIOR {
            &[
                CFG_THRESHOLD,
                CFG_HYSTERESIS,
                CFG_SAMPLE,
                CFG_SLEEP,
                CFG_BATTERY_PAUSE,
                CFG_TOOLTIP_CPU,
                CFG_TOOLTIP_RAM,
                CFG_TOOLTIP_BATTERY,
                CFG_OVERLAY,
            ]
        } else {
            &[
                CFG_THEME,
                CFG_STARTUP,
                CFG_SPEED,
                CFG_CURVE,
                CFG_SIZE,
                CFG_SMOOTH,
                CFG_INVERT,
                CFG_PAUSE,
            ]
        }
    }

    unsafe fn fl_show_page(hwnd: Hwnd) {
        let active = FL_ACTIVE_PAGE.load(Ordering::Relaxed);
        for page in [FL_PAGE_APPEARANCE, FL_PAGE_BEHAVIOR] {
            let show = if page == active { SW_SHOW } else { SW_HIDE };
            for id in fl_page_ids(page) {
                let control = GetDlgItem(hwnd, *id as i32);
                if control != 0 { ShowWindow(control, show); }
            }
        }
        InvalidateRect(hwnd, null(), 1);
    }

    unsafe fn fl_create_controls(hwnd: Hwnd, resize_window: bool) {
        if GetWindowLongPtrW(hwnd, FL_GWLP_USERDATA) == FL_READY_MAGIC { return; }
        fl_destroy_children(hwnd);
        let dpi = GetDpiForWindow(hwnd).max(96);
        let fonts = fl_fonts(dpi);
        if resize_window {
            SetWindowPos(
                hwnd,
                0,
                0,
                0,
                scale_px(FL_SETTINGS_WIDTH, dpi),
                scale_px(FL_SETTINGS_HEIGHT, dpi),
                FL_SWP_NOMOVE | FL_SWP_NOZORDER | FL_SWP_NOACTIVATE,
            );
        }

        let status = create_control(
            hwnd,
            "STATIC",
            "",
            WS_CHILD | WS_VISIBLE | FL_SS_NOPREFIX | SS_CENTERIMAGE,
            FL_MAIN_X + 16,
            FL_STATUS_TOP,
            FL_MAIN_WIDTH - 32,
            FL_STATUS_HEIGHT,
            CFG_STATUS,
            fonts.body,
            dpi,
        );
        if status != 0 { SendMessageW(status, WM_SETFONT, fonts.body as Wparam, 1); }

        let nav_style = WS_CHILD | WS_VISIBLE | WS_TABSTOP | FL_BS_OWNERDRAW;
        create_control(hwnd, "BUTTON", "Appearance", nav_style, FL_GUTTER, 104, FL_NAV_WIDTH, 40, FL_NAV_APPEARANCE, fonts.body, dpi);
        create_control(hwnd, "BUTTON", "Behavior", nav_style, FL_GUTTER, 152, FL_NAV_WIDTH, 40, FL_NAV_BEHAVIOR, fonts.body, dpi);
        create_control(hwnd, "BUTTON", "Reset defaults", nav_style, FL_GUTTER, 628, FL_NAV_WIDTH, 36, FL_RESET, fonts.body, dpi);

        let combo_x = FL_MAIN_X + FL_MAIN_WIDTH - 16 - 176;
        let edit_x = FL_MAIN_X + FL_MAIN_WIDTH - 16 - 104;
        let toggle_x = FL_MAIN_X + FL_MAIN_WIDTH - 16 - 44;
        let action_y = |index: i32, control_height: i32| -> i32 {
            FL_ROW_TOP + index * FL_ROW_STEP + (FL_ROW_HEIGHT - control_height) / 2
        };

        fl_create_combo(hwnd, CFG_THEME, combo_x, action_y(0, 32), 176, dpi);
        fl_create_toggle(hwnd, CFG_STARTUP, toggle_x, action_y(1, 28), dpi);
        create_control(hwnd, "EDIT", "", WS_CHILD | WS_VISIBLE | WS_TABSTOP | ES_AUTOHSCROLL, edit_x, action_y(2, 32), 104, 32, CFG_SPEED, fonts.body, dpi);
        fl_create_combo(hwnd, CFG_CURVE, combo_x, action_y(3, 32), 176, dpi);
        create_control(hwnd, "EDIT", "", WS_CHILD | WS_VISIBLE | WS_TABSTOP | ES_AUTOHSCROLL, edit_x, action_y(4, 32), 104, 32, CFG_SIZE, fonts.body, dpi);
        fl_create_toggle(hwnd, CFG_SMOOTH, toggle_x, action_y(5, 28), dpi);
        fl_create_toggle(hwnd, CFG_INVERT, toggle_x, action_y(6, 28), dpi);
        fl_create_toggle(hwnd, CFG_PAUSE, toggle_x, action_y(7, 28), dpi);

        create_control(hwnd, "EDIT", "", WS_CHILD | WS_VISIBLE | WS_TABSTOP | ES_AUTOHSCROLL, edit_x, action_y(0, 32), 104, 32, CFG_THRESHOLD, fonts.body, dpi);
        create_control(hwnd, "EDIT", "", WS_CHILD | WS_VISIBLE | WS_TABSTOP | ES_AUTOHSCROLL, edit_x, action_y(1, 32), 104, 32, CFG_HYSTERESIS, fonts.body, dpi);
        create_control(hwnd, "EDIT", "", WS_CHILD | WS_VISIBLE | WS_TABSTOP | ES_AUTOHSCROLL, edit_x, action_y(2, 32), 104, 32, CFG_SAMPLE, fonts.body, dpi);
        fl_create_toggle(hwnd, CFG_SLEEP, toggle_x, action_y(3, 28), dpi);
        fl_create_toggle(hwnd, CFG_BATTERY_PAUSE, toggle_x, action_y(4, 28), dpi);
        fl_create_toggle(hwnd, CFG_TOOLTIP_CPU, toggle_x, action_y(5, 28), dpi);
        fl_create_toggle(hwnd, CFG_TOOLTIP_RAM, toggle_x, action_y(6, 28), dpi);
        fl_create_toggle(hwnd, CFG_TOOLTIP_BATTERY, toggle_x, action_y(7, 28), dpi);
        fl_create_toggle(hwnd, CFG_OVERLAY, toggle_x, action_y(8, 28), dpi);

        SetWindowLongPtrW(hwnd, FL_GWLP_USERDATA, FL_READY_MAGIC);
        apply_settings_theme(hwnd);
        sync_settings_window();
        fl_show_page(hwnd);
    }

    unsafe fn fl_draw_card_text(
        dc: Hdc,
        dpi: u32,
        index: i32,
        label: &str,
        description: &str,
        fonts: FluentFonts,
        surface: ColorRef,
        text: ColorRef,
        secondary: ColorRef,
    ) {
        let top = FL_ROW_TOP + index * FL_ROW_STEP;
        let left = FL_MAIN_X + 16;
        let right = FL_MAIN_X + FL_MAIN_WIDTH - 200;
        fl_draw_text(
            dc,
            label,
            WorkRect {
                left: scale_px(left, dpi),
                top: scale_px(top + 7, dpi),
                right: scale_px(right, dpi),
                bottom: scale_px(top + 26, dpi),
            },
            fonts.body,
            text,
            Some(surface),
            FL_DT_LEFT | FL_DT_SINGLELINE | FL_DT_END_ELLIPSIS,
        );
        fl_draw_text(
            dc,
            description,
            WorkRect {
                left: scale_px(left, dpi),
                top: scale_px(top + 25, dpi),
                right: scale_px(right, dpi),
                bottom: scale_px(top + 43, dpi),
            },
            fonts.secondary,
            secondary,
            Some(surface),
            FL_DT_LEFT | FL_DT_SINGLELINE | FL_DT_END_ELLIPSIS,
        );
    }

    unsafe fn fl_paint_page(hwnd: Hwnd) {
        let mut paint: FluentPaintStruct = zeroed();
        let dc = BeginPaint(hwnd, &mut paint);
        if dc == 0 { return; }
        let dpi = GetDpiForWindow(hwnd).max(96);
        let fonts = fl_fonts(dpi);
        let light = system_uses_light_apps();
        let (surface, text, secondary, border, selected) = fl_colors(light);
        let page = FL_ACTIVE_PAGE.load(Ordering::Relaxed);

        if !FL_MICA_ACTIVE.load(Ordering::Relaxed) {
            if let Some(lock) = STATE.get() {
                if let Ok(state) = lock.lock() {
                    let mut client: WorkRect = zeroed();
                    if GetClientRect(hwnd, &mut client) != 0 && state.ui_bg_brush != 0 {
                        FillRect(dc, &client, state.ui_bg_brush);
                    }
                }
            }
        }

        fl_draw_text(
            dc,
            "CatCPU",
            WorkRect {
                left: scale_px(FL_GUTTER, dpi),
                top: scale_px(24, dpi),
                right: scale_px(FL_GUTTER + FL_NAV_WIDTH, dpi),
                bottom: scale_px(52, dpi),
            },
            fonts.section,
            text,
            None,
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE,
        );
        fl_draw_text(
            dc,
            "Settings",
            WorkRect {
                left: scale_px(FL_GUTTER, dpi),
                top: scale_px(50, dpi),
                right: scale_px(FL_GUTTER + FL_NAV_WIDTH, dpi),
                bottom: scale_px(72, dpi),
            },
            fonts.secondary,
            secondary,
            None,
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE,
        );

        let page_title = if page == FL_PAGE_BEHAVIOR { "Behavior" } else { "Appearance" };
        fl_draw_text(
            dc,
            page_title,
            WorkRect {
                left: scale_px(FL_MAIN_X, dpi),
                top: scale_px(20, dpi),
                right: scale_px(FL_MAIN_X + FL_MAIN_WIDTH, dpi),
                bottom: scale_px(58, dpi),
            },
            fonts.title,
            text,
            None,
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE,
        );

        let status_rect = WorkRect {
            left: scale_px(FL_MAIN_X, dpi),
            top: scale_px(FL_STATUS_TOP, dpi),
            right: scale_px(FL_MAIN_X + FL_MAIN_WIDTH, dpi),
            bottom: scale_px(FL_STATUS_TOP + FL_STATUS_HEIGHT, dpi),
        };
        fl_round_rect(dc, status_rect, scale_px(FL_CARD_RADIUS, dpi), Some(surface), border);

        let section = if page == FL_PAGE_BEHAVIOR { "Idle, power & tray" } else { "Appearance & animation" };
        fl_draw_text(
            dc,
            section,
            WorkRect {
                left: scale_px(FL_MAIN_X, dpi),
                top: scale_px(FL_SECTION_TOP, dpi),
                right: scale_px(FL_MAIN_X + FL_MAIN_WIDTH, dpi),
                bottom: scale_px(FL_SECTION_TOP + 24, dpi),
            },
            fonts.section,
            text,
            None,
            FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE,
        );

        let rows = if page == FL_PAGE_BEHAVIOR { 9 } else { 8 };
        for index in 0..rows {
            let top = FL_ROW_TOP + index * FL_ROW_STEP;
            let rect = WorkRect {
                left: scale_px(FL_MAIN_X, dpi),
                top: scale_px(top, dpi),
                right: scale_px(FL_MAIN_X + FL_MAIN_WIDTH, dpi),
                bottom: scale_px(top + FL_ROW_HEIGHT, dpi),
            };
            fl_round_rect(dc, rect, scale_px(FL_CARD_RADIUS, dpi), Some(surface), border);
        }

        if page == FL_PAGE_BEHAVIOR {
            let labels = [
                ("Sleep threshold", "0–100% CPU"),
                ("Wake hysteresis", "0–25 percentage points"),
                ("CPU sampling", "250–5000 ms"),
                ("Sleeping cat when idle", "Show the sleeping sprite below the threshold"),
                ("Pause animation on battery", "Reduce animation activity while unplugged"),
                ("Tooltip: CPU", "Show CPU usage in the tray tooltip"),
                ("Tooltip: RAM", "Show RAM usage in the tray tooltip"),
                ("Tooltip: battery", "Show power state in the tray tooltip"),
                ("Large overlay", "Use the overlay when the cat is larger than 32 px"),
            ];
            for (index, (label, description)) in labels.iter().enumerate() {
                fl_draw_card_text(dc, dpi, index as i32, label, description, fonts, surface, text, secondary);
            }
        } else {
            let labels = [
                ("Cat theme", "Follow Windows or override the cat contrast"),
                ("Start with Windows", "Launch CatCPU when the current user signs in"),
                ("Speed multiplier", "Animation speed from 0.10× to 5.00×"),
                ("Speed curve", "Choose how strongly CPU usage affects animation"),
                ("Cat size", "Visual size from 12 to 64 px"),
                ("Smooth speed transitions", "Blend changes instead of jumping between speeds"),
                ("Invert CPU / speed", "Reverse the CPU-to-animation relationship"),
                ("Pause animation", "Keep the cat visible without animation"),
            ];
            for (index, (label, description)) in labels.iter().enumerate() {
                fl_draw_card_text(dc, dpi, index as i32, label, description, fonts, surface, text, secondary);
            }
        }

        let active_nav_top = if page == FL_PAGE_BEHAVIOR { 152 } else { 104 };
        let accent = fl_accent_color();
        let accent_rect = WorkRect {
            left: scale_px(FL_GUTTER, dpi),
            top: scale_px(active_nav_top + 10, dpi),
            right: scale_px(FL_GUTTER + 4, dpi),
            bottom: scale_px(active_nav_top + 30, dpi),
        };
        fl_round_rect(dc, accent_rect, scale_px(2, dpi), Some(accent), accent);

        let _ = selected;
        EndPaint(hwnd, &paint);
    }

    unsafe fn fl_draw_button(draw: &FluentDrawItemStruct) {
        if draw.dc == 0 || draw.hwnd_item == 0 { return; }
        let dpi = GetDpiForWindow(draw.hwnd_item).max(96);
        let fonts = fl_fonts(dpi);
        let light = system_uses_light_apps();
        let (surface, text, _, border, selected) = fl_colors(light);
        let id = draw.control_id as usize;
        let page = FL_ACTIVE_PAGE.load(Ordering::Relaxed);
        let is_nav = matches!(id, FL_NAV_APPEARANCE | FL_NAV_BEHAVIOR);
        let active = (id == FL_NAV_APPEARANCE && page == FL_PAGE_APPEARANCE)
            || (id == FL_NAV_BEHAVIOR && page == FL_PAGE_BEHAVIOR);
        let pressed = draw.item_state & FL_ODS_SELECTED != 0;

        let _ = DrawThemeParentBackground(draw.hwnd_item, draw.dc, &draw.rect);
        let fill = if is_nav {
            if active || pressed { Some(selected) } else { None }
        } else if pressed {
            Some(selected)
        } else {
            Some(surface)
        };
        if fill.is_some() {
            fl_round_rect(draw.dc, draw.rect, scale_px(FL_CARD_RADIUS, dpi), fill, if is_nav { fill.unwrap_or(surface) } else { border });
        }

        let label = if id == FL_NAV_APPEARANCE {
            "Appearance"
        } else if id == FL_NAV_BEHAVIOR {
            "Behavior"
        } else {
            "Reset defaults"
        };
        let mut text_rect = draw.rect;
        text_rect.left += scale_px(if is_nav { 16 } else { 8 }, dpi);
        text_rect.right -= scale_px(8, dpi);
        fl_draw_text(
            draw.dc,
            label,
            text_rect,
            fonts.body,
            text,
            fill,
            if is_nav {
                FL_DT_LEFT | FL_DT_VCENTER | FL_DT_SINGLELINE
            } else {
                FL_DT_CENTER | FL_DT_VCENTER | FL_DT_SINGLELINE
            },
        );
    }

    unsafe fn fl_control_color(parent: Hwnd, w_param: Wparam, l_param: Lparam) -> Lresult {
        let Some(lock) = STATE.get() else { return 0; };
        let Ok(state) = lock.lock() else { return 0; };
        let light = state.ui_light_theme;
        let surface_brush = state.ui_surface_brush;
        let (surface, text, _, _, _) = fl_colors(light);
        drop(state);
        let dc = w_param as Hdc;
        let child = l_param as Hwnd;
        SetTextColor(dc, text);
        if child == GetDlgItem(parent, CFG_STATUS as i32)
            || matches!(GetDlgCtrlID(child).max(0) as usize, CFG_SPEED | CFG_SIZE | CFG_THRESHOLD | CFG_HYSTERESIS | CFG_SAMPLE)
        {
            SetBkMode(dc, FL_OPAQUE);
            SetBkColor(dc, surface);
            surface_brush as Lresult
        } else {
            SetBkMode(dc, FL_TRANSPARENT);
            GetStockObject(FL_HOLLOW_BRUSH) as Lresult
        }
    }

    fn fl_is_toggle(id: usize) -> bool {
        matches!(
            id,
            CFG_STARTUP
                | CFG_SMOOTH
                | CFG_INVERT
                | CFG_PAUSE
                | CFG_SLEEP
                | CFG_BATTERY_PAUSE
                | CFG_TOOLTIP_CPU
                | CFG_TOOLTIP_RAM
                | CFG_TOOLTIP_BATTERY
                | CFG_OVERLAY
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
                InvalidateRect(hwnd, null(), 1);
                return 0;
            }
            FL_WM_DPICHANGED => {
                let suggested = l_param as *const WorkRect;
                if !suggested.is_null() {
                    let rect = *suggested;
                    SetWindowPos(
                        hwnd,
                        0,
                        rect.left,
                        rect.top,
                        rect.right - rect.left,
                        rect.bottom - rect.top,
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
            WM_ERASEBKGND if FL_MICA_ACTIVE.load(Ordering::Relaxed) => return 1,
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
