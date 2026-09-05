    const WS_EX_TOPMOST: Dword = 0x0000_0008;
    const WS_EX_TRANSPARENT: Dword = 0x0000_0020;
    const WS_EX_TOOLWINDOW: Dword = 0x0000_0080;
    const WS_EX_LAYERED: Dword = 0x0008_0000;
    const WS_EX_NOACTIVATE: Dword = 0x0800_0000;
    const WS_POPUP: Dword = 0x8000_0000;
    const SW_HIDE: i32 = 0;
    const LWA_COLORKEY: Dword = 0x0000_0001;
    const DI_NORMAL: Uint = 0x0003;
    const SPI_GETWORKAREA: Uint = 0x0030;
    const SWP_NOSIZE: Uint = 0x0001;
    const SWP_NOMOVE: Uint = 0x0002;
    const SWP_NOACTIVATE: Uint = 0x0010;
    const SWP_SHOWWINDOW: Uint = 0x0040;
    const HWND_TOPMOST: Hwnd = -1;
    const OVERLAY_COLOR_KEY: Dword = 0x00ff00ff;

    #[repr(C)]
    struct MemoryStatusEx {
        length: Dword,
        memory_load: Dword,
        total_phys: u64,
        avail_phys: u64,
        total_page_file: u64,
        avail_page_file: u64,
        total_virtual: u64,
        avail_virtual: u64,
        avail_extended_virtual: u64,
    }

    #[repr(C)]
    struct SystemPowerStatus {
        ac_line_status: u8,
        battery_flag: u8,
        battery_life_percent: u8,
        system_status_flag: u8,
        battery_life_time: Dword,
        battery_full_life_time: Dword,
    }

    #[repr(C)]
    struct WorkRect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GlobalMemoryStatusEx(status: *mut MemoryStatusEx) -> Bool;
        fn GetSystemPowerStatus(status: *mut SystemPowerStatus) -> Bool;
    }

    #[link(name = "user32")]
    extern "system" {
        fn GetDC(hwnd: Hwnd) -> Hdc;
        fn ReleaseDC(hwnd: Hwnd, dc: Hdc) -> i32;
        fn DrawIconEx(
            dc: Hdc,
            x: i32,
            y: i32,
            icon: Hicon,
            width: i32,
            height: i32,
            step: Uint,
            brush: Hbrush,
            flags: Uint,
        ) -> Bool;
        fn FillRect(dc: Hdc, rect: *const WorkRect, brush: Hbrush) -> i32;
        fn SetLayeredWindowAttributes(hwnd: Hwnd, color_key: Dword, alpha: u8, flags: Dword) -> Bool;
        fn SetWindowPos(
            hwnd: Hwnd,
            insert_after: Hwnd,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            flags: Uint,
        ) -> Bool;
        fn SystemParametersInfoW(action: Uint, param: Uint, data: *mut c_void, update: Uint) -> Bool;
    }

    #[link(name = "gdi32")]
    extern "system" {
        fn CreateSolidBrush(color: Dword) -> Hbrush;
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

    fn system_on_battery() -> bool {
        unsafe {
            let mut status: SystemPowerStatus = zeroed();
            GetSystemPowerStatus(&mut status) != 0 && status.ac_line_status == 0
        }
    }

    fn tray_tooltip(state: &AppState) -> String {
        let mut parts = Vec::new();
        if state.settings.tooltip_cpu {
            parts.push(format!("CPU {:.0}%", state.cpu_percent));
        }
        if state.settings.tooltip_ram {
            parts.push(format!("RAM {:.0}%", state.ram_percent));
        }
        if parts.is_empty() {
            "CatCPU".to_string()
        } else {
            format!("CatCPU — {}", parts.join(" · "))
        }
    }

    fn update_tray_tooltip(state: &AppState) {
        let mut data = notify_data(state.hwnd, current_icon(state));
        data.flags = NIF_TIP;
        let label = wide(&tray_tooltip(state));
        let copy_len = (label.len().saturating_sub(1)).min(data.tip.len() - 1);
        data.tip.fill(0);
        data.tip[..copy_len].copy_from_slice(&label[..copy_len]);
        unsafe {
            Shell_NotifyIconW(NIM_MODIFY, &mut data);
        }
    }

    fn sync_overlay(state: &AppState) {
        if state.overlay_hwnd == 0 {
            return;
        }

        unsafe {
            if !state.settings.overlay_mode || state.settings.size_px <= ICON_CANVAS {
                ShowWindow(state.overlay_hwnd, SW_HIDE);
                return;
            }

            let size = state.settings.size_px.clamp(ICON_CANVAS + 1, 64) as i32;
            let mut work: WorkRect = zeroed();
            if SystemParametersInfoW(SPI_GETWORKAREA, 0, &mut work as *mut WorkRect as *mut c_void, 0) == 0 {
                return;
            }
            let x = work.right - size - 8;
            let y = work.bottom - size - 8;
            SetWindowPos(
                state.overlay_hwnd,
                HWND_TOPMOST,
                x,
                y,
                size,
                size,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
            SetLayeredWindowAttributes(state.overlay_hwnd, OVERLAY_COLOR_KEY, 255, LWA_COLORKEY);

            let dc = GetDC(state.overlay_hwnd);
            if dc == 0 {
                return;
            }
            let brush = CreateSolidBrush(OVERLAY_COLOR_KEY);
            if brush != 0 {
                let rect = WorkRect {
                    left: 0,
                    top: 0,
                    right: size,
                    bottom: size,
                };
                FillRect(dc, &rect, brush);
                DeleteObject(brush);
            }
            DrawIconEx(dc, 0, 0, current_icon(state), size, size, 0, 0, DI_NORMAL);
            ReleaseDC(state.overlay_hwnd, dc);
        }
    }

    unsafe extern "system" fn overlay_wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        DefWindowProcW(hwnd, msg, w_param, l_param)
    }
