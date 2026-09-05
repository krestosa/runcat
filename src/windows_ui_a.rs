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
            let class_name = wide("CatCPU.SettingsWindow");
            let title = wide("CatCPU Settings");
            let instance = GetModuleHandleW(null());
            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                530,
                505,
                owner,
                0,
                instance,
                null_mut(),
            );
            if hwnd == 0 {
                return;
            }

            if let Some(lock) = STATE.get() {
                if let Ok(mut state) = lock.lock() {
                    state.config_hwnd = hwnd;
                }
            }

            initialize_settings_controls(hwnd);
            sync_settings_window();
            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);
            SetForegroundWindow(hwnd);
        }
    }

    unsafe extern "system" fn settings_wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        match msg {
            WM_COMMAND => {
                let id = w_param & 0xffff;
                match id {
                    CFG_APPLY => apply_settings_from_window(hwnd),
                    CFG_RESET => {
                        if let Some(lock) = STATE.get() {
                            if let Ok(mut state) = lock.lock() {
                                state.settings = Settings::default();
                                state.settings.save();
                                KillTimer(state.hwnd, TIMER_CPU);
                                SetTimer(state.hwnd, TIMER_CPU, state.settings.cpu_sample_ms, null());
                                let _ = rebuild_visuals(&mut state, true);
                                apply_behavior(&mut state, true);
                            }
                        }
                        let _ = set_startup_enabled(false);
                        sync_settings_window();
                    }
                    CFG_CLOSE => {
                        DestroyWindow(hwnd);
                    }
                    _ => {}
                }
                return 0;
            }
            WM_CLOSE => {
                DestroyWindow(hwnd);
                return 0;
            }
            WM_DESTROY => {
                if let Some(lock) = STATE.get() {
                    if let Ok(mut state) = lock.lock() {
                        if state.config_hwnd == hwnd {
                            state.config_hwnd = 0;
                        }
                    }
                }
                return 0;
            }
            _ => {}
        }
        DefWindowProcW(hwnd, msg, w_param, l_param)
    }

    fn approx(value: f64, target: f64) -> bool {
        (value - target).abs() < 0.001
    }

    fn handle_menu_command(hwnd: Hwnd, command: usize) {
        if command == MENU_EXIT {
            unsafe {
                PostMessageW(hwnd, WM_CLOSE, 0, 0);
            }
            return;
        }

        if command == MENU_SETTINGS {
            open_settings_window(hwnd);
            return;
        }

        if command == MENU_STARTUP {
            let _ = set_startup_enabled(!startup_enabled());
            sync_settings_window();
            return;
        }

        let Some(lock) = STATE.get() else {
            return;
        };
        let Ok(mut state) = lock.lock() else {
            return;
        };

        let previous_theme = state.settings.theme;
        let previous_size = state.settings.size_px;
        let mut behavior_changed = false;
        let mut force_behavior = false;

        match command {
            MENU_THEME_AUTO => state.settings.theme = ThemeMode::Auto,
            MENU_THEME_LIGHT => state.settings.theme = ThemeMode::Light,
            MENU_THEME_DARK => state.settings.theme = ThemeMode::Dark,
            MENU_SPEED_HALF => {
                state.settings.speed_multiplier = 0.5;
                behavior_changed = true;
            }
            MENU_SPEED_NORMAL => {
                state.settings.speed_multiplier = 1.0;
                behavior_changed = true;
            }
            MENU_SPEED_FAST => {
                state.settings.speed_multiplier = 1.5;
                behavior_changed = true;
            }
            MENU_SPEED_FASTER => {
                state.settings.speed_multiplier = 2.0;
                behavior_changed = true;
            }
            MENU_SIZE_COMPACT => state.settings.size_px = 20,
            MENU_SIZE_NORMAL => state.settings.size_px = 26,
            MENU_SIZE_FULL => state.settings.size_px = 32,
            MENU_IDLE_OFF => {
                state.settings.idle_threshold = 0.0;
                behavior_changed = true;
                force_behavior = true;
            }
            MENU_IDLE_5 => {
                state.settings.idle_threshold = 5.0;
                behavior_changed = true;
                force_behavior = true;
            }
            MENU_IDLE_10 => {
                state.settings.idle_threshold = 10.0;
                behavior_changed = true;
                force_behavior = true;
            }
            MENU_IDLE_20 => {
                state.settings.idle_threshold = 20.0;
                behavior_changed = true;
                force_behavior = true;
            }
            MENU_SMOOTH => {
                state.settings.smooth_speed = !state.settings.smooth_speed;
                behavior_changed = true;
                force_behavior = !state.settings.smooth_speed;
            }
            MENU_INVERT => {
                state.settings.invert_speed = !state.settings.invert_speed;
                behavior_changed = true;
                force_behavior = true;
            }
            MENU_SLEEP_IDLE => {
                state.settings.sleep_idle = !state.settings.sleep_idle;
                behavior_changed = true;
                force_behavior = true;
            }
            MENU_RESET => {
                state.settings = Settings::default();
                state.settings.save();
                unsafe {
                    KillTimer(state.hwnd, TIMER_CPU);
                    SetTimer(state.hwnd, TIMER_CPU, state.settings.cpu_sample_ms, null());
                }
                let _ = rebuild_visuals(&mut state, true);
                apply_behavior(&mut state, true);
                drop(state);
                sync_settings_window();
                return;
            }
            _ => return,
        }

        state.settings.save();

        if state.settings.theme != previous_theme || state.settings.size_px != previous_size {
            let _ = rebuild_visuals(&mut state, true);
        }
        if behavior_changed {
            apply_behavior(&mut state, force_behavior);
        }
        drop(state);
        sync_settings_window();
    }
