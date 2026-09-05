    const SETTINGS_CHANGED_MESSAGE: Uint = WM_APP + 50;

    fn launch_modern_settings(owner: Hwnd) {
        let executable = std::env::current_exe()
            .ok()
            .map(|path| path.with_file_name("catcpu-settings.exe"));
        let Some(executable) = executable else {
            unsafe { warn_settings(owner, "Could not resolve catcpu-settings.exe."); }
            return;
        };

        if std::process::Command::new(&executable).spawn().is_err() {
            unsafe {
                warn_settings(
                    owner,
                    "Could not open the WinUI Settings app. Keep catcpu-settings.exe next to catcpu.exe.",
                );
            }
        }
    }

    fn apply_external_settings_update() {
        let next = Settings::load();
        let Some(lock) = STATE.get() else {
            return;
        };
        let Ok(mut state) = lock.lock() else {
            return;
        };

        let previous = state.settings;
        state.settings = next;

        let visual_changed = previous.theme != next.theme
            || previous.size_px != next.size_px
            || previous.overlay_mode != next.overlay_mode;
        let sample_changed = previous.cpu_sample_ms != next.cpu_sample_ms;

        unsafe {
            if sample_changed {
                KillTimer(state.hwnd, TIMER_CPU);
                SetTimer(state.hwnd, TIMER_CPU, next.cpu_sample_ms, null());
            }
        }

        if visual_changed {
            let _ = rebuild_visuals(&mut state, true);
        }
        sample_runtime_metrics(&mut state);
        apply_behavior(&mut state, true);
        update_tray_tooltip_if_changed(&mut state, true);
    }
