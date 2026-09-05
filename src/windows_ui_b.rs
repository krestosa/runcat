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
            if menu == 0 || theme_menu == 0 || speed_menu == 0 || size_menu == 0 || behavior_menu == 0 || idle_menu == 0 {
                for handle in [menu, theme_menu, speed_menu, size_menu, behavior_menu, idle_menu] {
                    if handle != 0 {
                        DestroyMenu(handle);
                    }
                }
                return;
            }

            append_item(menu, MENU_SETTINGS, "Settings...", false);
            AppendMenuW(menu, MF_SEPARATOR, 0, null());
            append_item(menu, MENU_STARTUP, "Start with Windows", startup);

            append_radio(theme_menu, MENU_THEME_AUTO, "Automatic", settings.theme == ThemeMode::Auto);
            append_radio(theme_menu, MENU_THEME_LIGHT, "Light taskbar (black cat)", settings.theme == ThemeMode::Light);
            append_radio(theme_menu, MENU_THEME_DARK, "Dark taskbar (white cat)", settings.theme == ThemeMode::Dark);
            append_submenu(menu, theme_menu, "Theme");

            append_radio(speed_menu, MENU_SPEED_HALF, "0.5x", approx(settings.speed_multiplier, 0.5));
            append_radio(speed_menu, MENU_SPEED_NORMAL, "1x", approx(settings.speed_multiplier, 1.0));
            append_radio(speed_menu, MENU_SPEED_FAST, "1.5x", approx(settings.speed_multiplier, 1.5));
            append_radio(speed_menu, MENU_SPEED_FASTER, "2x", approx(settings.speed_multiplier, 2.0));
            append_submenu(menu, speed_menu, "Speed");

            append_radio(size_menu, MENU_SIZE_COMPACT, "20 px", settings.size_px == 20);
            append_radio(size_menu, MENU_SIZE_NORMAL, "26 px", settings.size_px == 26);
            append_radio(size_menu, MENU_SIZE_FULL, "32 px", settings.size_px == 32);
            append_radio(size_menu, MENU_SIZE_LARGE, "48 px", settings.size_px == 48);
            append_radio(size_menu, MENU_SIZE_XLARGE, "64 px", settings.size_px == 64);
            append_submenu(menu, size_menu, "Cat size");

            append_radio(idle_menu, MENU_IDLE_OFF, "0%", approx(settings.idle_threshold, 0.0));
            append_radio(idle_menu, MENU_IDLE_5, "5%", approx(settings.idle_threshold, 5.0));
            append_radio(idle_menu, MENU_IDLE_10, "10%", approx(settings.idle_threshold, 10.0));
            append_radio(idle_menu, MENU_IDLE_20, "20%", approx(settings.idle_threshold, 20.0));
            append_submenu(behavior_menu, idle_menu, "Idle threshold");
            append_item(behavior_menu, MENU_SMOOTH, "Smooth speed changes", settings.smooth_speed);
            append_item(behavior_menu, MENU_INVERT, "Invert CPU / speed", settings.invert_speed);
            append_item(behavior_menu, MENU_SLEEP_IDLE, "Sleeping cat when idle", settings.sleep_idle);
            append_item(behavior_menu, MENU_BATTERY_PAUSE, "Pause animation on battery", settings.pause_on_battery);
            append_item(behavior_menu, MENU_OVERLAY, "Large overlay (>32 px)", settings.overlay_mode);
            append_submenu(menu, behavior_menu, "Behavior");

            AppendMenuW(menu, MF_SEPARATOR, 0, null());
            append_item(menu, MENU_RESET, "Reset settings", false);
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

    unsafe extern "system" fn wnd_proc(hwnd: Hwnd, msg: Uint, w_param: Wparam, l_param: Lparam) -> Lresult {
        if let Some(lock) = STATE.get() {
            let taskbar_created = lock.lock().ok().map(|state| state.taskbar_created).unwrap_or(0);
            if taskbar_created != 0 && msg == taskbar_created {
                if let Ok(state) = lock.lock() {
                    add_tray_icon(state.hwnd, current_icon(&state));
                    update_tray_tooltip(&state);
                    sync_overlay(&state);
                }
                return 0;
            }
        }

        match msg {
            WM_TIMER => {
                if w_param == TIMER_ANIMATION {
                    if let Some(lock) = STATE.get() {
                        if let Ok(mut state) = lock.lock() {
                            if !state.is_idle && !state.battery_paused {
                                state.frame = (state.frame + 1) % FRAME_COUNT;
                                update_tray_icon(state.hwnd, state.icons[state.frame]);
                                sync_overlay(&state);

                                if state.settings.smooth_speed {
                                    let current = state.animation_ms as f64;
                                    let alpha = 1.0 - (-current / SMOOTHING_TAU_MS).exp();
                                    let smoothed = current + alpha * (state.target_animation_ms - current);
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
                            if let Some(cpu) = cpu_usage_and_store(&mut state) {
                                state.cpu_percent = cpu;
                            }
                            if let Some(ram) = ram_usage_percent() {
                                state.ram_percent = ram;
                            }
                            state.on_battery = system_on_battery();
                            apply_behavior(&mut state, false);
                            if state.settings.theme == ThemeMode::Auto {
                                let _ = rebuild_visuals(&mut state, false);
                            }
                        }
                    }
                    update_settings_status();
                    return 0;
                }
            }
            WM_SETTINGCHANGE | WM_THEMECHANGED => {
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
                if l_param as Uint == WM_RBUTTONUP {
                    show_context_menu(hwnd);
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
