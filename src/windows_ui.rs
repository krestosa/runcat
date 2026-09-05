    fn animation_suspended(state: &AppState) -> bool {
        state.is_idle || state.battery_paused || state.settings.manual_pause
    }

    fn restart_animation_timer(state: &mut AppState, interval: Uint) {
        unsafe {
            KillTimer(state.hwnd, TIMER_ANIMATION);
            if !animation_suspended(state) {
                SetTimer(state.hwnd, TIMER_ANIMATION, interval.max(1), null());
            }
        }
    }

    fn apply_behavior(state: &mut AppState, immediate: bool) {
        let old_idle = state.is_idle;
        let old_battery_paused = state.battery_paused;
        let old_suspended = old_idle || old_battery_paused || state.settings.manual_pause;
        let old_interval = state.animation_ms;

        state.target_animation_ms = target_frame_interval(state.cpu_percent, state.settings);
        if immediate || !state.settings.smooth_speed {
            state.animation_ms = state
                .target_animation_ms
                .round()
                .clamp(1.0, u32::MAX as f64) as Uint;
        }

        state.battery_paused = state.settings.pause_on_battery && state.power.on_battery;
        state.is_idle = should_idle(state.cpu_percent, state.settings, state.is_idle);
        let new_suspended = animation_suspended(state);
        let state_changed = old_idle != state.is_idle
            || old_battery_paused != state.battery_paused
            || immediate;

        if new_suspended {
            state.frame = 0;
            unsafe {
                KillTimer(state.hwnd, TIMER_ANIMATION);
            }
        } else if old_suspended
            || immediate
            || (!state.settings.smooth_speed && old_interval != state.animation_ms)
        {
            restart_animation_timer(state, state.animation_ms);
        }

        if state_changed || old_suspended != new_suspended {
            update_tray_icon_if_changed(state, true);
            sync_overlay(state);
        }
        update_tray_tooltip_if_changed(state, false);
    }

    unsafe fn append_item(menu: Hmenu, id: usize, label: &str, checked: bool) {
        let label = wide(label);
        let mut flags = MF_STRING;
        if checked {
            flags |= MF_CHECKED;
        }
        AppendMenuW(menu, flags, id, label.as_ptr());
    }

    unsafe fn append_radio(menu: Hmenu, id: usize, label: &str, checked: bool) {
        let label = wide(label);
        let mut flags = MF_STRING | MF_RADIOCHECK;
        if checked {
            flags |= MF_CHECKED;
        }
        AppendMenuW(menu, flags, id, label.as_ptr());
    }

    unsafe fn append_submenu(menu: Hmenu, submenu: Hmenu, label: &str) {
        let label = wide(label);
        AppendMenuW(menu, MF_STRING | MF_POPUP, submenu as usize, label.as_ptr());
    }

    fn approx(value: f64, target: f64) -> bool {
        (value - target).abs() < 0.001
    }

    fn handle_menu_command(hwnd: Hwnd, command: usize) {
        match command {
            MENU_EXIT => {
                unsafe {
                    PostMessageW(hwnd, WM_CLOSE, 0, 0);
                }
                return;
            }
            MENU_SETTINGS => {
                launch_modern_settings(hwnd);
                return;
            }
            MENU_STARTUP => {
                let _ = set_startup_enabled(!startup_enabled());
                return;
            }
            MENU_RESET => {
                reset_app_settings();
                return;
            }
            _ => {}
        }

        let Some(lock) = STATE.get() else {
            return;
        };
        let Ok(mut state) = lock.lock() else {
            return;
        };

        let previous = state.settings;
        match command {
            MENU_THEME_AUTO => state.settings.theme = ThemeMode::Auto,
            MENU_THEME_LIGHT => state.settings.theme = ThemeMode::Light,
            MENU_THEME_DARK => state.settings.theme = ThemeMode::Dark,
            MENU_SPEED_HALF => state.settings.speed_multiplier = 0.5,
            MENU_SPEED_NORMAL => state.settings.speed_multiplier = 1.0,
            MENU_SPEED_FAST => state.settings.speed_multiplier = 1.5,
            MENU_SPEED_FASTER => state.settings.speed_multiplier = 2.0,
            MENU_SIZE_COMPACT => state.settings.size_px = 20,
            MENU_SIZE_NORMAL => state.settings.size_px = 26,
            MENU_SIZE_FULL => state.settings.size_px = 32,
            MENU_SIZE_LARGE => {
                state.settings.size_px = 48;
                state.settings.overlay_mode = true;
            }
            MENU_SIZE_XLARGE => {
                state.settings.size_px = 64;
                state.settings.overlay_mode = true;
            }
            MENU_IDLE_OFF => state.settings.idle_threshold = 0.0,
            MENU_IDLE_5 => state.settings.idle_threshold = 5.0,
            MENU_IDLE_10 => state.settings.idle_threshold = 10.0,
            MENU_IDLE_20 => state.settings.idle_threshold = 20.0,
            MENU_SMOOTH => state.settings.smooth_speed = !state.settings.smooth_speed,
            MENU_INVERT => state.settings.invert_speed = !state.settings.invert_speed,
            MENU_SLEEP_IDLE => state.settings.sleep_idle = !state.settings.sleep_idle,
            MENU_BATTERY_PAUSE => {
                state.settings.pause_on_battery = !state.settings.pause_on_battery
            }
            MENU_OVERLAY => state.settings.overlay_mode = !state.settings.overlay_mode,
            MENU_PAUSE => state.settings.manual_pause = !state.settings.manual_pause,
            MENU_TOOLTIP_CPU => state.settings.tooltip_cpu = !state.settings.tooltip_cpu,
            MENU_TOOLTIP_RAM => state.settings.tooltip_ram = !state.settings.tooltip_ram,
            MENU_TOOLTIP_BATTERY => {
                state.settings.tooltip_battery = !state.settings.tooltip_battery
            }
            _ => return,
        }

        let visual_changed = previous.theme != state.settings.theme
            || previous.size_px != state.settings.size_px
            || previous.overlay_mode != state.settings.overlay_mode;
        let _ = state.settings.save();

        if visual_changed {
            let _ = rebuild_visuals(&mut state, true);
        }
        apply_behavior(&mut state, true);
        update_tray_tooltip_if_changed(&mut state, true);
    }

    fn show_context_menu(hwnd: Hwnd) {
        let settings = STATE
            .get()
            .and_then(|lock| lock.lock().ok().map(|state| state.settings))
            .unwrap_or_default();
        let startup = startup_enabled();

        unsafe {
            let menu = CreatePopupMenu();
            let theme_menu = CreatePopupMenu();
            let speed_menu = CreatePopupMenu();
            let size_menu = CreatePopupMenu();
            let behavior_menu = CreatePopupMenu();
            let idle_menu = CreatePopupMenu();
            let tooltip_menu = CreatePopupMenu();
            let menus = [
                menu,
                theme_menu,
                speed_menu,
                size_menu,
                behavior_menu,
                idle_menu,
                tooltip_menu,
            ];
            if menus.iter().any(|handle| *handle == 0) {
                for handle in menus.into_iter().filter(|handle| *handle != 0) {
                    DestroyMenu(handle);
                }
                return;
            }

            append_item(menu, MENU_SETTINGS, "Settings…", false);
            append_item(menu, MENU_PAUSE, "Pause animation", settings.manual_pause);
            AppendMenuW(menu, MF_SEPARATOR, 0, null());
            append_item(menu, MENU_STARTUP, "Start with Windows", startup);

            append_radio(
                theme_menu,
                MENU_THEME_AUTO,
                "Automatic",
                settings.theme == ThemeMode::Auto,
            );
            append_radio(
                theme_menu,
                MENU_THEME_LIGHT,
                "Light taskbar (black cat)",
                settings.theme == ThemeMode::Light,
            );
            append_radio(
                theme_menu,
                MENU_THEME_DARK,
                "Dark taskbar (white cat)",
                settings.theme == ThemeMode::Dark,
            );
            append_submenu(menu, theme_menu, "Theme");

            append_radio(
                speed_menu,
                MENU_SPEED_HALF,
                "0.5×",
                approx(settings.speed_multiplier, 0.5),
            );
            append_radio(
                speed_menu,
                MENU_SPEED_NORMAL,
                "1×",
                approx(settings.speed_multiplier, 1.0),
            );
            append_radio(
                speed_menu,
                MENU_SPEED_FAST,
                "1.5×",
                approx(settings.speed_multiplier, 1.5),
            );
            append_radio(
                speed_menu,
                MENU_SPEED_FASTER,
                "2×",
                approx(settings.speed_multiplier, 2.0),
            );
            append_submenu(menu, speed_menu, "Speed");

            append_radio(size_menu, MENU_SIZE_COMPACT, "20 px", settings.size_px == 20);
            append_radio(size_menu, MENU_SIZE_NORMAL, "26 px", settings.size_px == 26);
            append_radio(size_menu, MENU_SIZE_FULL, "32 px", settings.size_px == 32);
            append_radio(
                size_menu,
                MENU_SIZE_LARGE,
                "48 px + overlay",
                settings.size_px == 48,
            );
            append_radio(
                size_menu,
                MENU_SIZE_XLARGE,
                "64 px + overlay",
                settings.size_px == 64,
            );
            append_submenu(menu, size_menu, "Cat size");

            append_radio(idle_menu, MENU_IDLE_OFF, "0%", approx(settings.idle_threshold, 0.0));
            append_radio(idle_menu, MENU_IDLE_5, "5%", approx(settings.idle_threshold, 5.0));
            append_radio(idle_menu, MENU_IDLE_10, "10%", approx(settings.idle_threshold, 10.0));
            append_radio(idle_menu, MENU_IDLE_20, "20%", approx(settings.idle_threshold, 20.0));
            append_submenu(behavior_menu, idle_menu, "Sleep threshold");
            append_item(
                behavior_menu,
                MENU_SMOOTH,
                "Smooth speed transitions",
                settings.smooth_speed,
            );
            append_item(
                behavior_menu,
                MENU_INVERT,
                "Invert CPU / speed",
                settings.invert_speed,
            );
            append_item(
                behavior_menu,
                MENU_SLEEP_IDLE,
                "Sleeping cat when idle",
                settings.sleep_idle,
            );
            append_item(
                behavior_menu,
                MENU_BATTERY_PAUSE,
                "Pause animation on battery",
                settings.pause_on_battery,
            );
            append_item(
                behavior_menu,
                MENU_OVERLAY,
                "Large overlay (>32 px)",
                settings.overlay_mode,
            );
            append_submenu(menu, behavior_menu, "Behavior");

            append_item(tooltip_menu, MENU_TOOLTIP_CPU, "CPU", settings.tooltip_cpu);
            append_item(tooltip_menu, MENU_TOOLTIP_RAM, "RAM", settings.tooltip_ram);
            append_item(
                tooltip_menu,
                MENU_TOOLTIP_BATTERY,
                "Battery",
                settings.tooltip_battery,
            );
            append_submenu(menu, tooltip_menu, "Tooltip");

            AppendMenuW(menu, MF_SEPARATOR, 0, null());
            append_item(menu, MENU_RESET, "Reset app settings", false);
            append_item(menu, MENU_EXIT, "Exit", false);

            let mut point = Point { x: 0, y: 0 };
            if GetCursorPos(&mut point) != 0 {
                SetForegroundWindow(hwnd);
                let command = TrackPopupMenu(
                    menu,
                    TPM_RIGHTBUTTON | TPM_RETURNCMD | TPM_NONOTIFY,
                    point.x,
                    point.y,
                    0,
                    hwnd,
                    null(),
                );
                if command != 0 {
                    handle_menu_command(hwnd, command as usize);
                }
                PostMessageW(hwnd, WM_NULL, 0, 0);
            }

            DestroyMenu(menu);
        }
    }

    fn sample_runtime_metrics(state: &mut AppState) {
        if let Some(cpu) = cpu_usage_and_store(state) {
            state.cpu_percent = cpu;
        }

        if state.settings.tooltip_ram {
            if let Some(ram) = ram_usage_percent() {
                state.ram_percent = ram;
            }
        }

        if state.settings.pause_on_battery || state.settings.tooltip_battery {
            state.power = read_power_status();
        }
    }

    unsafe extern "system" fn wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        if let Some(lock) = STATE.get() {
            let taskbar_created = lock
                .lock()
                .ok()
                .map(|state| state.taskbar_created)
                .unwrap_or(0);
            if taskbar_created != 0 && msg == taskbar_created {
                if let Ok(mut state) = lock.lock() {
                    state.last_tray_icon = 0;
                    state.last_tooltip.clear();
                    let _ = add_tray_icon(&mut state);
                    sync_overlay(&state);
                }
                return 0;
            }
        }

        match msg {
            SETTINGS_CHANGED_MESSAGE => {
                apply_external_settings_update();
                return 0;
            }
            WM_TIMER => {
                if w_param == TIMER_ANIMATION {
                    if let Some(lock) = STATE.get() {
                        if let Ok(mut state) = lock.lock() {
                            if !animation_suspended(&state) {
                                state.frame = (state.frame + 1) % FRAME_COUNT;
                                update_tray_icon_if_changed(&mut state, false);
                                sync_overlay(&state);

                                if state.settings.smooth_speed {
                                    let current = state.animation_ms as f64;
                                    let alpha = 1.0 - (-current / SMOOTHING_TAU_MS).exp();
                                    let smoothed = current
                                        + alpha * (state.target_animation_ms - current);
                                    let next = smoothed.round().clamp(1.0, u32::MAX as f64) as Uint;
                                    if next != state.animation_ms {
                                        state.animation_ms = next;
                                        restart_animation_timer(&mut state, next);
                                    }
                                }
                            }
                        }
                    }
                    return 0;
                }

                if w_param == TIMER_CPU {
                    if let Some(lock) = STATE.get() {
                        if let Ok(mut state) = lock.lock() {
                            sample_runtime_metrics(&mut state);
                            apply_behavior(&mut state, false);
                        }
                    }
                    return 0;
                }
            }
            WM_SETTINGCHANGE | WM_THEMECHANGED | WM_SYSCOLORCHANGE => {
                if let Some(lock) = STATE.get() {
                    if let Ok(mut state) = lock.lock() {
                        if state.settings.theme == ThemeMode::Auto {
                            let _ = rebuild_visuals(&mut state, false);
                        }
                        sync_overlay(&state);
                    }
                }
                return 0;
            }
            TRAY_CALLBACK => {
                match l_param as Uint {
                    WM_RBUTTONUP => show_context_menu(hwnd),
                    WM_LBUTTONUP | WM_LBUTTONDBLCLK => launch_modern_settings(hwnd),
                    _ => {}
                }
                return 0;
            }
            WM_CLOSE => {
                DestroyWindow(hwnd);
                return 0;
            }
            WM_DESTROY => {
                KillTimer(hwnd, TIMER_ANIMATION);
                KillTimer(hwnd, TIMER_CPU);
                remove_tray_icon(hwnd);
                PostQuitMessage(0);
                return 0;
            }
            _ => {}
        }

        DefWindowProcW(hwnd, msg, w_param, l_param)
    }
