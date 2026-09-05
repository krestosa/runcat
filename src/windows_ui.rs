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
                    WM_RBUTTONUP => launch_tray_quick_settings(hwnd),
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
