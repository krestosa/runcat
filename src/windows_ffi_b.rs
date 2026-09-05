    fn parse_bool(value: &str, fallback: bool) -> bool {
        match value.trim() {
            "true" | "1" | "yes" => true,
            "false" | "0" | "no" => false,
            _ => fallback,
        }
    }

    fn settings_path() -> PathBuf {
        let base = std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("CatCPU").join("settings.ini")
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    struct Msg {
        hwnd: Hwnd,
        message: Uint,
        w_param: Wparam,
        l_param: Lparam,
        time: Dword,
        pt: Point,
        l_private: Dword,
    }

    #[repr(C)]
    struct WndClassW {
        style: Uint,
        wnd_proc: Option<unsafe extern "system" fn(Hwnd, Uint, Wparam, Lparam) -> Lresult>,
        cls_extra: i32,
        wnd_extra: i32,
        instance: Hinstance,
        icon: Hicon,
        cursor: Hcursor,
        background: Hbrush,
        menu_name: *const u16,
        class_name: *const u16,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct FileTime {
        low: Dword,
        high: Dword,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Guid {
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    }

    #[repr(C)]
    struct NotifyIconDataW {
        cb_size: Dword,
        hwnd: Hwnd,
        id: Uint,
        flags: Uint,
        callback_message: Uint,
        icon: Hicon,
        tip: [u16; 128],
        state: Dword,
        state_mask: Dword,
        info: [u16; 256],
        timeout_or_version: Uint,
        info_title: [u16; 64],
        info_flags: Dword,
        guid_item: Guid,
        balloon_icon: Hicon,
    }

    #[repr(C)]
    struct GdiplusStartupInput {
        version: Uint,
        debug_event_callback: *const c_void,
        suppress_background_thread: Bool,
        suppress_external_codecs: Bool,
    }

    #[repr(C)]
    struct Unknown {
        vtbl: *const UnknownVtable,
    }

    #[repr(C)]
    struct UnknownVtable {
        query_interface: unsafe extern "system" fn(*mut Unknown, *const Guid, *mut *mut c_void) -> i32,
        add_ref: unsafe extern "system" fn(*mut Unknown) -> u32,
        release: unsafe extern "system" fn(*mut Unknown) -> u32,
    }

    #[repr(C)]
    struct Rect {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    }

    #[repr(C)]
    struct BitmapData {
        width: Uint,
        height: Uint,
        stride: i32,
        pixel_format: i32,
        scan0: *mut c_void,
        reserved: UlongPtr,
    }

    #[repr(C)]
    struct BitmapInfoHeader {
        size: Dword,
        width: i32,
        height: i32,
        planes: u16,
        bit_count: u16,
        compression: Dword,
        size_image: Dword,
        x_pels_per_meter: i32,
        y_pels_per_meter: i32,
        clr_used: Dword,
        clr_important: Dword,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct RgbQuad {
        blue: u8,
        green: u8,
        red: u8,
        reserved: u8,
    }

    #[repr(C)]
    struct BitmapInfo {
        header: BitmapInfoHeader,
        colors: [RgbQuad; 1],
    }

    #[repr(C)]
    struct IconInfo {
        is_icon: Bool,
        x_hotspot: Dword,
        y_hotspot: Dword,
        mask: Hbitmap,
        color: Hbitmap,
    }
