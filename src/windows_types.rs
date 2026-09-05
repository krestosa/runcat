    type Bool = i32;
    type Dword = u32;
    type Uint = u32;
    type UlongPtr = usize;
    type Wparam = usize;
    type Lparam = isize;
    type Lresult = isize;
    type Hinstance = isize;
    type Hwnd = isize;
    type Hicon = isize;
    type Hcursor = isize;
    type Hbrush = isize;
    type Hmenu = isize;
    type Hbitmap = isize;
    type Hdc = isize;
    type Hfont = isize;
    type Handle = isize;
    type Hkey = isize;
    type Atom = u16;
    type GpStatus = i32;
    type ColorRef = Dword;

    const WM_DESTROY: Uint = 0x0002;
    const WM_CLOSE: Uint = 0x0010;
    const WM_SETTINGCHANGE: Uint = 0x001A;
    const WM_ERASEBKGND: Uint = 0x0014;
    const WM_SYSCOLORCHANGE: Uint = 0x0015;
    const WM_NULL: Uint = 0x0000;
    const WM_COMMAND: Uint = 0x0111;
    const WM_SETFONT: Uint = 0x0030;
    const WM_TIMER: Uint = 0x0113;
    const WM_CTLCOLORBTN: Uint = 0x0135;
    const WM_CTLCOLOREDIT: Uint = 0x0133;
    const WM_CTLCOLORLISTBOX: Uint = 0x0134;
    const WM_CTLCOLORSTATIC: Uint = 0x0138;
    const WM_LBUTTONUP: Uint = 0x0202;
    const WM_LBUTTONDBLCLK: Uint = 0x0203;
    const WM_RBUTTONUP: Uint = 0x0205;
    const WM_THEMECHANGED: Uint = 0x031A;
    const WM_APP: Uint = 0x8000;

    const TRAY_CALLBACK: Uint = WM_APP + 1;
    const TIMER_ANIMATION: usize = 1;
    const TIMER_CPU: usize = 2;

    const MENU_SETTINGS: usize = 1000;
    const MENU_STARTUP: usize = 1001;
    const MENU_THEME_AUTO: usize = 1100;
    const MENU_THEME_LIGHT: usize = 1101;
    const MENU_THEME_DARK: usize = 1102;
    const MENU_SPEED_HALF: usize = 1200;
    const MENU_SPEED_NORMAL: usize = 1201;
    const MENU_SPEED_FAST: usize = 1202;
    const MENU_SPEED_FASTER: usize = 1203;
    const MENU_SIZE_COMPACT: usize = 1300;
    const MENU_SIZE_NORMAL: usize = 1301;
    const MENU_SIZE_FULL: usize = 1302;
    const MENU_SIZE_LARGE: usize = 1303;
    const MENU_SIZE_XLARGE: usize = 1304;
    const MENU_IDLE_OFF: usize = 1400;
    const MENU_IDLE_5: usize = 1401;
    const MENU_IDLE_10: usize = 1402;
    const MENU_IDLE_20: usize = 1403;
    const MENU_SMOOTH: usize = 1500;
    const MENU_INVERT: usize = 1501;
    const MENU_SLEEP_IDLE: usize = 1502;
    const MENU_BATTERY_PAUSE: usize = 1503;
    const MENU_OVERLAY: usize = 1504;
    const MENU_PAUSE: usize = 1505;
    const MENU_TOOLTIP_CPU: usize = 1600;
    const MENU_TOOLTIP_RAM: usize = 1601;
    const MENU_TOOLTIP_BATTERY: usize = 1602;
    const MENU_RESET: usize = 1900;
    const MENU_EXIT: usize = 1999;

    const NIM_ADD: Dword = 0x0000_0000;
    const NIM_MODIFY: Dword = 0x0000_0001;
    const NIM_DELETE: Dword = 0x0000_0002;
    const NIF_MESSAGE: Uint = 0x0000_0001;
    const NIF_ICON: Uint = 0x0000_0002;
    const NIF_TIP: Uint = 0x0000_0004;

    const MF_STRING: Uint = 0x0000_0000;
    const MF_CHECKED: Uint = 0x0000_0008;
    const MF_POPUP: Uint = 0x0000_0010;
    const MF_SEPARATOR: Uint = 0x0000_0800;
    const MF_RADIOCHECK: Uint = 0x0000_0200;
    const TPM_RIGHTBUTTON: Uint = 0x0002;
    const TPM_RETURNCMD: Uint = 0x0100;
    const TPM_NONOTIFY: Uint = 0x0080;

    const FRAME_COUNT: usize = 5;
    const DEFAULT_CPU_SAMPLE_MS: Uint = 1000;
    const SMOOTHING_TAU_MS: f64 = 500.0;
    const TRAY_CANVAS: u32 = 32;
    const MIN_CAT_SIZE: u32 = 12;
    const MAX_CAT_SIZE: u32 = 64;

    const WS_OVERLAPPED: Dword = 0x0000_0000;
    const WS_CAPTION: Dword = 0x00C0_0000;
    const WS_SYSMENU: Dword = 0x0008_0000;
    const WS_MINIMIZEBOX: Dword = 0x0002_0000;
    const WS_CHILD: Dword = 0x4000_0000;
    const WS_VISIBLE: Dword = 0x1000_0000;
    const WS_TABSTOP: Dword = 0x0001_0000;
    const WS_BORDER: Dword = 0x0080_0000;
    const WS_POPUP: Dword = 0x8000_0000;
    const ES_AUTOHSCROLL: Dword = 0x0000_0080;
    const BS_AUTOCHECKBOX: Dword = 0x0000_0003;
    const BS_DEFPUSHBUTTON: Dword = 0x0000_0001;
    const CBS_DROPDOWNLIST: Dword = 0x0000_0003;
    const SS_CENTERIMAGE: Dword = 0x0000_0200;
    const BST_CHECKED: usize = 1;
    const BM_GETCHECK: Uint = 0x00F0;
    const BM_SETCHECK: Uint = 0x00F1;
    const CB_ADDSTRING: Uint = 0x0143;
    const CB_GETCURSEL: Uint = 0x0147;
    const CB_SETCURSEL: Uint = 0x014E;
    const SW_HIDE: i32 = 0;
    const SW_SHOW: i32 = 5;
    const SW_RESTORE: i32 = 9;
    const MB_OK: Uint = 0x0000_0000;
    const MB_ICONWARNING: Uint = 0x0000_0030;
    const CW_USEDEFAULT: i32 = i32::MIN;
    const TRANSPARENT: i32 = 1;

    const CFG_THEME: usize = 2001;
    const CFG_STARTUP: usize = 2002;
    const CFG_SPEED: usize = 2003;
    const CFG_SIZE: usize = 2004;
    const CFG_THRESHOLD: usize = 2005;
    const CFG_SAMPLE: usize = 2006;
    const CFG_SMOOTH: usize = 2007;
    const CFG_INVERT: usize = 2008;
    const CFG_SLEEP: usize = 2009;
    const CFG_APPLY: usize = 2010;
    const CFG_RESET: usize = 2011;
    const CFG_CLOSE: usize = 2012;
    const CFG_STATUS: usize = 2013;
    const CFG_CURVE: usize = 2014;
    const CFG_HYSTERESIS: usize = 2015;
    const CFG_TOOLTIP_CPU: usize = 2016;
    const CFG_TOOLTIP_RAM: usize = 2017;
    const CFG_BATTERY_PAUSE: usize = 2018;
    const CFG_OVERLAY: usize = 2019;
    const CFG_TOOLTIP_BATTERY: usize = 2020;
    const CFG_PAUSE: usize = 2021;

    const DIB_RGB_COLORS: Uint = 0;
    const BI_RGB: Dword = 0;
    const IMAGE_LOCK_MODE_READ: Uint = 0x0001;
    const PIXEL_FORMAT_32BPP_ARGB: i32 = 0x0026_200A;
    const DI_NORMAL: Uint = 0x0003;
    const SPI_GETWORKAREA: Uint = 0x0030;

    const ERROR_SUCCESS: i32 = 0;
    const ERROR_ALREADY_EXISTS: Dword = 183;
    const KEY_QUERY_VALUE: Dword = 0x0001;
    const KEY_SET_VALUE: Dword = 0x0002;
    const REG_SZ: Dword = 1;
    const REG_DWORD: Dword = 4;
    const HKEY_CURRENT_USER: Hkey = -2_147_483_647isize;
    const MOVEFILE_REPLACE_EXISTING: Dword = 0x0000_0001;
    const MOVEFILE_WRITE_THROUGH: Dword = 0x0000_0008;

    const WS_EX_TOPMOST: Dword = 0x0000_0008;
    const WS_EX_TRANSPARENT: Dword = 0x0000_0020;
    const WS_EX_TOOLWINDOW: Dword = 0x0000_0080;
    const WS_EX_LAYERED: Dword = 0x0008_0000;
    const WS_EX_NOACTIVATE: Dword = 0x0800_0000;
    const LWA_COLORKEY: Dword = 0x0000_0001;
    const SWP_NOACTIVATE: Uint = 0x0010;
    const SWP_SHOWWINDOW: Uint = 0x0040;
    const HWND_TOPMOST: Hwnd = -1;
    const OVERLAY_COLOR_KEY: Dword = 0x00ff00ff;

    const DWMWA_USE_IMMERSIVE_DARK_MODE_OLD: Dword = 19;
    const DWMWA_USE_IMMERSIVE_DARK_MODE: Dword = 20;
    const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: isize = -4;

    const FW_NORMAL: i32 = 400;
    const FW_SEMIBOLD: i32 = 600;
    const DEFAULT_CHARSET: Dword = 1;
    const CLEARTYPE_QUALITY: Dword = 5;

    const CAT_0: &[u8] = include_bytes!("../assets/cat_0.png");
    const CAT_1: &[u8] = include_bytes!("../assets/cat_1.png");
    const CAT_2: &[u8] = include_bytes!("../assets/cat_2.png");
    const CAT_3: &[u8] = include_bytes!("../assets/cat_3.png");
    const CAT_4: &[u8] = include_bytes!("../assets/cat_4.png");
    const SLEEPING_CAT: &[u8] = include_bytes!("../assets/sleeping-cat.png");
    const CAT_FRAMES: [&[u8]; FRAME_COUNT] = [CAT_0, CAT_1, CAT_2, CAT_3, CAT_4];

    const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    const RUN_VALUE: &str = "CatCPU";
    const THEME_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ThemeMode {
        Auto,
        Light,
        Dark,
    }

    impl ThemeMode {
        fn as_str(self) -> &'static str {
            match self {
                Self::Auto => "auto",
                Self::Light => "light",
                Self::Dark => "dark",
            }
        }

        fn parse(value: &str) -> Option<Self> {
            match value {
                "auto" => Some(Self::Auto),
                "light" => Some(Self::Light),
                "dark" => Some(Self::Dark),
                _ => None,
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum SpeedCurve {
        Smooth,
        Linear,
        Reactive,
    }

    impl SpeedCurve {
        fn as_str(self) -> &'static str {
            match self {
                Self::Smooth => "smooth",
                Self::Linear => "linear",
                Self::Reactive => "reactive",
            }
        }

        fn parse(value: &str) -> Option<Self> {
            match value {
                "smooth" => Some(Self::Smooth),
                "linear" => Some(Self::Linear),
                "reactive" => Some(Self::Reactive),
                _ => None,
            }
        }
    }

    #[derive(Clone, Copy)]
    struct Settings {
        theme: ThemeMode,
        speed_multiplier: f64,
        speed_curve: SpeedCurve,
        size_px: u32,
        idle_threshold: f64,
        idle_hysteresis: f64,
        cpu_sample_ms: Uint,
        smooth_speed: bool,
        invert_speed: bool,
        sleep_idle: bool,
        tooltip_cpu: bool,
        tooltip_ram: bool,
        tooltip_battery: bool,
        pause_on_battery: bool,
        manual_pause: bool,
        overlay_mode: bool,
    }

    impl Default for Settings {
        fn default() -> Self {
            Self {
                theme: ThemeMode::Auto,
                speed_multiplier: 1.0,
                speed_curve: SpeedCurve::Smooth,
                size_px: 32,
                idle_threshold: 0.0,
                idle_hysteresis: 2.0,
                cpu_sample_ms: DEFAULT_CPU_SAMPLE_MS,
                smooth_speed: true,
                invert_speed: false,
                sleep_idle: true,
                tooltip_cpu: true,
                tooltip_ram: false,
                tooltip_battery: false,
                pause_on_battery: false,
                manual_pause: false,
                overlay_mode: false,
            }
        }
    }

    impl Settings {
        fn load() -> Self {
            let mut settings = Self::default();
            let Ok(text) = fs::read_to_string(settings_path()) else {
                return settings;
            };

            for line in text.lines() {
                let Some((key, raw_value)) = line.split_once('=') else {
                    continue;
                };
                let value = raw_value.trim();
                match key.trim() {
                    "theme" => {
                        if let Some(parsed) = ThemeMode::parse(value) {
                            settings.theme = parsed;
                        }
                    }
                    "speed_multiplier" | "speed" => {
                        if let Some(parsed) = parse_f64_range(value, 0.10, 5.0) {
                            settings.speed_multiplier = parsed;
                        }
                    }
                    "speed_curve" => {
                        if let Some(parsed) = SpeedCurve::parse(value) {
                            settings.speed_curve = parsed;
                        }
                    }
                    "size_px" => {
                        if let Some(parsed) = parse_u32_range(value, MIN_CAT_SIZE, MAX_CAT_SIZE) {
                            settings.size_px = parsed;
                        }
                    }
                    "size" => {
                        settings.size_px = match value {
                            "compact" => 20,
                            "normal" => 26,
                            "full" => 32,
                            "large" => 48,
                            "xlarge" => 64,
                            _ => settings.size_px,
                        };
                    }
                    "idle_threshold" => {
                        if let Some(parsed) = parse_f64_range(value, 0.0, 100.0) {
                            settings.idle_threshold = parsed;
                        }
                    }
                    "idle_hysteresis" => {
                        if let Some(parsed) = parse_f64_range(value, 0.0, 25.0) {
                            settings.idle_hysteresis = parsed;
                        }
                    }
                    "cpu_sample_ms" => {
                        if let Some(parsed) = parse_u32_range(value, 250, 5000) {
                            settings.cpu_sample_ms = parsed;
                        }
                    }
                    "smooth_speed" => settings.smooth_speed = parse_bool(value, true),
                    "invert_speed" => settings.invert_speed = parse_bool(value, false),
                    "sleep_idle" => settings.sleep_idle = parse_bool(value, true),
                    "tooltip_cpu" => settings.tooltip_cpu = parse_bool(value, true),
                    "tooltip_ram" => settings.tooltip_ram = parse_bool(value, false),
                    "tooltip_battery" => settings.tooltip_battery = parse_bool(value, false),
                    "pause_on_battery" => settings.pause_on_battery = parse_bool(value, false),
                    "manual_pause" => settings.manual_pause = parse_bool(value, false),
                    "overlay_mode" => settings.overlay_mode = parse_bool(value, false),
                    _ => {}
                }
            }

            settings
        }

        fn save(self) -> bool {
            let text = format!(
                concat!(
                    "theme={}\n",
                    "speed_multiplier={:.2}\n",
                    "speed_curve={}\n",
                    "size_px={}\n",
                    "idle_threshold={:.1}\n",
                    "idle_hysteresis={:.1}\n",
                    "cpu_sample_ms={}\n",
                    "smooth_speed={}\n",
                    "invert_speed={}\n",
                    "sleep_idle={}\n",
                    "tooltip_cpu={}\n",
                    "tooltip_ram={}\n",
                    "tooltip_battery={}\n",
                    "pause_on_battery={}\n",
                    "manual_pause={}\n",
                    "overlay_mode={}\n"
                ),
                self.theme.as_str(),
                self.speed_multiplier,
                self.speed_curve.as_str(),
                self.size_px,
                self.idle_threshold,
                self.idle_hysteresis,
                self.cpu_sample_ms,
                self.smooth_speed,
                self.invert_speed,
                self.sleep_idle,
                self.tooltip_cpu,
                self.tooltip_ram,
                self.tooltip_battery,
                self.pause_on_battery,
                self.manual_pause,
                self.overlay_mode,
            );
            atomic_write_settings(&text)
        }
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
    #[derive(Clone, Copy)]
    struct WorkRect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
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

    #[derive(Clone, Copy)]
    struct PowerSnapshot {
        on_battery: bool,
        battery_percent: Option<u8>,
    }

    struct IconSet {
        tray: [Hicon; FRAME_COUNT],
        tray_sleep: Hicon,
        overlay: [Hicon; FRAME_COUNT],
        overlay_sleep: Hicon,
    }

    struct AppState {
        hwnd: Hwnd,
        overlay_hwnd: Hwnd,
        source_frames: Vec<FramePixels>,
        source_sleep: FramePixels,
        visuals: IconSet,
        frame: usize,
        last_idle: u64,
        last_kernel: u64,
        last_user: u64,
        cpu_percent: f64,
        ram_percent: f64,
        power: PowerSnapshot,
        animation_ms: Uint,
        target_animation_ms: f64,
        is_idle: bool,
        battery_paused: bool,
        settings: Settings,
        effective_light_theme: bool,
        taskbar_created: Uint,
        gdiplus_token: UlongPtr,
        config_hwnd: Hwnd,
        last_tray_icon: Hicon,
        last_tooltip: String,
        ui_bg_brush: Hbrush,
        ui_surface_brush: Hbrush,
        ui_font: Hfont,
        ui_header_font: Hfont,
        ui_light_theme: bool,
        ui_dpi: u32,
    }

    static STATE: OnceLock<Mutex<AppState>> = OnceLock::new();
