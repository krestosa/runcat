    fn scale_px(value: i32, dpi: u32) -> i32 {
        ((value as i64 * dpi.max(96) as i64 + 48) / 96) as i32
    }

    fn create_ui_font(dpi: u32, weight: i32, base_px: i32) -> Hfont {
        let face = wide("Segoe UI");
        unsafe {
            CreateFontW(
                -scale_px(base_px, dpi), 0, 0, 0, weight, 0, 0, 0,
                DEFAULT_CHARSET, 0, 0, CLEARTYPE_QUALITY, 0, face.as_ptr(),
            )
        }
    }

    fn destroy_ui_resources(state: &mut AppState) {
        unsafe {
            for object in [
                state.ui_bg_brush,
                state.ui_surface_brush,
                state.ui_font,
                state.ui_header_font,
            ] {
                if object != 0 {
                    DeleteObject(object);
                }
            }
        }
        state.ui_bg_brush = 0;
        state.ui_surface_brush = 0;
        state.ui_font = 0;
        state.ui_header_font = 0;
        state.ui_dpi = 0;
    }

    fn ensure_ui_resources(state: &mut AppState, dpi: u32, light: bool) {
        if state.ui_dpi == dpi
            && state.ui_light_theme == light
            && state.ui_bg_brush != 0
            && state.ui_surface_brush != 0
            && state.ui_font != 0
            && state.ui_header_font != 0
        {
            return;
        }

        destroy_ui_resources(state);
        let (background, surface) = if light {
            (rgb(243, 243, 243), rgb(255, 255, 255))
        } else {
            (rgb(32, 32, 32), rgb(45, 45, 45))
        };
        unsafe {
            state.ui_bg_brush = CreateSolidBrush(background);
            state.ui_surface_brush = CreateSolidBrush(surface);
        }
        state.ui_font = create_ui_font(dpi, FW_NORMAL, 14);
        state.ui_header_font = create_ui_font(dpi, FW_SEMIBOLD, 17);
        state.ui_dpi = dpi;
        state.ui_light_theme = light;
    }

    fn prepare_ui_resources(hwnd: Hwnd) -> (u32, Hfont, Hfont) {
        let dpi = unsafe {
            let value = GetDpiForWindow(hwnd);
            if value == 0 { 96 } else { value }
        };
        let light = system_uses_light_apps();
        let Some(lock) = STATE.get() else { return (dpi, 0, 0) };
        let Ok(mut state) = lock.lock() else { return (dpi, 0, 0) };
        ensure_ui_resources(&mut state, dpi, light);
        (dpi, state.ui_font, state.ui_header_font)
    }

    unsafe fn theme_child(hwnd: Hwnd, dark: bool, edit_like: bool) {
        if hwnd == 0 { return; }
        if dark {
            let name = wide(if edit_like { "DarkMode_CFD" } else { "DarkMode_Explorer" });
            SetWindowTheme(hwnd, name.as_ptr(), null());
        } else {
            SetWindowTheme(hwnd, null(), null());
        }
    }

    fn apply_settings_theme(hwnd: Hwnd) {
        if hwnd == 0 || unsafe { IsWindow(hwnd) == 0 } { return; }
        let light = system_uses_light_apps();
        let dark: Bool = if light { 0 } else { 1 };
        unsafe {
            if DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &dark as *const Bool as *const c_void,
                size_of::<Bool>() as Dword,
            ) != 0
            {
                DwmSetWindowAttribute(
                    hwnd,
                    DWMWA_USE_IMMERSIVE_DARK_MODE_OLD,
                    &dark as *const Bool as *const c_void,
                    size_of::<Bool>() as Dword,
                );
            }
        }

        let (dpi, _, _) = prepare_ui_resources(hwnd);
        let edit_like = [CFG_THEME, CFG_CURVE, CFG_SPEED, CFG_SIZE, CFG_THRESHOLD, CFG_HYSTERESIS, CFG_SAMPLE];
        let buttons = [
            CFG_STARTUP, CFG_SMOOTH, CFG_INVERT, CFG_PAUSE, CFG_SLEEP,
            CFG_BATTERY_PAUSE, CFG_TOOLTIP_CPU, CFG_TOOLTIP_RAM,
            CFG_TOOLTIP_BATTERY, CFG_OVERLAY, CFG_APPLY, CFG_RESET, CFG_CLOSE,
        ];
        unsafe {
            for id in edit_like { theme_child(GetDlgItem(hwnd, id as i32), !light, true); }
            for id in buttons { theme_child(GetDlgItem(hwnd, id as i32), !light, false); }
            InvalidateRect(hwnd, null(), 1);
            UpdateWindow(hwnd);
        }
        if let Some(lock) = STATE.get() {
            if let Ok(mut state) = lock.lock() {
                ensure_ui_resources(&mut state, dpi, light);
            }
        }
    }

    unsafe fn create_control(
        parent: Hwnd,
        class_name: &str,
        text: &str,
        style: Dword,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        id: usize,
        font: Hfont,
        dpi: u32,
    ) -> Hwnd {
        let class_name = wide(class_name);
        let text = wide(text);
        let hwnd = CreateWindowExW(
            0, class_name.as_ptr(), text.as_ptr(), style,
            scale_px(x, dpi), scale_px(y, dpi), scale_px(width, dpi), scale_px(height, dpi),
            parent, id as Hmenu, GetModuleHandleW(null()), null_mut(),
        );
        if hwnd != 0 && font != 0 { SendMessageW(hwnd, WM_SETFONT, font as Wparam, 1); }
        hwnd
    }

    unsafe fn set_control_text(parent: Hwnd, id: usize, text: &str) {
        let control = GetDlgItem(parent, id as i32);
        if control != 0 {
            let text = wide(text);
            SetWindowTextW(control, text.as_ptr());
        }
    }

    unsafe fn get_control_text(parent: Hwnd, id: usize) -> String {
        let control = GetDlgItem(parent, id as i32);
        if control == 0 { return String::new(); }
        let len = GetWindowTextLengthW(control);
        if len <= 0 { return String::new(); }
        let mut buffer = vec![0u16; len as usize + 1];
        let read = GetWindowTextW(control, buffer.as_mut_ptr(), buffer.len() as i32);
        String::from_utf16_lossy(&buffer[..read.max(0) as usize])
    }

    unsafe fn set_checkbox(parent: Hwnd, id: usize, checked: bool) {
        let control = GetDlgItem(parent, id as i32);
        if control != 0 { SendMessageW(control, BM_SETCHECK, if checked { BST_CHECKED } else { 0 }, 0); }
    }

    unsafe fn checkbox_checked(parent: Hwnd, id: usize) -> bool {
        let control = GetDlgItem(parent, id as i32);
        control != 0 && SendMessageW(control, BM_GETCHECK, 0, 0) as usize == BST_CHECKED
    }

    unsafe fn warn_settings(hwnd: Hwnd, message: &str) {
        let message = wide(message);
        let title = wide("CatCPU Settings");
        MessageBoxW(hwnd, message.as_ptr(), title.as_ptr(), MB_OK | MB_ICONWARNING);
    }

    fn power_label(power: PowerSnapshot) -> String {
        if power.on_battery {
            power.battery_percent.map(|p| format!("Battery {p}%")).unwrap_or_else(|| "Battery".to_string())
        } else {
            "AC".to_string()
        }
    }

    fn update_settings_status() {
        let Some(lock) = STATE.get() else { return };
        let Ok(state) = lock.lock() else { return };
        let hwnd = state.config_hwnd;
        if hwnd == 0 { return; }
        let text = format!(
            "CPU {:.1}%   •   RAM {:.1}%   •   {}   •   {}",
            state.cpu_percent,
            state.ram_percent,
            power_label(state.power),
            state_label(&state),
        );
        drop(state);
        unsafe { if IsWindow(hwnd) != 0 { set_control_text(hwnd, CFG_STATUS, &text); } }
    }

    fn sync_settings_window() {
        let Some(lock) = STATE.get() else { return };
        let Ok(state) = lock.lock() else { return };
        let hwnd = state.config_hwnd;
        if hwnd == 0 { return; }
        let settings = state.settings;
        drop(state);

        unsafe {
            if IsWindow(hwnd) == 0 { return; }
            let theme = GetDlgItem(hwnd, CFG_THEME as i32);
            if theme != 0 {
                let selected = match settings.theme { ThemeMode::Auto => 0, ThemeMode::Light => 1, ThemeMode::Dark => 2 };
                SendMessageW(theme, CB_SETCURSEL, selected, 0);
            }
            let curve = GetDlgItem(hwnd, CFG_CURVE as i32);
            if curve != 0 {
                let selected = match settings.speed_curve { SpeedCurve::Smooth => 0, SpeedCurve::Linear => 1, SpeedCurve::Reactive => 2 };
                SendMessageW(curve, CB_SETCURSEL, selected, 0);
            }
            set_checkbox(hwnd, CFG_STARTUP, startup_enabled());
            set_control_text(hwnd, CFG_SPEED, &format!("{:.2}", settings.speed_multiplier));
            set_control_text(hwnd, CFG_SIZE, &settings.size_px.to_string());
            set_control_text(hwnd, CFG_THRESHOLD, &format!("{:.1}", settings.idle_threshold));
            set_control_text(hwnd, CFG_HYSTERESIS, &format!("{:.1}", settings.idle_hysteresis));
            set_control_text(hwnd, CFG_SAMPLE, &settings.cpu_sample_ms.to_string());
            set_checkbox(hwnd, CFG_SMOOTH, settings.smooth_speed);
            set_checkbox(hwnd, CFG_INVERT, settings.invert_speed);
            set_checkbox(hwnd, CFG_PAUSE, settings.manual_pause);
            set_checkbox(hwnd, CFG_SLEEP, settings.sleep_idle);
            set_checkbox(hwnd, CFG_BATTERY_PAUSE, settings.pause_on_battery);
            set_checkbox(hwnd, CFG_TOOLTIP_CPU, settings.tooltip_cpu);
            set_checkbox(hwnd, CFG_TOOLTIP_RAM, settings.tooltip_ram);
            set_checkbox(hwnd, CFG_TOOLTIP_BATTERY, settings.tooltip_battery);
            set_checkbox(hwnd, CFG_OVERLAY, settings.overlay_mode);
        }
        update_settings_status();
    }

    fn parse_settings_number(hwnd: Hwnd, id: usize, min: f64, max: f64, label: &str) -> Option<f64> {
        let text = unsafe { get_control_text(hwnd, id) };
        let value = parse_f64_range(&text, min, max);
        if value.is_none() {
            unsafe { warn_settings(hwnd, &format!("{label} must be between {min} and {max}.")); }
        }
        value
    }

    fn apply_settings_from_window(hwnd: Hwnd) {
        let Some(speed_multiplier) = parse_settings_number(hwnd, CFG_SPEED, 0.10, 5.0, "Speed") else { return };
        let size_text = unsafe { get_control_text(hwnd, CFG_SIZE) };
        let Some(size_px) = parse_u32_range(&size_text, MIN_CAT_SIZE, MAX_CAT_SIZE) else {
            unsafe { warn_settings(hwnd, "Cat size must be an integer between 12 and 64 px."); }
            return;
        };
        let Some(idle_threshold) = parse_settings_number(hwnd, CFG_THRESHOLD, 0.0, 100.0, "Sleep threshold") else { return };
        let Some(idle_hysteresis) = parse_settings_number(hwnd, CFG_HYSTERESIS, 0.0, 25.0, "Wake hysteresis") else { return };
        let sample_text = unsafe { get_control_text(hwnd, CFG_SAMPLE) };
        let Some(cpu_sample_ms) = parse_u32_range(&sample_text, 250, 5000) else {
            unsafe { warn_settings(hwnd, "CPU sampling must be an integer between 250 and 5000 ms."); }
            return;
        };

        let theme_selection = unsafe { SendMessageW(GetDlgItem(hwnd, CFG_THEME as i32), CB_GETCURSEL, 0, 0) };
        let curve_selection = unsafe { SendMessageW(GetDlgItem(hwnd, CFG_CURVE as i32), CB_GETCURSEL, 0, 0) };
        let theme = match theme_selection { 1 => ThemeMode::Light, 2 => ThemeMode::Dark, _ => ThemeMode::Auto };
        let speed_curve = match curve_selection { 1 => SpeedCurve::Linear, 2 => SpeedCurve::Reactive, _ => SpeedCurve::Smooth };
        let startup = unsafe { checkbox_checked(hwnd, CFG_STARTUP) };
        if !set_startup_enabled(startup) && startup != startup_enabled() {
            unsafe { warn_settings(hwnd, "Windows startup setting could not be changed."); }
            return;
        }

        let Some(lock) = STATE.get() else { return };
        let Ok(mut state) = lock.lock() else { return };
        let previous = state.settings;
        state.settings = Settings {
            theme,
            speed_multiplier,
            speed_curve,
            size_px,
            idle_threshold,
            idle_hysteresis,
            cpu_sample_ms,
            smooth_speed: unsafe { checkbox_checked(hwnd, CFG_SMOOTH) },
            invert_speed: unsafe { checkbox_checked(hwnd, CFG_INVERT) },
            sleep_idle: unsafe { checkbox_checked(hwnd, CFG_SLEEP) },
            tooltip_cpu: unsafe { checkbox_checked(hwnd, CFG_TOOLTIP_CPU) },
            tooltip_ram: unsafe { checkbox_checked(hwnd, CFG_TOOLTIP_RAM) },
            tooltip_battery: unsafe { checkbox_checked(hwnd, CFG_TOOLTIP_BATTERY) },
            pause_on_battery: unsafe { checkbox_checked(hwnd, CFG_BATTERY_PAUSE) },
            manual_pause: unsafe { checkbox_checked(hwnd, CFG_PAUSE) },
            overlay_mode: unsafe { checkbox_checked(hwnd, CFG_OVERLAY) },
        };

        let visual_changed = previous.theme != state.settings.theme
            || previous.size_px != state.settings.size_px
            || previous.overlay_mode != state.settings.overlay_mode;
        let sample_changed = previous.cpu_sample_ms != state.settings.cpu_sample_ms;
        let saved = state.settings.save();
        unsafe {
            if sample_changed {
                KillTimer(state.hwnd, TIMER_CPU);
                SetTimer(state.hwnd, TIMER_CPU, state.settings.cpu_sample_ms, null());
            }
        }
        if visual_changed { let _ = rebuild_visuals(&mut state, true); }
        sample_runtime_metrics(&mut state);
        apply_behavior(&mut state, true);
        update_tray_tooltip_if_changed(&mut state, true);
        drop(state);
        sync_settings_window();
        if !saved {
            unsafe { warn_settings(hwnd, "Changes were applied, but settings.ini could not be saved."); }
        }
    }

    fn reset_app_settings() {
        let Some(lock) = STATE.get() else { return };
        let Ok(mut state) = lock.lock() else { return };
        state.settings = Settings::default();
        let _ = state.settings.save();
        unsafe {
            KillTimer(state.hwnd, TIMER_CPU);
            SetTimer(state.hwnd, TIMER_CPU, state.settings.cpu_sample_ms, null());
        }
        let _ = rebuild_visuals(&mut state, true);
        sample_runtime_metrics(&mut state);
        apply_behavior(&mut state, true);
        drop(state);
        sync_settings_window();
    }

    unsafe fn add_combo_items(combo: Hwnd, values: &[&str]) {
        if combo == 0 { return; }
        for value in values {
            let value = wide(value);
            SendMessageW(combo, CB_ADDSTRING, 0, value.as_ptr() as Lparam);
        }
    }

    unsafe fn initialize_settings_controls(hwnd: Hwnd) {
        let (dpi, font, header) = prepare_ui_resources(hwnd);
        let label = WS_CHILD | WS_VISIBLE;
        let edit = WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_BORDER | ES_AUTOHSCROLL;
        let check = WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_AUTOCHECKBOX;
        let button = WS_CHILD | WS_VISIBLE | WS_TABSTOP;

        create_control(hwnd, "STATIC", &format!("CatCPU {}", env!("CARGO_PKG_VERSION")), label, 28, 18, 620, 30, 0, header, dpi);
        create_control(hwnd, "STATIC", "Native, lightweight and synchronized with Windows appearance.", label, 28, 48, 620, 22, 0, font, dpi);
        create_control(hwnd, "STATIC", "", label | SS_CENTERIMAGE, 28, 78, 636, 34, CFG_STATUS, font, dpi);

        create_control(hwnd, "STATIC", "Appearance & animation", label, 28, 132, 300, 26, 0, header, dpi);
        create_control(hwnd, "STATIC", "Cat theme", label, 28, 168, 130, 22, 0, font, dpi);
        let theme = create_control(hwnd, "COMBOBOX", "", WS_CHILD | WS_VISIBLE | WS_TABSTOP | CBS_DROPDOWNLIST, 164, 164, 172, 132, CFG_THEME, font, dpi);
        add_combo_items(theme, &["Automatic", "Light / black cat", "Dark / white cat"]);
        create_control(hwnd, "BUTTON", "Start with Windows", check, 28, 204, 250, 24, CFG_STARTUP, font, dpi);
        create_control(hwnd, "STATIC", "Speed multiplier", label, 28, 246, 130, 22, 0, font, dpi);
        create_control(hwnd, "EDIT", "", edit, 164, 242, 76, 26, CFG_SPEED, font, dpi);
        create_control(hwnd, "STATIC", "0.10–5.00×", label, 248, 246, 88, 22, 0, font, dpi);
        create_control(hwnd, "STATIC", "Speed curve", label, 28, 282, 130, 22, 0, font, dpi);
        let curve = create_control(hwnd, "COMBOBOX", "", WS_CHILD | WS_VISIBLE | WS_TABSTOP | CBS_DROPDOWNLIST, 164, 278, 172, 120, CFG_CURVE, font, dpi);
        add_combo_items(curve, &["Smooth", "Linear", "Reactive"]);
        create_control(hwnd, "STATIC", "Cat size", label, 28, 318, 130, 22, 0, font, dpi);
        create_control(hwnd, "EDIT", "", edit, 164, 314, 76, 26, CFG_SIZE, font, dpi);
        create_control(hwnd, "STATIC", "12–64 px", label, 248, 318, 88, 22, 0, font, dpi);
        create_control(hwnd, "BUTTON", "Smooth speed transitions", check, 28, 358, 280, 24, CFG_SMOOTH, font, dpi);
        create_control(hwnd, "BUTTON", "Invert CPU / speed", check, 28, 390, 260, 24, CFG_INVERT, font, dpi);
        create_control(hwnd, "BUTTON", "Pause animation", check, 28, 422, 260, 24, CFG_PAUSE, font, dpi);

        create_control(hwnd, "STATIC", "Idle, power & tray", label, 372, 132, 292, 26, 0, header, dpi);
        create_control(hwnd, "STATIC", "Sleep threshold", label, 372, 168, 138, 22, 0, font, dpi);
        create_control(hwnd, "EDIT", "", edit, 518, 164, 76, 26, CFG_THRESHOLD, font, dpi);
        create_control(hwnd, "STATIC", "0–100%", label, 602, 168, 62, 22, 0, font, dpi);
        create_control(hwnd, "STATIC", "Wake hysteresis", label, 372, 204, 138, 22, 0, font, dpi);
        create_control(hwnd, "EDIT", "", edit, 518, 200, 76, 26, CFG_HYSTERESIS, font, dpi);
        create_control(hwnd, "STATIC", "0–25%", label, 602, 204, 62, 22, 0, font, dpi);
        create_control(hwnd, "STATIC", "CPU sampling", label, 372, 240, 138, 22, 0, font, dpi);
        create_control(hwnd, "EDIT", "", edit, 518, 236, 76, 26, CFG_SAMPLE, font, dpi);
        create_control(hwnd, "STATIC", "ms", label, 602, 240, 62, 22, 0, font, dpi);
        create_control(hwnd, "BUTTON", "Sleeping cat when idle", check, 372, 278, 280, 24, CFG_SLEEP, font, dpi);
        create_control(hwnd, "BUTTON", "Pause animation on battery", check, 372, 310, 280, 24, CFG_BATTERY_PAUSE, font, dpi);
        create_control(hwnd, "BUTTON", "Tooltip: CPU", check, 372, 350, 130, 24, CFG_TOOLTIP_CPU, font, dpi);
        create_control(hwnd, "BUTTON", "Tooltip: RAM", check, 514, 350, 130, 24, CFG_TOOLTIP_RAM, font, dpi);
        create_control(hwnd, "BUTTON", "Tooltip: battery", check, 372, 382, 190, 24, CFG_TOOLTIP_BATTERY, font, dpi);
        create_control(hwnd, "BUTTON", "Large overlay for >32 px", check, 372, 414, 280, 24, CFG_OVERLAY, font, dpi);

        create_control(hwnd, "STATIC", "Tip: left-click the tray cat to reopen this window.", label, 28, 470, 500, 22, 0, font, dpi);
        create_control(hwnd, "BUTTON", "Apply", button | BS_DEFPUSHBUTTON, 388, 506, 88, 32, CFG_APPLY, font, dpi);
        create_control(hwnd, "BUTTON", "Reset", button, 486, 506, 82, 32, CFG_RESET, font, dpi);
        create_control(hwnd, "BUTTON", "Close", button, 578, 506, 82, 32, CFG_CLOSE, font, dpi);

        apply_settings_theme(hwnd);
    }

    fn ui_colors(light: bool) -> (ColorRef, ColorRef, ColorRef, ColorRef) {
        if light {
            (rgb(243, 243, 243), rgb(255, 255, 255), rgb(24, 24, 24), rgb(90, 90, 90))
        } else {
            (rgb(32, 32, 32), rgb(45, 45, 45), rgb(242, 242, 242), rgb(190, 190, 190))
        }
    }

    unsafe fn settings_control_color(dc: Hdc, edit_like: bool) -> Lresult {
        let Some(lock) = STATE.get() else { return 0 };
        let Ok(state) = lock.lock() else { return 0 };
        let (_, surface, text, _) = ui_colors(state.ui_light_theme);
        SetTextColor(dc, text);
        SetBkColor(dc, surface);
        if !edit_like { SetBkMode(dc, TRANSPARENT); }
        if edit_like { state.ui_surface_brush } else { state.ui_bg_brush }
    }

    unsafe fn erase_settings_background(hwnd: Hwnd, dc: Hdc) -> Lresult {
        let Some(lock) = STATE.get() else { return 0 };
        let Ok(state) = lock.lock() else { return 0 };
        let mut rect: WorkRect = zeroed();
        if GetClientRect(hwnd, &mut rect) != 0 && state.ui_bg_brush != 0 {
            FillRect(dc, &rect, state.ui_bg_brush);
            1
        } else { 0 }
    }

    fn open_settings_window(owner: Hwnd) {
        if let Some(lock) = STATE.get() {
            if let Ok(state) = lock.lock() {
                let existing = state.config_hwnd;
                drop(state);
                unsafe {
                    if existing != 0 && IsWindow(existing) != 0 {
                        ShowWindow(existing, SW_RESTORE);
                        SetForegroundWindow(existing);
                        sync_settings_window();
                        return;
                    }
                }
            }
        }

        unsafe {
            let dpi = {
                let value = GetDpiForSystem();
                if value == 0 { 96 } else { value }
            };
            let class_name = wide("CatCPU.SettingsWindow");
            let title = wide("CatCPU Settings");
            let hwnd = CreateWindowExW(
                0, class_name.as_ptr(), title.as_ptr(),
                WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
                CW_USEDEFAULT, CW_USEDEFAULT, scale_px(710, dpi), scale_px(590, dpi),
                owner, 0, GetModuleHandleW(null()), null_mut(),
            );
            if hwnd == 0 { return; }
            if let Some(lock) = STATE.get() {
                if let Ok(mut state) = lock.lock() { state.config_hwnd = hwnd; }
            }
            initialize_settings_controls(hwnd);
            sync_settings_window();
            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);
            SetForegroundWindow(hwnd);
        }
    }

    unsafe extern "system" fn settings_wnd_proc(hwnd: Hwnd, msg: Uint, w_param: Wparam, l_param: Lparam) -> Lresult {
        match msg {
            WM_COMMAND => {
                match w_param & 0xffff {
                    CFG_APPLY => apply_settings_from_window(hwnd),
                    CFG_RESET => reset_app_settings(),
                    CFG_CLOSE => { DestroyWindow(hwnd); },
                    _ => {}
                }
                0
            }
            WM_SETTINGCHANGE | WM_THEMECHANGED | WM_SYSCOLORCHANGE => {
                apply_settings_theme(hwnd);
                0
            }
            WM_ERASEBKGND => erase_settings_background(hwnd, w_param as Hdc),
            WM_CTLCOLOREDIT | WM_CTLCOLORLISTBOX => settings_control_color(w_param as Hdc, true),
            WM_CTLCOLORSTATIC | WM_CTLCOLORBTN => settings_control_color(w_param as Hdc, false),
            WM_CLOSE => { DestroyWindow(hwnd); 0 }
            WM_DESTROY => {
                if let Some(lock) = STATE.get() {
                    if let Ok(mut state) = lock.lock() {
                        if state.config_hwnd == hwnd { state.config_hwnd = 0; }
                    }
                }
                0
            }
            _ => DefWindowProcW(hwnd, msg, w_param, l_param),
        }
    }
