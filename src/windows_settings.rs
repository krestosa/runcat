    unsafe fn warn_settings(hwnd: Hwnd, message: &str) {
        let message = wide(message);
        let title = wide("CatCPU Settings");
        MessageBoxW(hwnd, message.as_ptr(), title.as_ptr(), MB_OK | MB_ICONWARNING);
    }

    fn reset_app_settings() {
        let Some(lock) = STATE.get() else {
            return;
        };
        let Ok(mut state) = lock.lock() else {
            return;
        };

        state.settings = Settings::default();
        let _ = state.settings.save();
        unsafe {
            KillTimer(state.hwnd, TIMER_CPU);
            SetTimer(state.hwnd, TIMER_CPU, state.settings.cpu_sample_ms, null());
        }
        let _ = rebuild_visuals(&mut state, true);
        sample_runtime_metrics(&mut state);
        apply_behavior(&mut state, true);
        update_tray_tooltip_if_changed(&mut state, true);
    }
