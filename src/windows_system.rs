    fn rgb(red: u8, green: u8, blue: u8) -> ColorRef {
        red as ColorRef | ((green as ColorRef) << 8) | ((blue as ColorRef) << 16)
    }

    fn parse_bool(value: &str, fallback: bool) -> bool {
        match value.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => true,
            "false" | "0" | "no" | "off" => false,
            _ => fallback,
        }
    }

    fn parse_f64_range(value: &str, min: f64, max: f64) -> Option<f64> {
        let parsed = value.trim().replace(',', ".").parse::<f64>().ok()?;
        parsed.is_finite().then_some(parsed).filter(|value| (min..=max).contains(value))
    }

    fn parse_u32_range(value: &str, min: u32, max: u32) -> Option<u32> {
        let parsed = value.trim().parse::<u32>().ok()?;
        (min..=max).contains(&parsed).then_some(parsed)
    }

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn wide_path(path: &Path) -> Vec<u16> {
        path.as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }

    fn settings_path() -> PathBuf {
        let base = std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("CatCPU").join("settings.ini")
    }

    fn atomic_write_settings(text: &str) -> bool {
        let path = settings_path();
        let Some(parent) = path.parent() else {
            return false;
        };
        if fs::create_dir_all(parent).is_err() {
            return false;
        }

        let temp = path.with_extension("ini.tmp");
        if fs::write(&temp, text).is_err() {
            return false;
        }

        let from = wide_path(&temp);
        let to = wide_path(&path);
        let moved = unsafe {
            MoveFileExW(
                from.as_ptr(),
                to.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            ) != 0
        };
        if !moved {
            let _ = fs::remove_file(temp);
        }
        moved
    }

    fn filetime_u64(value: FileTime) -> u64 {
        ((value.high as u64) << 32) | value.low as u64
    }

    fn read_cpu_times() -> Option<(u64, u64, u64)> {
        unsafe {
            let mut idle: FileTime = zeroed();
            let mut kernel: FileTime = zeroed();
            let mut user: FileTime = zeroed();
            if GetSystemTimes(&mut idle, &mut kernel, &mut user) == 0 {
                return None;
            }
            Some((filetime_u64(idle), filetime_u64(kernel), filetime_u64(user)))
        }
    }

    fn cpu_usage_and_store(state: &mut AppState) -> Option<f64> {
        let (idle, kernel, user) = read_cpu_times()?;
        if state.last_kernel == 0 && state.last_user == 0 {
            state.last_idle = idle;
            state.last_kernel = kernel;
            state.last_user = user;
            return None;
        }

        let idle_delta = idle.saturating_sub(state.last_idle);
        let kernel_delta = kernel.saturating_sub(state.last_kernel);
        let user_delta = user.saturating_sub(state.last_user);

        state.last_idle = idle;
        state.last_kernel = kernel;
        state.last_user = user;

        let total = kernel_delta.saturating_add(user_delta);
        if total == 0 {
            return None;
        }

        let busy = total.saturating_sub(idle_delta);
        Some((busy as f64 * 100.0 / total as f64).clamp(0.0, 100.0))
    }

    fn target_frame_interval(cpu_percent: f64, settings: Settings) -> f64 {
        let mut utilization = (cpu_percent / 100.0).clamp(0.0, 1.0);
        if settings.invert_speed {
            utilization = 1.0 - utilization;
        }

        let remaining = 1.0 - utilization;
        let cycle_ms = match settings.speed_curve {
            SpeedCurve::Smooth => 250.0 + 850.0 * remaining.powi(2),
            SpeedCurve::Linear => 250.0 + 850.0 * remaining,
            SpeedCurve::Reactive => 250.0 + 850.0 * remaining.powi(3),
        };
        (cycle_ms / FRAME_COUNT as f64 / settings.speed_multiplier).clamp(10.0, 2000.0)
    }

    fn should_idle(cpu_percent: f64, settings: Settings, currently_idle: bool) -> bool {
        if settings.invert_speed {
            return false;
        }
        let wake_threshold = (settings.idle_threshold + settings.idle_hysteresis).min(100.0);
        if currently_idle {
            cpu_percent <= wake_threshold
        } else {
            cpu_percent <= settings.idle_threshold
        }
    }

    fn ram_usage_percent() -> Option<f64> {
        unsafe {
            let mut status: MemoryStatusEx = zeroed();
            status.length = size_of::<MemoryStatusEx>() as Dword;
            if GlobalMemoryStatusEx(&mut status) == 0 {
                return None;
            }
            Some((status.memory_load as f64).clamp(0.0, 100.0))
        }
    }

    fn read_power_status() -> PowerSnapshot {
        unsafe {
            let mut status: SystemPowerStatus = zeroed();
            if GetSystemPowerStatus(&mut status) == 0 {
                return PowerSnapshot {
                    on_battery: false,
                    battery_percent: None,
                };
            }
            PowerSnapshot {
                on_battery: status.ac_line_status == 0,
                battery_percent: (status.battery_life_percent <= 100)
                    .then_some(status.battery_life_percent),
            }
        }
    }

    fn read_registry_dword(subkey: &str, value_name: &str) -> Option<u32> {
        unsafe {
            let subkey = wide(subkey);
            let value_name = wide(value_name);
            let mut key: Hkey = 0;
            if RegOpenKeyExW(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                0,
                KEY_QUERY_VALUE,
                &mut key,
            ) != ERROR_SUCCESS
            {
                return None;
            }

            let mut value_type = 0;
            let mut value = 0u32;
            let mut value_size = size_of::<u32>() as Dword;
            let result = RegQueryValueExW(
                key,
                value_name.as_ptr(),
                null_mut(),
                &mut value_type,
                &mut value as *mut u32 as *mut u8,
                &mut value_size,
            );
            RegCloseKey(key);

            if result == ERROR_SUCCESS
                && value_type == REG_DWORD
                && value_size == size_of::<u32>() as Dword
            {
                Some(value)
            } else {
                None
            }
        }
    }

    fn system_uses_light_taskbar() -> bool {
        read_registry_dword(THEME_KEY, "SystemUsesLightTheme")
            .or_else(|| read_registry_dword(THEME_KEY, "AppsUseLightTheme"))
            .unwrap_or(1)
            != 0
    }

    fn system_uses_light_apps() -> bool {
        read_registry_dword(THEME_KEY, "AppsUseLightTheme").unwrap_or(1) != 0
    }

    fn effective_light_theme(settings: Settings) -> bool {
        match settings.theme {
            ThemeMode::Auto => system_uses_light_taskbar(),
            ThemeMode::Light => true,
            ThemeMode::Dark => false,
        }
    }

    fn registry_value_exists(subkey: &str, value_name: &str) -> bool {
        unsafe {
            let subkey = wide(subkey);
            let value_name = wide(value_name);
            let mut key: Hkey = 0;
            if RegOpenKeyExW(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                0,
                KEY_QUERY_VALUE,
                &mut key,
            ) != ERROR_SUCCESS
            {
                return false;
            }

            let mut value_type = 0;
            let mut size = 0;
            let result = RegQueryValueExW(
                key,
                value_name.as_ptr(),
                null_mut(),
                &mut value_type,
                null_mut(),
                &mut size,
            );
            RegCloseKey(key);
            result == ERROR_SUCCESS && value_type == REG_SZ
        }
    }

    fn startup_enabled() -> bool {
        registry_value_exists(RUN_KEY, RUN_VALUE)
    }

    fn current_exe_command() -> Option<Vec<u16>> {
        unsafe {
            let mut buffer = vec![0u16; 32_768];
            let len = GetModuleFileNameW(0, buffer.as_mut_ptr(), buffer.len() as Dword) as usize;
            if len == 0 || len >= buffer.len() {
                return None;
            }
            let path = String::from_utf16_lossy(&buffer[..len]);
            Some(wide(&format!("\"{}\"", path)))
        }
    }

    fn set_startup_enabled(enabled: bool) -> bool {
        unsafe {
            let subkey = wide(RUN_KEY);
            let value_name = wide(RUN_VALUE);
            let mut key: Hkey = 0;
            let mut disposition = 0;
            if RegCreateKeyExW(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                0,
                null_mut(),
                0,
                KEY_SET_VALUE,
                null(),
                &mut key,
                &mut disposition,
            ) != ERROR_SUCCESS
            {
                return false;
            }

            let result = if enabled {
                let Some(command) = current_exe_command() else {
                    RegCloseKey(key);
                    return false;
                };
                RegSetValueExW(
                    key,
                    value_name.as_ptr(),
                    0,
                    REG_SZ,
                    command.as_ptr() as *const u8,
                    (command.len() * 2) as Dword,
                )
            } else {
                RegDeleteValueW(key, value_name.as_ptr())
            };

            RegCloseKey(key);
            result == ERROR_SUCCESS
        }
    }

    fn config_window_open(state: &AppState) -> bool {
        state.config_hwnd != 0 && unsafe { IsWindow(state.config_hwnd) != 0 }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn cpu_speed_is_monotonic_for_normal_mode() {
            let settings = Settings::default();
            let idle = target_frame_interval(0.0, settings);
            let medium = target_frame_interval(50.0, settings);
            let busy = target_frame_interval(100.0, settings);
            assert!(idle > medium);
            assert!(medium > busy);
        }

        #[test]
        fn idle_hysteresis_requires_a_higher_wake_threshold() {
            let mut settings = Settings::default();
            settings.idle_threshold = 5.0;
            settings.idle_hysteresis = 3.0;

            assert!(should_idle(5.0, settings, false));
            assert!(should_idle(7.5, settings, true));
            assert!(!should_idle(8.5, settings, true));
        }

        #[test]
        fn inverted_speed_never_enters_idle_state() {
            let mut settings = Settings::default();
            settings.invert_speed = true;
            settings.idle_threshold = 100.0;
            assert!(!should_idle(0.0, settings, false));
        }
    }
