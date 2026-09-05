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

            if result == ERROR_SUCCESS && value_type == REG_DWORD && value_size == size_of::<u32>() as Dword {
                Some(value)
            } else {
                None
            }
        }
    }

    fn system_uses_light_theme() -> bool {
        read_registry_dword(THEME_KEY, "SystemUsesLightTheme")
            .or_else(|| read_registry_dword(THEME_KEY, "AppsUseLightTheme"))
            .unwrap_or(1)
            != 0
    }

    fn effective_light_theme(settings: Settings) -> bool {
        match settings.theme {
            ThemeMode::Auto => system_uses_light_theme(),
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
