    fn run() -> Result<(), &'static str> {
        unsafe {
            let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

            let mutex_name = wide("Local\\CatCPU.Singleton");
            let mutex = CreateMutexW(null(), 0, mutex_name.as_ptr());
            if mutex == 0 {
                return Err("CreateMutexW failed");
            }
            let _instance_guard = OwnedHandle(mutex);
            if GetLastError() == ERROR_ALREADY_EXISTS {
                return Ok(());
            }

            let startup = GdiplusStartupInput {
                version: 1,
                debug_event_callback: null(),
                suppress_background_thread: 0,
                suppress_external_codecs: 0,
            };
            let mut gdiplus_token: UlongPtr = 0;
            if GdiplusStartup(&mut gdiplus_token, &startup, null_mut()) != 0 {
                return Err("GdiplusStartup failed");
            }

            let mut source_frames = Vec::with_capacity(FRAME_COUNT);
            for bytes in CAT_FRAMES {
                match load_frame_pixels(bytes) {
                    Ok(frame) => source_frames.push(frame),
                    Err(error) => {
                        GdiplusShutdown(gdiplus_token);
                        return Err(error);
                    }
                }
            }

            let source_sleep = match load_frame_pixels(SLEEPING_CAT) {
                Ok(frame) => frame,
                Err(error) => {
                    GdiplusShutdown(gdiplus_token);
                    return Err(error);
                }
            };

            let settings = Settings::load();
            let light_theme = effective_light_theme(settings);
            let visuals =
                match build_visuals(&source_frames, &source_sleep, light_theme, settings) {
                    Ok(visuals) => visuals,
                    Err(error) => {
                        GdiplusShutdown(gdiplus_token);
                        return Err(error);
                    }
                };

            let instance = GetModuleHandleW(null());
            if instance == 0 {
                destroy_visuals(&visuals);
                GdiplusShutdown(gdiplus_token);
                return Err("GetModuleHandleW failed");
            }

            let class_name = wide("CatCPU.HiddenWindow");
            let window_name = wide("CatCPU");
            let class = WndClassW {
                style: 0,
                wnd_proc: Some(wnd_proc),
                cls_extra: 0,
                wnd_extra: 0,
                instance,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: null(),
                class_name: class_name.as_ptr(),
            };
            if RegisterClassW(&class) == 0 {
                destroy_visuals(&visuals);
                GdiplusShutdown(gdiplus_token);
                return Err("RegisterClassW failed");
            }

            let settings_class_name = wide("CatCPU.SettingsWindow");
            let settings_class = WndClassW {
                style: 0,
                wnd_proc: Some(mica_settings_wnd_proc),
                cls_extra: 0,
                wnd_extra: 0,
                instance,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: null(),
                class_name: settings_class_name.as_ptr(),
            };
            if RegisterClassW(&settings_class) == 0 {
                destroy_visuals(&visuals);
                GdiplusShutdown(gdiplus_token);
                return Err("settings RegisterClassW failed");
            }

            let overlay_class_name = wide("CatCPU.OverlayWindow");
            let overlay_class = WndClassW {
                style: 0,
                wnd_proc: Some(overlay_wnd_proc),
                cls_extra: 0,
                wnd_extra: 0,
                instance,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: null(),
                class_name: overlay_class_name.as_ptr(),
            };
            if RegisterClassW(&overlay_class) == 0 {
                destroy_visuals(&visuals);
                GdiplusShutdown(gdiplus_token);
                return Err("overlay RegisterClassW failed");
            }

            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                window_name.as_ptr(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                instance,
                null_mut(),
            );
            if hwnd == 0 {
                destroy_visuals(&visuals);
                GdiplusShutdown(gdiplus_token);
                return Err("CreateWindowExW failed");
            }

            let overlay_hwnd = CreateWindowExW(
                WS_EX_LAYERED
                    | WS_EX_TRANSPARENT
                    | WS_EX_TOOLWINDOW
                    | WS_EX_TOPMOST
                    | WS_EX_NOACTIVATE,
                overlay_class_name.as_ptr(),
                window_name.as_ptr(),
                WS_POPUP,
                0,
                0,
                1,
                1,
                0,
                0,
                instance,
                null_mut(),
            );
            if overlay_hwnd == 0 {
                DestroyWindow(hwnd);
                destroy_visuals(&visuals);
                GdiplusShutdown(gdiplus_token);
                return Err("overlay CreateWindowExW failed");
            }

            let (idle, kernel, user) = read_cpu_times().unwrap_or((0, 0, 0));
            let taskbar_created_name = wide("TaskbarCreated");
            let taskbar_created = RegisterWindowMessageW(taskbar_created_name.as_ptr());
            let initial_interval = target_frame_interval(0.0, settings).round() as Uint;
            let power = read_power_status();
            let battery_paused = settings.pause_on_battery && power.on_battery;
            let initial_is_idle = should_idle(0.0, settings, false);

            let state = AppState {
                hwnd,
                overlay_hwnd,
                source_frames,
                source_sleep,
                visuals,
                frame: 0,
                last_idle: idle,
                last_kernel: kernel,
                last_user: user,
                cpu_percent: 0.0,
                ram_percent: ram_usage_percent().unwrap_or(0.0),
                power,
                animation_ms: initial_interval.max(1),
                target_animation_ms: initial_interval.max(1) as f64,
                is_idle: initial_is_idle,
                battery_paused,
                settings,
                effective_light_theme: light_theme,
                taskbar_created,
                gdiplus_token,
                config_hwnd: 0,
                last_tray_icon: 0,
                last_tooltip: String::new(),
                ui_bg_brush: 0,
                ui_surface_brush: 0,
                ui_font: 0,
                ui_header_font: 0,
                ui_light_theme: system_uses_light_apps(),
                ui_dpi: 0,
            };

            if let Err(returned) = STATE.set(Mutex::new(state)) {
                DestroyWindow(overlay_hwnd);
                DestroyWindow(hwnd);
                if let Ok(mut state) = returned.into_inner() {
                    destroy_visuals(&state.visuals);
                    destroy_ui_resources(&mut state);
                    GdiplusShutdown(state.gdiplus_token);
                }
                return Err("state initialization failed");
            }

            let tray_added = if let Some(lock) = STATE.get() {
                if let Ok(mut state) = lock.lock() {
                    let added = add_tray_icon(&mut state);
                    if added {
                        sync_overlay(&state);
                        if !animation_suspended(&state) {
                            SetTimer(hwnd, TIMER_ANIMATION, state.animation_ms, null());
                        }
                    }
                    added
                } else {
                    false
                }
            } else {
                false
            };

            if !tray_added {
                DestroyWindow(overlay_hwnd);
                DestroyWindow(hwnd);
                return Err("Shell_NotifyIconW failed");
            }

            SetTimer(hwnd, TIMER_CPU, settings.cpu_sample_ms, null());

            let mut msg: Msg = zeroed();
            loop {
                let result = GetMessageW(&mut msg, 0, 0, 0);
                if result <= 0 {
                    break;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let (config_hwnd, overlay_hwnd) = if let Some(lock) = STATE.get() {
                if let Ok(state) = lock.lock() {
                    (state.config_hwnd, state.overlay_hwnd)
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            };

            if config_hwnd != 0 && IsWindow(config_hwnd) != 0 {
                DestroyWindow(config_hwnd);
            }
            if overlay_hwnd != 0 && IsWindow(overlay_hwnd) != 0 {
                DestroyWindow(overlay_hwnd);
            }

            if let Some(lock) = STATE.get() {
                if let Ok(mut state) = lock.lock() {
                    destroy_visuals(&state.visuals);
                    destroy_ui_resources(&mut state);
                    GdiplusShutdown(state.gdiplus_token);
                }
            }
        }

        Ok(())
    }

    pub fn main() {
        if let Err(error) = run() {
            unsafe {
                let message = wide(error);
                let title = wide("CatCPU");
                MessageBoxW(
                    0,
                    message.as_ptr(),
                    title.as_ptr(),
                    MB_OK | MB_ICONWARNING,
                );
            }
        }
    }
