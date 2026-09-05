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
        fn GetClientRect(hwnd: Hwnd, rect: *mut WorkRect) -> Bool;
        fn InvalidateRect(hwnd: Hwnd, rect: *const WorkRect, erase: Bool) -> Bool;
        fn GetDpiForWindow(hwnd: Hwnd) -> Uint;
        fn GetDpiForSystem() -> Uint;
        fn SetProcessDpiAwarenessContext(value: isize) -> Bool;
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetModuleHandleW(module_name: *const u16) -> Hinstance;
        fn CreateMutexW(attributes: *const c_void, initial_owner: Bool, name: *const u16) -> Handle;
        fn GetLastError() -> Dword;
        fn CloseHandle(handle: Handle) -> Bool;
        fn GetModuleFileNameW(module: Hinstance, filename: *mut u16, size: Dword) -> Dword;
        fn GetSystemTimes(idle: *mut FileTime, kernel: *mut FileTime, user: *mut FileTime) -> Bool;
        fn GlobalMemoryStatusEx(status: *mut MemoryStatusEx) -> Bool;
        fn GetSystemPowerStatus(status: *mut SystemPowerStatus) -> Bool;
        fn MoveFileExW(existing: *const u16, new_name: *const u16, flags: Dword) -> Bool;
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
        fn GdiplusStartup(
            token: *mut UlongPtr,
            input: *const GdiplusStartupInput,
            output: *mut c_void,
        ) -> GpStatus;
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
        fn CreateSolidBrush(color: ColorRef) -> Hbrush;
        fn CreateFontW(
            height: i32,
            width: i32,
            escapement: i32,
            orientation: i32,
            weight: i32,
            italic: Dword,
            underline: Dword,
            strikeout: Dword,
            charset: Dword,
            out_precision: Dword,
            clip_precision: Dword,
            quality: Dword,
            pitch_and_family: Dword,
            face: *const u16,
        ) -> Hfont;
        fn SetTextColor(dc: Hdc, color: ColorRef) -> ColorRef;
        fn SetBkColor(dc: Hdc, color: ColorRef) -> ColorRef;
        fn SetBkMode(dc: Hdc, mode: i32) -> i32;
    }

    #[link(name = "advapi32")]
    extern "system" {
        fn RegOpenKeyExW(
            root: Hkey,
            subkey: *const u16,
            options: Dword,
            access: Dword,
            result: *mut Hkey,
        ) -> i32;
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

    #[link(name = "uxtheme")]
    extern "system" {
        fn SetWindowTheme(hwnd: Hwnd, sub_app_name: *const u16, sub_id_list: *const u16) -> i32;
    }

    #[link(name = "dwmapi")]
    extern "system" {
        fn DwmSetWindowAttribute(
            hwnd: Hwnd,
            attribute: Dword,
            value: *const c_void,
            value_size: Dword,
        ) -> i32;
    }
