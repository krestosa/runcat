    fn run() -> Result<(), &'static str> {
        unsafe {
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
            let icons = match build_icon_set(&source_frames, light_theme, settings.size_px) {
                Ok(icons) => icons,
                Err(error) => {
                    GdiplusShutdown(gdiplus_token);
                    return Err(error);
                }
            };

            let sleep_icon = match create_icon(&source_sleep, alpha_bounds(&source_sleep), light_theme, settings.size_px) {
                Ok(icon) => icon,
                Err(error) => {
                    destroy_icon_set(&icons);
                    GdiplusShutdown(gdiplus_token);
                    return Err(error);
                }
            };

            let instance = GetModuleHandleW(null());
            if instance == 0 {
                destroy_icon_set(&icons);
                DestroyIcon(sleep_icon);
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
                destroy_icon_set(&icons);
                DestroyIcon(sleep_icon);
                GdiplusShutdown(gdiplus_token);
                return Err("RegisterClassW failed");
            }

            let settings_class_name = wide("CatCPU.SettingsWindow");
            let settings_class = WndClassW {
                style: 0,
                wnd_proc: Some(settings_wnd_proc),
                cls_extra: 0,
                wnd_extra: 0,
                instance,
                icon: 0,
                cursor: 0,
                background: GetSysColorBrush(COLOR_WINDOW),
                menu_name: null(),
                class_name: settings_class_name.as_ptr(),
            };
            if RegisterClassW(&settings_class) == 0 {
                destroy_icon_set(&icons);
                DestroyIcon(sleep_icon);
                GdiplusShutdown(gdiplus_token);
                return Err("settings RegisterClassW failed");
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
                destroy_icon_set(&icons);
                DestroyIcon(sleep_icon);
                GdiplusShutdown(gdiplus_token);
                return Err("CreateWindowExW failed");
            }

            let (idle, kernel, user) = read_cpu_times().unwrap_or((0, 0, 0));
            let taskbar_created_name = wide("TaskbarCreated");
            let taskbar_created = RegisterWindowMessageW(taskbar_created_name.as_ptr());
            let initial_interval = target_frame_interval(0.0, settings).round() as Uint;

            let initial_is_idle = should_idle(0.0, settings);
            let initial_icon = if initial_is_idle && settings.sleep_idle {
                sleep_icon
            } else {
                icons[0]
            };
            let state = AppState {
                hwnd,
                source_frames,
                source_sleep,
                icons,
                sleep_icon,
                frame: 0,
                last_idle: idle,
                last_kernel: kernel,
                last_user: user,
                cpu_percent: 0.0,
                animation_ms: initial_interval.max(1),
                target_animation_ms: initial_interval.max(1) as f64,
                is_idle: initial_is_idle,
                settings,
                effective_light_theme: light_theme,
                taskbar_created,
                gdiplus_token,
                config_hwnd: 0,
            };

            if let Err(returned) = STATE.set(Mutex::new(state)) {
                DestroyWindow(hwnd);
                if let Ok(state) = returned.into_inner() {
                    destroy_icon_set(&state.icons);
                    if state.sleep_icon != 0 {
                        DestroyIcon(state.sleep_icon);
                    }
                    GdiplusShutdown(state.gdiplus_token);
                }
                return Err("state initialization failed");
            }

            if !add_tray_icon(hwnd, initial_icon) {
                DestroyWindow(hwnd);
                return Err("Shell_NotifyIconW failed");
            }

            if let Some(lock) = STATE.get() {
                if let Ok(state) = lock.lock() {
                    if !state.is_idle {
                        SetTimer(hwnd, TIMER_ANIMATION, state.animation_ms, null());
                    }
                }
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

            if let Some(lock) = STATE.get() {
                if let Ok(state) = lock.lock() {
                    destroy_icon_set(&state.icons);
                    if state.sleep_icon != 0 {
                        DestroyIcon(state.sleep_icon);
                    }
                    GdiplusShutdown(state.gdiplus_token);
                }
            }
        }

        Ok(())
    }

    pub fn main() {
        let _ = run();
    }
