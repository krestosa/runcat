    fn restart_animation_timer(state: &mut AppState, interval: Uint) {
        unsafe {
            KillTimer(state.hwnd, TIMER_ANIMATION);
            if !state.is_idle && !state.battery_paused {
                SetTimer(state.hwnd, TIMER_ANIMATION, interval.max(1), null());
            }
        }
    }

    fn current_icon(state: &AppState) -> Hicon {
        if (state.is_idle || state.battery_paused) && state.settings.sleep_idle {
            state.sleep_icon
        } else {
            state.icons[state.frame]
        }
    }

    fn apply_behavior(state: &mut AppState, immediate: bool) {
        state.target_animation_ms = target_frame_interval(state.cpu_percent, state.settings);
        if immediate || !state.settings.smooth_speed {
            state.animation_ms = state.target_animation_ms.round().clamp(1.0, u32::MAX as f64) as Uint;
        }

        state.battery_paused = state.settings.pause_on_battery && state.on_battery;
        let next_idle = should_idle(state.cpu_percent, state.settings, state.is_idle);
        if next_idle || state.battery_paused {
            state.is_idle = next_idle;
            state.frame = 0;
            unsafe {
                KillTimer(state.hwnd, TIMER_ANIMATION);
            }
            update_tray_icon(state.hwnd, current_icon(state));
        } else {
            let was_idle = state.is_idle;
            state.is_idle = false;
            update_tray_icon(state.hwnd, current_icon(state));
            if was_idle || immediate || !state.settings.smooth_speed {
                let interval = state.animation_ms;
                restart_animation_timer(state, interval);
            }
        }
        update_tray_tooltip(state);
        sync_overlay(state);
    }

    fn rebuild_visuals(state: &mut AppState, force: bool) -> Result<(), &'static str> {
        let next_light = effective_light_theme(state.settings);
        let theme_changed = next_light != state.effective_light_theme;
        if !force && !theme_changed {
            return Ok(());
        }

        let new_icons = build_icon_set(
            &state.source_frames,
            next_light,
            state.settings.size_px,
        )?;
        let new_sleep = match create_icon(&state.source_sleep, alpha_bounds(&state.source_sleep), next_light, state.settings.size_px) {
            Ok(icon) => icon,
            Err(error) => {
                destroy_icon_set(&new_icons);
                return Err(error);
            }
        };
        let old_icons = std::mem::replace(&mut state.icons, new_icons);
        let old_sleep = std::mem::replace(&mut state.sleep_icon, new_sleep);
        state.effective_light_theme = next_light;
        update_tray_icon(state.hwnd, current_icon(state));
        update_tray_tooltip(state);
        sync_overlay(state);
        destroy_icon_set(&old_icons);
        unsafe {
            if old_sleep != 0 {
                DestroyIcon(old_sleep);
            }
        }
        Ok(())
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
        font: isize,
    ) -> Hwnd {
        let class_name = wide(class_name);
        let text = wide(text);
        let instance = GetModuleHandleW(null());
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            text.as_ptr(),
            style,
            x,
            y,
            width,
            height,
            parent,
            id as Hmenu,
            instance,
            null_mut(),
        );
        if hwnd != 0 && font != 0 {
            SendMessageW(hwnd, WM_SETFONT, font as Wparam, 1);
        }
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
        if control == 0 {
            return String::new();
        }
        let len = GetWindowTextLengthW(control);
        if len <= 0 {
            return String::new();
        }
        let mut buffer = vec![0u16; len as usize + 1];
        let read = GetWindowTextW(control, buffer.as_mut_ptr(), buffer.len() as i32);
        String::from_utf16_lossy(&buffer[..read.max(0) as usize])
    }

    unsafe fn set_checkbox(parent: Hwnd, id: usize, checked: bool) {
        let control = GetDlgItem(parent, id as i32);
        if control != 0 {
            SendMessageW(control, BM_SETCHECK, if checked { BST_CHECKED } else { 0 }, 0);
        }
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
