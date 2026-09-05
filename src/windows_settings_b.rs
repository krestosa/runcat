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
        let sleeping = state.is_idle && state.settings.sleep_idle;
        let status = format!(
            "CPU: {:.1}%    State: {}",
            state.cpu_percent,
            if sleeping {
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

            let combo = GetDlgItem(hwnd, CFG_THEME as i32);
            if combo != 0 {
                let selection = match settings.theme {
                    ThemeMode::Auto => 0,
                    ThemeMode::Light => 1,
                    ThemeMode::Dark => 2,
                };
                SendMessageW(combo, CB_SETCURSEL, selection, 0);
            }

            set_checkbox(hwnd, CFG_STARTUP, startup_enabled());
            set_control_text(hwnd, CFG_SPEED, &format!("{:.2}", settings.speed_multiplier));
            set_control_text(hwnd, CFG_SIZE, &settings.size_px.to_string());
            set_control_text(hwnd, CFG_THRESHOLD, &format!("{:.1}", settings.idle_threshold));
            set_control_text(hwnd, CFG_SAMPLE, &settings.cpu_sample_ms.to_string());
            set_checkbox(hwnd, CFG_SMOOTH, settings.smooth_speed);
            set_checkbox(hwnd, CFG_INVERT, settings.invert_speed);
            set_checkbox(hwnd, CFG_SLEEP, settings.sleep_idle);
        }
        update_settings_status();
    }

    fn parse_decimal_input(text: &str) -> Option<f64> {
        text.trim().replace(',', ".").parse::<f64>().ok()
    }

    fn apply_settings_from_window(hwnd: Hwnd) {
        let (
            theme_selection,
            startup,
            speed_text,
            size_text,
            threshold_text,
            sample_text,
            smooth,
            invert,
            sleep_idle,
        ) = unsafe {
            (
                SendMessageW(GetDlgItem(hwnd, CFG_THEME as i32), CB_GETCURSEL, 0, 0),
                checkbox_checked(hwnd, CFG_STARTUP),
                get_control_text(hwnd, CFG_SPEED),
                get_control_text(hwnd, CFG_SIZE),
                get_control_text(hwnd, CFG_THRESHOLD),
                get_control_text(hwnd, CFG_SAMPLE),
                checkbox_checked(hwnd, CFG_SMOOTH),
                checkbox_checked(hwnd, CFG_INVERT),
                checkbox_checked(hwnd, CFG_SLEEP),
            )
        };

        let Some(speed_multiplier) = parse_decimal_input(&speed_text) else {
            unsafe { warn_settings(hwnd, "Speed multiplier must be a number between 0.10 and 5.00.") };
            return;
        };
        if !(0.10..=5.0).contains(&speed_multiplier) {
            unsafe { warn_settings(hwnd, "Speed multiplier must be between 0.10 and 5.00.") };
            return;
        }

        let Ok(size_px) = size_text.trim().parse::<u32>() else {
            unsafe { warn_settings(hwnd, "Cat size must be an integer between 12 and 32 pixels.") };
            return;
        };
        if !(12..=32).contains(&size_px) {
            unsafe { warn_settings(hwnd, "Cat size must be between 12 and 32 pixels.") };
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
            size_px,
            idle_threshold,
            cpu_sample_ms,
            smooth_speed: smooth,
            invert_speed: invert,
            sleep_idle,
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

        create_control(hwnd, "STATIC", "CatCPU", label_style, 24, 18, 460, 24, 0, font);
        create_control(
            hwnd,
            "STATIC",
            "Native tray cat settings. Changes are applied without restarting.",
            label_style,
            24,
            44,
            460,
            22,
            0,
            font,
        );
        create_control(hwnd, "STATIC", "", label_style, 24, 74, 460, 20, CFG_STATUS, font);

        create_control(hwnd, "STATIC", "Theme", label_style, 24, 112, 190, 22, 0, font);
        let combo = create_control(
            hwnd,
            "COMBOBOX",
            "",
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | CBS_DROPDOWNLIST,
            240,
            108,
            240,
            160,
            CFG_THEME,
            font,
        );
        if combo != 0 {
            for item in ["Automatic", "Light taskbar / black cat", "Dark taskbar / white cat"] {
                let item = wide(item);
                SendMessageW(combo, CB_ADDSTRING, 0, item.as_ptr() as Lparam);
            }
        }

        create_control(
            hwnd,
            "BUTTON",
            "Start with Windows",
            check_style,
            24,
            146,
            300,
            24,
            CFG_STARTUP,
            font,
        );

        create_control(hwnd, "STATIC", "Speed multiplier", label_style, 24, 190, 190, 22, 0, font);
        create_control(hwnd, "EDIT", "", edit_style, 240, 186, 100, 26, CFG_SPEED, font);
        create_control(hwnd, "STATIC", "0.10 - 5.00 x", label_style, 352, 190, 128, 22, 0, font);

        create_control(hwnd, "STATIC", "Cat size", label_style, 24, 226, 190, 22, 0, font);
        create_control(hwnd, "EDIT", "", edit_style, 240, 222, 100, 26, CFG_SIZE, font);
        create_control(hwnd, "STATIC", "12 - 32 px", label_style, 352, 226, 128, 22, 0, font);

        create_control(hwnd, "STATIC", "Idle / sleep threshold", label_style, 24, 262, 190, 22, 0, font);
        create_control(hwnd, "EDIT", "", edit_style, 240, 258, 100, 26, CFG_THRESHOLD, font);
        create_control(hwnd, "STATIC", "0 - 100 %", label_style, 352, 262, 128, 22, 0, font);

        create_control(hwnd, "STATIC", "CPU sample interval", label_style, 24, 298, 190, 22, 0, font);
        create_control(hwnd, "EDIT", "", edit_style, 240, 294, 100, 26, CFG_SAMPLE, font);
        create_control(hwnd, "STATIC", "250 - 5000 ms", label_style, 352, 298, 128, 22, 0, font);

        create_control(
            hwnd,
            "BUTTON",
            "Smooth speed changes",
            check_style,
            24,
            340,
            220,
            24,
            CFG_SMOOTH,
            font,
        );
        create_control(
            hwnd,
            "BUTTON",
            "Invert CPU / speed",
            check_style,
            260,
            340,
            200,
            24,
            CFG_INVERT,
            font,
        );
        create_control(
            hwnd,
            "BUTTON",
            "Show sleeping cat when CPU is at/below threshold",
            check_style,
            24,
            372,
            420,
            24,
            CFG_SLEEP,
            font,
        );

        create_control(hwnd, "BUTTON", "Apply", button_style, 214, 422, 90, 30, CFG_APPLY, font);
        create_control(hwnd, "BUTTON", "Reset", button_style, 314, 422, 80, 30, CFG_RESET, font);
        create_control(hwnd, "BUTTON", "Close", button_style, 404, 422, 80, 30, CFG_CLOSE, font);
    }
