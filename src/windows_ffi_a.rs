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
    type Handle = isize;
    type Hkey = isize;
    type Atom = u16;
    type GpStatus = i32;

    const WM_DESTROY: Uint = 0x0002;
    const WM_CLOSE: Uint = 0x0010;
    const WM_SETTINGCHANGE: Uint = 0x001A;
    const WM_NULL: Uint = 0x0000;
    const WM_COMMAND: Uint = 0x0111;
    const WM_SETFONT: Uint = 0x0030;
    const WM_TIMER: Uint = 0x0113;
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
    const MENU_IDLE_OFF: usize = 1400;
    const MENU_IDLE_5: usize = 1401;
    const MENU_IDLE_10: usize = 1402;
    const MENU_IDLE_20: usize = 1403;
    const MENU_SMOOTH: usize = 1500;
    const MENU_INVERT: usize = 1501;
    const MENU_SLEEP_IDLE: usize = 1502;
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
    const ICON_CANVAS: u32 = 32;

    const WS_OVERLAPPED: Dword = 0x0000_0000;
    const WS_CAPTION: Dword = 0x00C0_0000;
    const WS_SYSMENU: Dword = 0x0008_0000;
    const WS_MINIMIZEBOX: Dword = 0x0002_0000;
    const WS_CHILD: Dword = 0x4000_0000;
    const WS_VISIBLE: Dword = 0x1000_0000;
    const WS_TABSTOP: Dword = 0x0001_0000;
    const WS_BORDER: Dword = 0x0080_0000;
    const ES_AUTOHSCROLL: Dword = 0x0000_0080;
    const BS_AUTOCHECKBOX: Dword = 0x0000_0003;
    const CBS_DROPDOWNLIST: Dword = 0x0000_0003;
    const BST_CHECKED: usize = 1;
    const BM_GETCHECK: Uint = 0x00F0;
    const BM_SETCHECK: Uint = 0x00F1;
    const CB_ADDSTRING: Uint = 0x0143;
    const CB_GETCURSEL: Uint = 0x0147;
    const CB_SETCURSEL: Uint = 0x014E;
    const SW_SHOW: i32 = 5;
    const SW_RESTORE: i32 = 9;
    const DEFAULT_GUI_FONT: i32 = 17;
    const COLOR_WINDOW: i32 = 5;
    const MB_OK: Uint = 0x0000_0000;
    const MB_ICONWARNING: Uint = 0x0000_0030;
    const CW_USEDEFAULT: i32 = i32::MIN;

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

    const DIB_RGB_COLORS: Uint = 0;
    const BI_RGB: Dword = 0;
    const IMAGE_LOCK_MODE_READ: Uint = 0x0001;
    const PIXEL_FORMAT_32BPP_ARGB: i32 = 0x0026_200A;

    const ERROR_SUCCESS: i32 = 0;
    const ERROR_ALREADY_EXISTS: Dword = 183;
    const KEY_QUERY_VALUE: Dword = 0x0001;
    const KEY_SET_VALUE: Dword = 0x0002;
    const REG_SZ: Dword = 1;
    const REG_DWORD: Dword = 4;
    const HKEY_CURRENT_USER: Hkey = -2_147_483_647isize;

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

    #[derive(Clone, Copy)]
    struct Settings {
        theme: ThemeMode,
        speed_multiplier: f64,
        size_px: u32,
        idle_threshold: f64,
        cpu_sample_ms: Uint,
        smooth_speed: bool,
        invert_speed: bool,
        sleep_idle: bool,
    }

    impl Default for Settings {
        fn default() -> Self {
            Self {
                theme: ThemeMode::Auto,
                speed_multiplier: 1.0,
                size_px: 32,
                idle_threshold: 0.0,
                cpu_sample_ms: DEFAULT_CPU_SAMPLE_MS,
                smooth_speed: true,
                invert_speed: false,
                sleep_idle: true,
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
                let Some((key, value)) = line.split_once('=') else {
                    continue;
                };
                let value = value.trim();
                match key.trim() {
                    "theme" => {
                        if let Some(parsed) = ThemeMode::parse(value) {
                            settings.theme = parsed;
                        }
                    }
                    "speed_multiplier" | "speed" => {
                        if let Ok(parsed) = value.parse::<f64>() {
                            if (0.10..=5.0).contains(&parsed) {
                                settings.speed_multiplier = parsed;
                            }
                        }
                    }
                    "size_px" => {
                        if let Ok(parsed) = value.parse::<u32>() {
                            if (12..=32).contains(&parsed) {
                                settings.size_px = parsed;
                            }
                        }
                    }
                    "size" => {
                        settings.size_px = match value {
                            "compact" => 20,
                            "normal" => 26,
                            "full" => 32,
                            _ => settings.size_px,
                        };
                    }
                    "idle_threshold" => {
                        if let Ok(parsed) = value.parse::<f64>() {
                            if (0.0..=100.0).contains(&parsed) {
                                settings.idle_threshold = parsed;
                            }
                        }
                    }
                    "cpu_sample_ms" => {
                        if let Ok(parsed) = value.parse::<u32>() {
                            if (250..=5000).contains(&parsed) {
                                settings.cpu_sample_ms = parsed;
                            }
                        }
                    }
                    "smooth_speed" => settings.smooth_speed = parse_bool(value, true),
                    "invert_speed" => settings.invert_speed = parse_bool(value, false),
                    "sleep_idle" => settings.sleep_idle = parse_bool(value, true),
                    _ => {}
                }
            }

            settings
        }

        fn save(self) {
            let path = settings_path();
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            let text = format!(
                "theme={}\nspeed_multiplier={:.2}\nsize_px={}\nidle_threshold={:.1}\ncpu_sample_ms={}\nsmooth_speed={}\ninvert_speed={}\nsleep_idle={}\n",
                self.theme.as_str(),
                self.speed_multiplier,
                self.size_px,
                self.idle_threshold,
                self.cpu_sample_ms,
                self.smooth_speed,
                self.invert_speed,
                self.sleep_idle,
            );
            let _ = fs::write(path, text);
        }
    }
