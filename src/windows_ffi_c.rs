    #[link(name = "user32")]
    extern "system" {
        fn RegisterClassW(class: *const WndClassW) -> Atom;
        fn CreateWindowExW(
            ex_style: Dword,
            class_name: *const u16,
            window_name: *const u16,
            style: Dword,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            parent: Hwnd,
            menu: Hmenu,
            instance: Hinstance,
            param: *mut c_void,
        ) -> Hwnd;
        fn DefWindowProcW(hwnd: Hwnd, msg: Uint, w_param: Wparam, l_param: Lparam) -> Lresult;
        fn DestroyWindow(hwnd: Hwnd) -> Bool;
        fn GetMessageW(msg: *mut Msg, hwnd: Hwnd, min: Uint, max: Uint) -> Bool;
        fn TranslateMessage(msg: *const Msg) -> Bool;
        fn DispatchMessageW(msg: *const Msg) -> Lresult;
        fn PostQuitMessage(exit_code: i32);
        fn PostMessageW(hwnd: Hwnd, msg: Uint, w_param: Wparam, l_param: Lparam) -> Bool;
        fn SetTimer(hwnd: Hwnd, id: usize, interval_ms: Uint, callback: *const c_void) -> usize;
        fn KillTimer(hwnd: Hwnd, id: usize) -> Bool;
        fn GetCursorPos(point: *mut Point) -> Bool;
        fn SetForegroundWindow(hwnd: Hwnd) -> Bool;
        fn CreatePopupMenu() -> Hmenu;
        fn AppendMenuW(menu: Hmenu, flags: Uint, id_or_submenu: usize, text: *const u16) -> Bool;
        fn TrackPopupMenu(
            menu: Hmenu,
            flags: Uint,
            x: i32,
            y: i32,
            reserved: i32,
            hwnd: Hwnd,
            rect: *const c_void,
        ) -> Uint;
        fn DestroyMenu(menu: Hmenu) -> Bool;
        fn RegisterWindowMessageW(text: *const u16) -> Uint;
        fn DestroyIcon(icon: Hicon) -> Bool;
        fn CreateIconIndirect(info: *const IconInfo) -> Hicon;
        fn SendMessageW(hwnd: Hwnd, msg: Uint, w_param: Wparam, l_param: Lparam) -> Lresult;
        fn ShowWindow(hwnd: Hwnd, command: i32) -> Bool;
        fn UpdateWindow(hwnd: Hwnd) -> Bool;
        fn IsWindow(hwnd: Hwnd) -> Bool;
        fn GetDlgItem(hwnd: Hwnd, id: i32) -> Hwnd;
        fn SetWindowTextW(hwnd: Hwnd, text: *const u16) -> Bool;
        fn GetWindowTextLengthW(hwnd: Hwnd) -> i32;
        fn GetWindowTextW(hwnd: Hwnd, text: *mut u16, max_count: i32) -> i32;
        fn MessageBoxW(hwnd: Hwnd, text: *const u16, caption: *const u16, kind: Uint) -> i32;
        fn GetSysColorBrush(index: i32) -> Hbrush;
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetModuleHandleW(module_name: *const u16) -> Hinstance;
        fn CreateMutexW(attributes: *const c_void, initial_owner: Bool, name: *const u16) -> Handle;
        fn GetLastError() -> Dword;
        fn CloseHandle(handle: Handle) -> Bool;
        fn GetModuleFileNameW(module: Hinstance, filename: *mut u16, size: Dword) -> Dword;
        fn GetSystemTimes(idle: *mut FileTime, kernel: *mut FileTime, user: *mut FileTime) -> Bool;
    }

    #[link(name = "shell32")]
    extern "system" {
        fn Shell_NotifyIconW(message: Dword, data: *mut NotifyIconDataW) -> Bool;
    }

    #[link(name = "shlwapi")]
    extern "system" {
        fn SHCreateMemStream(data: *const u8, size: Uint) -> *mut Unknown;
    }

    #[link(name = "gdiplus")]
    extern "system" {
        fn GdiplusStartup(token: *mut UlongPtr, input: *const GdiplusStartupInput, output: *mut c_void) -> GpStatus;
        fn GdiplusShutdown(token: UlongPtr);
        fn GdipLoadImageFromStream(stream: *mut Unknown, image: *mut *mut c_void) -> GpStatus;
        fn GdipGetImageWidth(image: *mut c_void, width: *mut Uint) -> GpStatus;
        fn GdipGetImageHeight(image: *mut c_void, height: *mut Uint) -> GpStatus;
        fn GdipBitmapLockBits(
            bitmap: *mut c_void,
            rect: *const Rect,
            flags: Uint,
            format: i32,
            locked_data: *mut BitmapData,
        ) -> GpStatus;
        fn GdipBitmapUnlockBits(bitmap: *mut c_void, locked_data: *mut BitmapData) -> GpStatus;
        fn GdipDisposeImage(image: *mut c_void) -> GpStatus;
    }

    #[link(name = "gdi32")]
    extern "system" {
        fn CreateDIBSection(
            dc: Hdc,
            info: *const BitmapInfo,
            usage: Uint,
            bits: *mut *mut c_void,
            section: Handle,
            offset: Dword,
        ) -> Hbitmap;
        fn CreateBitmap(
            width: i32,
            height: i32,
            planes: Uint,
            bit_count: Uint,
            bits: *const c_void,
        ) -> Hbitmap;
        fn DeleteObject(object: isize) -> Bool;
        fn GetStockObject(index: i32) -> isize;
    }

    #[link(name = "advapi32")]
    extern "system" {
        fn RegOpenKeyExW(root: Hkey, subkey: *const u16, options: Dword, access: Dword, result: *mut Hkey) -> i32;
        fn RegCreateKeyExW(
            root: Hkey,
            subkey: *const u16,
            reserved: Dword,
            class_name: *mut u16,
            options: Dword,
            access: Dword,
            security_attributes: *const c_void,
            result: *mut Hkey,
            disposition: *mut Dword,
        ) -> i32;
        fn RegQueryValueExW(
            key: Hkey,
            value_name: *const u16,
            reserved: *mut Dword,
            value_type: *mut Dword,
            data: *mut u8,
            data_size: *mut Dword,
        ) -> i32;
        fn RegSetValueExW(
            key: Hkey,
            value_name: *const u16,
            reserved: Dword,
            value_type: Dword,
            data: *const u8,
            data_size: Dword,
        ) -> i32;
        fn RegDeleteValueW(key: Hkey, value_name: *const u16) -> i32;
        fn RegCloseKey(key: Hkey) -> i32;
    }

    struct OwnedHandle(Handle);

    impl Drop for OwnedHandle {
        fn drop(&mut self) {
            if self.0 != 0 {
                unsafe {
                    CloseHandle(self.0);
                }
            }
        }
    }

    struct FramePixels {
        width: u32,
        height: u32,
        pixels: Vec<u8>,
    }

    #[derive(Clone, Copy)]
    struct AlphaBounds {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    }

    struct AppState {
        hwnd: Hwnd,
        overlay_hwnd: Hwnd,
        source_frames: Vec<FramePixels>,
        source_sleep: FramePixels,
        icons: [Hicon; FRAME_COUNT],
        sleep_icon: Hicon,
        frame: usize,
        last_idle: u64,
        last_kernel: u64,
        last_user: u64,
        cpu_percent: f64,
        ram_percent: f64,
        animation_ms: Uint,
        target_animation_ms: f64,
        is_idle: bool,
        on_battery: bool,
        battery_paused: bool,
        settings: Settings,
        effective_light_theme: bool,
        taskbar_created: Uint,
        gdiplus_token: UlongPtr,
        config_hwnd: Hwnd,
    }

    static STATE: OnceLock<Mutex<AppState>> = OnceLock::new();
