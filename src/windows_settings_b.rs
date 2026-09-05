    fn update_settings_status() {
        let Some(lock) = STATE.get() else {
            return;
        };
        let Ok(state) = lock.lock() else {
            return;
        };
        let hwnd = state.config_hwnd;
        if hwnd == 0 {
            return;
        }
        let status = format!(
            "CPU: {:.1}%    RAM: {:.1}%    Power: {}    State: {}",
            state.cpu_percent,
            state.ram_percent,
            if state.on_battery { "Battery" } else { "AC" },
            if state.battery_paused {
                "Paused on battery"
            } else if state.is_idle && state.settings.sleep_idle {
                "Sleeping"
            } else if state.is_idle {
                "Idle"
            } else {
                "Running"
            }
        );
        drop(state);
        unsafe {
            if IsWindow(hwnd) != 0 {
                set_control_text(hwnd, CFG_STATUS, &status);
            }
        }
    }

    fn sync_settings_window() {
        let Some(lock) = STATE.get() else {
            return;
        };
        let Ok(state) = lock.lock() else {
            return;
        };
        let hwnd = state.config_hwnd;
        if hwnd == 0 {
            return;
        }
        let settings = state.settings;
        drop(state);

        unsafe {
            if IsWindow(hwnd) == 0 {
                return;
            }

            let theme_combo = GetDlgItem(hwnd, CFG_THEME as i32);
            if theme_combo != 0 {
                let selection = match settings.theme {
                    ThemeMode::Auto => 0,
                    ThemeMode::Light => 1,
                    ThemeMode::Dark => 2,
                };
                SendMessageW(theme_combo, CB_SETCURSEL, selection, 0);
            }

            let curve_combo = GetDlgItem(hwnd, CFG_CURVE as i32);
            if curve_combo != 0 {
                let selection = match settings.speed_curve {
                    SpeedCurve::Smooth => 0,
                    SpeedCurve::Linear => 1,
                    SpeedCurve::Reactive => 2,
                };
                SendMessageW(curve_combo, CB_SETCURSEL, selection, 0);
            }

            set_checkbox(hwnd, CFG_STARTUP, startup_enabled());
            set_control_text(hwnd, CFG_SPEED, &format!("{:.2}", settings.speed_multiplier));
            set_control_text(hwnd, CFG_SIZE, &settings.size_px.to_string());
            set_control_text(hwnd, CFG_THRESHOLD, &format!("{:.1}", settings.idle_threshold));
            set_control_text(hwnd, CFG_HYSTERESIS, &format!("{:.1}", settings.idle_hysteresis));
            set_control_text(hwnd, CFG_SAMPLE, &settings.cpu_sample_ms.to_string());
            set_checkbox(hwnd, CFG_SMOOTH, settings.smooth_speed);
            set_checkbox(hwnd, CFG_INVERT, settings.invert_speed);
            set_checkbox(hwnd, CFG_SLEEP, settings.sleep_idle);
            set_checkbox(hwnd, CFG_TOOLTIP_CPU, settings.tooltip_cpu);
            set_checkbox(hwnd, CFG_TOOLTIP_RAM, settings.tooltip_ram);
            set_checkbox(hwnd, CFG_BATTERY_PAUSE, settings.pause_on_battery);
            set_checkbox(hwnd, CFG_OVERLAY, settings.overlay_mode);
        }
        update_settings_status();
    }

    fn parse_decimal_input(text: &str) -> Option<f64> {
        text.trim().replace(',', ".").parse::<f64>().ok()
    }

    fn apply_settings_from_window(hwnd: Hwnd) {
        let theme_selection = unsafe { SendMessageW(GetDlgItem(hwnd, CFG_THEME as i32), CB_GETCURSEL, 0, 0) };
        let curve_selection = unsafe { SendMessageW(GetDlgItem(hwnd, CFG_CURVE as i32), CB_GETCURSEL, 0, 0) };
        let startup = unsafe { checkbox_checked(hwnd, CFG_STARTUP) };
        let speed_text = unsafe { get_control_text(hwnd, CFG_SPEED) };
        let size_text = unsafe { get_control_text(hwnd, CFG_SIZE) };
        let threshold_text = unsafe { get_control_text(hwnd, CFG_THRESHOLD) };
        let hysteresis_text = unsafe { get_control_text(hwnd, CFG_HYSTERESIS) };
        let sample_text = unsafe { get_control_text(hwnd, CFG_SAMPLE) };
        let smooth = unsafe { checkbox_checked(hwnd, CFG_SMOOTH) };
        let invert = unsafe { checkbox_checked(hwnd, CFG_INVERT) };
        let sleep_idle = unsafe { checkbox_checked(hwnd, CFG_SLEEP) };
        let tooltip_cpu = unsafe { checkbox_checked(hwnd, CFG_TOOLTIP_CPU) };
        let tooltip_ram = unsafe { checkbox_checked(hwnd, CFG_TOOLTIP_RAM) };
        let pause_on_battery = unsafe { checkbox_checked(hwnd, CFG_BATTERY_PAUSE) };
        let overlay_mode = unsafe { checkbox_checked(hwnd, CFG_OVERLAY) };

        let Some(speed_multiplier) = parse_decimal_input(&speed_text) else {
            unsafe { warn_settings(hwnd, "Speed multiplier must be a number between 0.10 and 5.00.") };
            return;
        };
        if !(0.10..=5.0).contains(&speed_multiplier) {
            unsafe { warn_settings(hwnd, "Speed multiplier must be between 0.10 and 5.00.") };
            return;
        }

        let Ok(size_px) = size_text.trim().parse::<u32>() else {
            unsafe { warn_settings(hwnd, "Cat size must be an integer between 12 and 64 pixels.") };
            return;
        };
        if !(12..=64).contains(&size_px) {
            unsafe { warn_settings(hwnd, "Cat size must be between 12 and 64 pixels.") };
            return;
        }

        let Some(idle_threshold) = parse_decimal_input(&threshold_text) else {
            unsafe { warn_settings(hwnd, "Idle threshold must be a number between 0 and 100 percent.") };
            return;
        };
        if !(0.0..=100.0).contains(&idle_threshold) {
            unsafe { warn_settings(hwnd, "Idle threshold must be between 0 and 100 percent.") };
            return;
        }

        let Some(idle_hysteresis) = parse_decimal_input(&hysteresis_text) else {
            unsafe { warn_settings(hwnd, "Idle hysteresis must be a number between 0 and 25 percent.") };
            return;
        };
        if !(0.0..=25.0).contains(&idle_hysteresis) {
            unsafe { warn_settings(hwnd, "Idle hysteresis must be between 0 and 25 percent.") };
            return;
        }

        let Ok(cpu_sample_ms) = sample_text.trim().parse::<u32>() else {
            unsafe { warn_settings(hwnd, "CPU sample interval must be an integer between 250 and 5000 ms.") };
            return;
        };
        if !(250..=5000).contains(&cpu_sample_ms) {
            unsafe { warn_settings(hwnd, "CPU sample interval must be between 250 and 5000 ms.") };
            return;
        }

        let theme = match theme_selection {
            1 => ThemeMode::Light,
            2 => ThemeMode::Dark,
            _ => ThemeMode::Auto,
        };
        let speed_curve = match curve_selection {
            1 => SpeedCurve::Linear,
            2 => SpeedCurve::Reactive,
            _ => SpeedCurve::Smooth,
        };

        if !set_startup_enabled(startup) && startup != startup_enabled() {
            unsafe { warn_settings(hwnd, "Windows startup setting could not be changed.") };
            return;
        }

        let Some(lock) = STATE.get() else {
            return;
        };
        let Ok(mut state) = lock.lock() else {
            return;
        };

        let visual_changed = state.settings.theme != theme || state.settings.size_px != size_px;
        let sample_changed = state.settings.cpu_sample_ms != cpu_sample_ms;
        state.settings = Settings {
            theme,
            speed_multiplier,
            speed_curve,
            size_px,
            idle_threshold,
            idle_hysteresis,
            cpu_sample_ms,
            smooth_speed: smooth,
            invert_speed: invert,
            sleep_idle,
            tooltip_cpu,
            tooltip_ram,
            pause_on_battery,
            overlay_mode,
        };
        state.settings.save();

        unsafe {
            if sample_changed {
                KillTimer(state.hwnd, TIMER_CPU);
                SetTimer(state.hwnd, TIMER_CPU, state.settings.cpu_sample_ms, null());
            }
        }
        if visual_changed {
            let _ = rebuild_visuals(&mut state, true);
        }
        apply_behavior(&mut state, true);
        drop(state);
        sync_settings_window();
    }

    unsafe fn initialize_settings_controls(hwnd: Hwnd) {
        let font = GetStockObject(DEFAULT_GUI_FONT);
        let label_style = WS_CHILD | WS_VISIBLE;
        let edit_style = WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_BORDER | ES_AUTOHSCROLL;
        let check_style = WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_AUTOCHECKBOX;
        let button_style = WS_CHILD | WS_VISIBLE | WS_TABSTOP;

        create_control(hwnd, "STATIC", "CatCPU", label_style, 24, 18, 500, 24, 0, font);
        create_control(hwnd, "STATIC", "Native tray cat settings. Changes apply immediately.", label_style, 24, 44, 500, 22, 0, font);
        create_control(hwnd, "STATIC", "", label_style, 24, 74, 500, 20, CFG_STATUS, font);

        create_control(hwnd, "STATIC", "Theme", label_style, 24, 112, 190, 22, 0, font);
        let theme_combo = create_control(hwnd, "COMBOBOX", "", WS_CHILD | WS_VISIBLE | WS_TABSTOP | CBS_DROPDOWNLIST, 240, 108, 250, 160, CFG_THEME, font);
        if theme_combo != 0 {
            for item in ["Automatic", "Light taskbar / black cat", "Dark taskbar / white cat"] {
                let item = wide(item);
                SendMessageW(theme_combo, CB_ADDSTRING, 0, item.as_ptr() as Lparam);
            }
        }

        create_control(hwnd, "BUTTON", "Start with Windows", check_style, 24, 146, 300, 24, CFG_STARTUP, font);

        create_control(hwnd, "STATIC", "Speed multiplier", label_style, 24, 190, 190, 22, 0, font);
        create_control(hwnd, "EDIT", "", edit_style, 240, 186, 100, 26, CFG_SPEED, font);
        create_control(hwnd, "STATIC", "0.10 - 5.00 x", label_style, 352, 190, 138, 22, 0, font);

        create_control(hwnd, "STATIC", "Speed curve", label_style, 24, 226, 190, 22, 0, font);
        let curve_combo = create_control(hwnd, "COMBOBOX", "", WS_CHILD | WS_VISIBLE | WS_TABSTOP | CBS_DROPDOWNLIST, 240, 222, 250, 120, CFG_CURVE, font);
        if curve_combo != 0 {
            for item in ["Smooth", "Linear", "Reactive"] {
                let item = wide(item);
                SendMessageW(curve_combo, CB_ADDSTRING, 0, item.as_ptr() as Lparam);
            }
        }

        create_control(hwnd, "STATIC", "Cat size", label_style, 24, 262, 190, 22, 0, font);
        create_control(hwnd, "EDIT", "", edit_style, 240, 258, 100, 26, CFG_SIZE, font);
        create_control(hwnd, "STATIC", "12 - 64 px", label_style, 352, 262, 138, 22, 0, font);

        create_control(hwnd, "STATIC", "Idle / sleep threshold", label_style, 24, 298, 190, 22, 0, font);
        create_control(hwnd, "EDIT", "", edit_style, 240, 294, 100, 26, CFG_THRESHOLD, font);
        create_control(hwnd, "STATIC", "0 - 100 %", label_style, 352, 298, 138, 22, 0, font);

        create_control(hwnd, "STATIC", "Idle hysteresis", label_style, 24, 334, 190, 22, 0, font);
        create_control(hwnd, "EDIT", "", edit_style, 240, 330, 100, 26, CFG_HYSTERESIS, font);
        create_control(hwnd, "STATIC", "0 - 25 %", label_style, 352, 334, 138, 22, 0, font);

        create_control(hwnd, "STATIC", "CPU sample interval", label_style, 24, 370, 190, 22, 0, font);
        create_control(hwnd, "EDIT", "", edit_style, 240, 366, 100, 26, CFG_SAMPLE, font);
        create_control(hwnd, "STATIC", "250 - 5000 ms", label_style, 352, 370, 138, 22, 0, font);

        create_control(hwnd, "BUTTON", "Smooth speed changes", check_style, 24, 410, 220, 24, CFG_SMOOTH, font);
        create_control(hwnd, "BUTTON", "Invert CPU / speed", check_style, 270, 410, 210, 24, CFG_INVERT, font);
        create_control(hwnd, "BUTTON", "Sleeping cat when idle", check_style, 24, 442, 220, 24, CFG_SLEEP, font);
        create_control(hwnd, "BUTTON", "Pause animation on battery", check_style, 270, 442, 220, 24, CFG_BATTERY_PAUSE, font);
        create_control(hwnd, "BUTTON", "Tooltip: CPU", check_style, 24, 474, 160, 24, CFG_TOOLTIP_CPU, font);
        create_control(hwnd, "BUTTON", "Tooltip: RAM", check_style, 190, 474, 160, 24, CFG_TOOLTIP_RAM, font);
        create_control(hwnd, "BUTTON", "Large overlay for sizes above 32 px", check_style, 24, 506, 330, 24, CFG_OVERLAY, font);
        create_control(hwnd, "STATIC", "Windows controls the physical tray icon size; overlay enables 33-64 px display.", label_style, 24, 536, 500, 22, 0, font);

        create_control(hwnd, "BUTTON", "Apply", button_style, 224, 574, 90, 30, CFG_APPLY, font);
        create_control(hwnd, "BUTTON", "Reset", button_style, 324, 574, 80, 30, CFG_RESET, font);
        create_control(hwnd, "BUTTON", "Close", button_style, 414, 574, 80, 30, CFG_CLOSE, font);
    }
