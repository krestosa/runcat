#![windows_subsystem = "windows"]

use std::ffi::c_void;
use std::fs;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr::{null, null_mut};
use windows_reactor::*;

const WM_APP: u32 = 0x8000;
const WM_CLOSE: u32 = 0x0010;
const SETTINGS_CHANGED_MESSAGE: u32 = WM_APP + 50;
const HKEY_CURRENT_USER: isize = -2_147_483_647isize;
const ERROR_SUCCESS: i32 = 0;
const ERROR_FILE_NOT_FOUND: i32 = 2;
const ERROR_ALREADY_EXISTS: u32 = 183;
const KEY_QUERY_VALUE: u32 = 0x0001;
const KEY_SET_VALUE: u32 = 0x0002;
const REG_SZ: u32 = 1;
const MOVEFILE_REPLACE_EXISTING: u32 = 0x0000_0001;
const MOVEFILE_WRITE_THROUGH: u32 = 0x0000_0008;
const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_VALUE: &str = "CatCPU";

#[link(name = "user32")]
extern "system" {
    fn FindWindowW(class_name: *const u16, window_name: *const u16) -> isize;
    fn PostMessageW(hwnd: isize, message: u32, w_param: usize, l_param: isize) -> i32;
}

#[link(name = "kernel32")]
extern "system" {
    fn CreateMutexW(attributes: *const c_void, initial_owner: i32, name: *const u16) -> isize;
    fn GetLastError() -> u32;
    fn CloseHandle(handle: isize) -> i32;
    fn MoveFileExW(existing: *const u16, new_name: *const u16, flags: u32) -> i32;
}

#[link(name = "advapi32")]
extern "system" {
    fn RegOpenKeyExW(
        key: isize,
        subkey: *const u16,
        options: u32,
        access: u32,
        result: *mut isize,
    ) -> i32;
    fn RegQueryValueExW(
        key: isize,
        value_name: *const u16,
        reserved: *mut u32,
        value_type: *mut u32,
        data: *mut u8,
        data_size: *mut u32,
    ) -> i32;
    fn RegCreateKeyExW(
        key: isize,
        subkey: *const u16,
        reserved: u32,
        class_name: *mut u16,
        options: u32,
        access: u32,
        security_attributes: *const c_void,
        result: *mut isize,
        disposition: *mut u32,
    ) -> i32;
    fn RegSetValueExW(
        key: isize,
        value_name: *const u16,
        reserved: u32,
        value_type: u32,
        data: *const u8,
        data_size: u32,
    ) -> i32;
    fn RegDeleteValueW(key: isize, value_name: *const u16) -> i32;
    fn RegCloseKey(key: isize) -> i32;
}

struct OwnedHandle(isize);

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        if self.0 != 0 {
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
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

fn parse_bool(value: &str, fallback: bool) -> bool {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => fallback,
    }
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

fn tray_hwnd() -> isize {
    let class_name = wide("CatCPU.HiddenWindow");
    unsafe { FindWindowW(class_name.as_ptr(), null()) }
}

fn notify_tray_app() {
    let hwnd = tray_hwnd();
    if hwnd != 0 {
        unsafe {
            PostMessageW(hwnd, SETTINGS_CHANGED_MESSAGE, 0, 0);
        }
    }
}

fn exit_tray_app() {
    let hwnd = tray_hwnd();
    if hwnd != 0 {
        unsafe {
            PostMessageW(hwnd, WM_CLOSE, 0, 0);
        }
    }
}

fn startup_enabled() -> bool {
    unsafe {
        let subkey = wide(RUN_KEY);
        let value_name = wide(RUN_VALUE);
        let mut key = 0isize;
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

        let mut value_type = 0u32;
        let mut size = 0u32;
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

fn set_startup_enabled(enabled: bool) -> bool {
    unsafe {
        let subkey = wide(RUN_KEY);
        let value_name = wide(RUN_VALUE);
        let mut key = 0isize;
        let mut disposition = 0u32;
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
            let Some(settings_exe) = std::env::current_exe().ok() else {
                RegCloseKey(key);
                return false;
            };
            let tray_exe = settings_exe.with_file_name("catcpu.exe");
            let command = wide(&format!("\"{}\"", tray_exe.display()));
            RegSetValueExW(
                key,
                value_name.as_ptr(),
                0,
                REG_SZ,
                command.as_ptr() as *const u8,
                (command.len() * size_of::<u16>()) as u32,
            )
        } else {
            RegDeleteValueW(key, value_name.as_ptr())
        };

        RegCloseKey(key);
        result == ERROR_SUCCESS || (!enabled && result == ERROR_FILE_NOT_FOUND)
    }
}

#[derive(Clone, Copy)]
struct SettingsModel {
    theme: usize,
    speed_multiplier: f64,
    speed_curve: usize,
    size_px: u32,
    idle_threshold: f64,
    idle_hysteresis: f64,
    cpu_sample_ms: u32,
    smooth_speed: bool,
    invert_speed: bool,
    sleep_idle: bool,
    tooltip_cpu: bool,
    tooltip_ram: bool,
    tooltip_battery: bool,
    pause_on_battery: bool,
    manual_pause: bool,
    overlay_mode: bool,
    startup: bool,
}

impl Default for SettingsModel {
    fn default() -> Self {
        Self {
            theme: 0,
            speed_multiplier: 1.0,
            speed_curve: 0,
            size_px: 32,
            idle_threshold: 0.0,
            idle_hysteresis: 2.0,
            cpu_sample_ms: 1000,
            smooth_speed: true,
            invert_speed: false,
            sleep_idle: true,
            tooltip_cpu: true,
            tooltip_ram: false,
            tooltip_battery: false,
            pause_on_battery: false,
            manual_pause: false,
            overlay_mode: false,
            startup: startup_enabled(),
        }
    }
}

impl SettingsModel {
    fn load() -> Self {
        let mut model = Self::default();
        let Ok(text) = fs::read_to_string(settings_path()) else {
            return model;
        };

        for line in text.lines() {
            let Some((key, raw)) = line.split_once('=') else {
                continue;
            };
            let value = raw.trim();
            match key.trim() {
                "theme" => {
                    model.theme = match value {
                        "light" => 1,
                        "dark" => 2,
                        _ => 0,
                    }
                }
                "speed_multiplier" | "speed" => {
                    if let Ok(parsed) = value.replace(',', ".").parse::<f64>() {
                        if parsed.is_finite() && (0.10..=5.0).contains(&parsed) {
                            model.speed_multiplier = parsed;
                        }
                    }
                }
                "speed_curve" => {
                    model.speed_curve = match value {
                        "linear" => 1,
                        "reactive" => 2,
                        _ => 0,
                    }
                }
                "size_px" => {
                    if let Ok(parsed) = value.parse::<u32>() {
                        if (12..=64).contains(&parsed) {
                            model.size_px = parsed;
                        }
                    }
                }
                "idle_threshold" => {
                    if let Ok(parsed) = value.replace(',', ".").parse::<f64>() {
                        if parsed.is_finite() && (0.0..=100.0).contains(&parsed) {
                            model.idle_threshold = parsed;
                        }
                    }
                }
                "idle_hysteresis" => {
                    if let Ok(parsed) = value.replace(',', ".").parse::<f64>() {
                        if parsed.is_finite() && (0.0..=25.0).contains(&parsed) {
                            model.idle_hysteresis = parsed;
                        }
                    }
                }
                "cpu_sample_ms" => {
                    if let Ok(parsed) = value.parse::<u32>() {
                        if (250..=5000).contains(&parsed) {
                            model.cpu_sample_ms = parsed;
                        }
                    }
                }
                "smooth_speed" => model.smooth_speed = parse_bool(value, true),
                "invert_speed" => model.invert_speed = parse_bool(value, false),
                "sleep_idle" => model.sleep_idle = parse_bool(value, true),
                "tooltip_cpu" => model.tooltip_cpu = parse_bool(value, true),
                "tooltip_ram" => model.tooltip_ram = parse_bool(value, false),
                "tooltip_battery" => model.tooltip_battery = parse_bool(value, false),
                "pause_on_battery" => model.pause_on_battery = parse_bool(value, false),
                "manual_pause" => model.manual_pause = parse_bool(value, false),
                "overlay_mode" => model.overlay_mode = parse_bool(value, false),
                _ => {}
            }
        }
        model.startup = startup_enabled();
        model
    }

    fn save(self) -> bool {
        let theme = match self.theme {
            1 => "light",
            2 => "dark",
            _ => "auto",
        };
        let curve = match self.speed_curve {
            1 => "linear",
            2 => "reactive",
            _ => "smooth",
        };
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
            theme,
            self.speed_multiplier,
            curve,
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

fn persist_model(model: SettingsModel) {
    if model.save() {
        notify_tray_app();
    }
}

fn open_full_settings() {
    if let Ok(exe) = std::env::current_exe() {
        let _ = std::process::Command::new(exe).spawn();
    }
}

#[derive(Clone)]
enum Message {
    Page(Option<String>),
    PaneOpen(bool),
    DisplayMode(NavigationViewDisplayMode),
    Theme(Option<usize>),
    Startup(bool),
    Speed(Option<f64>),
    Curve(Option<usize>),
    Size(Option<f64>),
    Smooth(bool),
    Invert(bool),
    Pause(bool),
    Threshold(Option<f64>),
    Hysteresis(Option<f64>),
    Sample(Option<f64>),
    Sleep(bool),
    BatteryPause(bool),
    TooltipCpu(bool),
    TooltipRam(bool),
    TooltipBattery(bool),
    Overlay(bool),
    Reset,
}

struct SettingsApp {
    page: String,
    pane_open: bool,
    display_mode: NavigationViewDisplayMode,
    model: SettingsModel,
}

impl SettingsApp {
    fn persist(&self) {
        persist_model(self.model);
    }

    fn settings_card(title: &str, description: &str, action: impl Into<View>) -> View {
        Border::new()
            .background(ThemeBrush::CardBackground)
            .border_brush(ThemeBrush::CardStroke)
            .border_thickness(1.0)
            .corner_radius(4.0)
            .min_height(68.0)
            .padding(16.0)
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .content(
                Grid::new()
                    .columns([GridLength::STAR, GridLength::Auto])
                    .column_spacing(16.0)
                    .horizontal_alignment(HorizontalAlignment::Stretch)
                    .children((
                        StackPanel::new()
                            .spacing(2.0)
                            .vertical_alignment(VerticalAlignment::Center)
                            .horizontal_alignment(HorizontalAlignment::Stretch)
                            .grid_column(0)
                            .children((
                                TextBlock::new()
                                    .text(title)
                                    .font_size(14.0)
                                    .text_wrapping(TextWrapping::Wrap),
                                TextBlock::new()
                                    .text(description)
                                    .font_size(12.0)
                                    .opacity(0.7)
                                    .text_wrapping(TextWrapping::Wrap),
                            )),
                        Border::new()
                            .grid_column(1)
                            .vertical_alignment(VerticalAlignment::Center)
                            .horizontal_alignment(HorizontalAlignment::Right)
                            .content(action),
                    )),
            )
            .into()
    }

    fn page_shell(title: &str, body: impl Into<View>) -> View {
        ScrollViewer::new()
            .horizontal_scroll_bar_visibility(ScrollBarVisibility::Disabled)
            .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .vertical_alignment(VerticalAlignment::Stretch)
            .content(
                StackPanel::new()
                    .max_width(760.0)
                    .horizontal_alignment(HorizontalAlignment::Stretch)
                    .margin(Thickness::uniform(24.0))
                    .spacing(16.0)
                    .children((
                        TextBlock::new()
                            .text(title)
                            .font_size(28.0)
                            .font_weight(FontWeight::SEMI_BOLD),
                        TextBlock::new()
                            .text("Changes are applied immediately.")
                            .font_size(12.0)
                            .opacity(0.7),
                        body.into(),
                    )),
            )
            .into()
    }

    fn appearance_page(&self, context: &mut ViewContext<Self>) -> View {
        let theme = ComboBox::new()
            .width(220.0)
            .items_source([
                "Automatic",
                "Light taskbar (black cat)",
                "Dark taskbar (white cat)",
            ])
            .selected_index(self.model.theme)
            .on_selection_changed(context.callback(Message::Theme));
        let size = NumberBox::new()
            .width(140.0)
            .minimum(12.0)
            .maximum(64.0)
            .value(self.model.size_px as f64)
            .on_value_changed(context.callback(Message::Size));
        let startup = ToggleSwitch::new()
            .is_on(self.model.startup)
            .on_toggled(context.callback(Message::Startup));
        let overlay = ToggleSwitch::new()
            .is_on(self.model.overlay_mode)
            .on_toggled(context.callback(Message::Overlay));

        Self::page_shell(
            "Appearance",
            StackPanel::new()
                .spacing(8.0)
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .children((
                    Self::settings_card(
                        "Cat theme",
                        "Follow Windows automatically or override the cat contrast.",
                        theme,
                    ),
                    Self::settings_card(
                        "Cat size",
                        "Set the visual size from 12 to 64 px.",
                        size,
                    ),
                    Self::settings_card(
                        "Large overlay",
                        "Show sizes above the notification-area limit in an overlay.",
                        overlay,
                    ),
                    Self::settings_card(
                        "Start with Windows",
                        "Launch CatCPU when the current user signs in.",
                        startup,
                    ),
                    Button::new()
                        .horizontal_alignment(HorizontalAlignment::Left)
                        .on_click(context.callback(|_| Message::Reset))
                        .content("Reset defaults"),
                )),
        )
    }

    fn animation_page(&self, context: &mut ViewContext<Self>) -> View {
        let speed = NumberBox::new()
            .width(140.0)
            .minimum(0.10)
            .maximum(5.0)
            .value(self.model.speed_multiplier)
            .on_value_changed(context.callback(Message::Speed));
        let curve = ComboBox::new()
            .width(180.0)
            .items_source(["Smooth", "Linear", "Reactive"])
            .selected_index(self.model.speed_curve)
            .on_selection_changed(context.callback(Message::Curve));
        let smooth = ToggleSwitch::new()
            .is_on(self.model.smooth_speed)
            .on_toggled(context.callback(Message::Smooth));
        let invert = ToggleSwitch::new()
            .is_on(self.model.invert_speed)
            .on_toggled(context.callback(Message::Invert));
        let pause = ToggleSwitch::new()
            .is_on(self.model.manual_pause)
            .on_toggled(context.callback(Message::Pause));

        Self::page_shell(
            "Animation",
            StackPanel::new()
                .spacing(8.0)
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .children((
                    Self::settings_card(
                        "Speed multiplier",
                        "Scale animation speed from 0.10× to 5.00×.",
                        speed,
                    ),
                    Self::settings_card(
                        "Speed curve",
                        "Choose how strongly CPU usage changes animation speed.",
                        curve,
                    ),
                    Self::settings_card(
                        "Smooth speed transitions",
                        "Blend speed changes rather than stepping between values.",
                        smooth,
                    ),
                    Self::settings_card(
                        "Invert CPU / speed",
                        "Reverse the relationship between CPU usage and animation speed.",
                        invert,
                    ),
                    Self::settings_card(
                        "Pause animation",
                        "Keep the current cat visible without advancing frames.",
                        pause,
                    ),
                )),
        )
    }

    fn idle_page(&self, context: &mut ViewContext<Self>) -> View {
        let threshold = NumberBox::new()
            .width(140.0)
            .minimum(0.0)
            .maximum(100.0)
            .value(self.model.idle_threshold)
            .on_value_changed(context.callback(Message::Threshold));
        let hysteresis = NumberBox::new()
            .width(140.0)
            .minimum(0.0)
            .maximum(25.0)
            .value(self.model.idle_hysteresis)
            .on_value_changed(context.callback(Message::Hysteresis));
        let sample = NumberBox::new()
            .width(140.0)
            .minimum(250.0)
            .maximum(5000.0)
            .value(self.model.cpu_sample_ms as f64)
            .on_value_changed(context.callback(Message::Sample));
        let sleeping = ToggleSwitch::new()
            .is_on(self.model.sleep_idle)
            .on_toggled(context.callback(Message::Sleep));
        let battery = ToggleSwitch::new()
            .is_on(self.model.pause_on_battery)
            .on_toggled(context.callback(Message::BatteryPause));

        Self::page_shell(
            "Idle & power",
            StackPanel::new()
                .spacing(8.0)
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .children((
                    Self::settings_card(
                        "Sleep threshold",
                        "Enter the CPU percentage at or below which the cat becomes idle.",
                        threshold,
                    ),
                    Self::settings_card(
                        "Wake hysteresis",
                        "Require extra CPU activity before waking to prevent rapid toggling.",
                        hysteresis,
                    ),
                    Self::settings_card(
                        "CPU sampling",
                        "Set the CPU sampling interval from 250 to 5000 ms.",
                        sample,
                    ),
                    Self::settings_card(
                        "Sleeping cat when idle",
                        "Use the sleeping sprite instead of freezing a running frame.",
                        sleeping,
                    ),
                    Self::settings_card(
                        "Pause animation on battery",
                        "Reduce animation activity while the computer is unplugged.",
                        battery,
                    ),
                )),
        )
    }

    fn tray_page(&self, context: &mut ViewContext<Self>) -> View {
        let cpu = ToggleSwitch::new()
            .is_on(self.model.tooltip_cpu)
            .on_toggled(context.callback(Message::TooltipCpu));
        let ram = ToggleSwitch::new()
            .is_on(self.model.tooltip_ram)
            .on_toggled(context.callback(Message::TooltipRam));
        let battery = ToggleSwitch::new()
            .is_on(self.model.tooltip_battery)
            .on_toggled(context.callback(Message::TooltipBattery));

        Self::page_shell(
            "Tray",
            StackPanel::new()
                .spacing(8.0)
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .children((
                    Self::settings_card(
                        "CPU in tooltip",
                        "Show current total CPU usage in the notification-area tooltip.",
                        cpu,
                    ),
                    Self::settings_card(
                        "RAM in tooltip",
                        "Show current physical memory usage in the tooltip.",
                        ram,
                    ),
                    Self::settings_card(
                        "Battery in tooltip",
                        "Show AC or battery state and charge percentage in the tooltip.",
                        battery,
                    ),
                )),
        )
    }
}

impl Component for SettingsApp {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            page: "appearance".to_string(),
            pane_open: true,
            display_mode: NavigationViewDisplayMode::Expanded,
            model: SettingsModel::load(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        let mut persist = true;
        match message {
            Message::Page(Some(page))
                if matches!(page.as_str(), "appearance" | "animation" | "idle" | "tray") =>
            {
                self.page = page;
                persist = false;
            }
            Message::Page(_) => persist = false,
            Message::PaneOpen(value) => {
                self.pane_open = value;
                persist = false;
            }
            Message::DisplayMode(value) => {
                self.display_mode = value;
                persist = false;
            }
            Message::Theme(Some(index)) if index <= 2 => self.model.theme = index,
            Message::Theme(_) => persist = false,
            Message::Startup(value) => {
                if set_startup_enabled(value) {
                    self.model.startup = value;
                }
                persist = false;
            }
            Message::Speed(Some(value)) if (0.10..=5.0).contains(&value) => {
                self.model.speed_multiplier = value;
            }
            Message::Speed(_) => persist = false,
            Message::Curve(Some(index)) if index <= 2 => self.model.speed_curve = index,
            Message::Curve(_) => persist = false,
            Message::Size(Some(value)) if (12.0..=64.0).contains(&value) => {
                self.model.size_px = value.round() as u32;
            }
            Message::Size(_) => persist = false,
            Message::Smooth(value) => self.model.smooth_speed = value,
            Message::Invert(value) => self.model.invert_speed = value,
            Message::Pause(value) => self.model.manual_pause = value,
            Message::Threshold(Some(value)) if (0.0..=100.0).contains(&value) => {
                self.model.idle_threshold = value;
            }
            Message::Threshold(_) => persist = false,
            Message::Hysteresis(Some(value)) if (0.0..=25.0).contains(&value) => {
                self.model.idle_hysteresis = value;
            }
            Message::Hysteresis(_) => persist = false,
            Message::Sample(Some(value)) if (250.0..=5000.0).contains(&value) => {
                self.model.cpu_sample_ms = value.round() as u32;
            }
            Message::Sample(_) => persist = false,
            Message::Sleep(value) => self.model.sleep_idle = value,
            Message::BatteryPause(value) => self.model.pause_on_battery = value,
            Message::TooltipCpu(value) => self.model.tooltip_cpu = value,
            Message::TooltipRam(value) => self.model.tooltip_ram = value,
            Message::TooltipBattery(value) => self.model.tooltip_battery = value,
            Message::Overlay(value) => self.model.overlay_mode = value,
            Message::Reset => {
                let startup = self.model.startup;
                self.model = SettingsModel::default();
                self.model.startup = startup;
            }
        }
        if persist {
            self.persist();
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("CatCPU Settings");
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
                .client_size(900.0, 700.0)
                .constraints(WindowConstraints {
                    min_width: Some(560.0),
                    min_height: Some(520.0),
                    ..Default::default()
                }),
        );

        let items = [
            ("appearance", "Appearance", Symbol::Setting),
            ("animation", "Animation", Symbol::Play),
            ("idle", "Idle & power", Symbol::Clock),
            ("tray", "Tray", Symbol::More),
        ]
        .into_iter()
        .map(|(tag, label, symbol)| {
            KeyedView::new(
                tag,
                NavigationViewItem::new()
                    .tag(tag)
                    .is_selected(self.page == tag)
                    .slots([
                        SlotView::new(NavigationViewItemSlot::Content, label),
                        SlotView::new(
                            NavigationViewItemSlot::Icon,
                            SymbolIcon::new().symbol(symbol),
                        ),
                    ]),
            )
        });

        let content = match self.page.as_str() {
            "animation" => self.animation_page(context),
            "idle" => self.idle_page(context),
            "tray" => self.tray_page(context),
            _ => self.appearance_page(context),
        };

        NavigationView::new()
            .is_pane_open(self.pane_open)
            .on_is_pane_open_changed(context.callback(Message::PaneOpen))
            .pane_display_mode(NavigationViewPaneDisplayMode::Auto)
            .on_display_mode_changed(context.callback(Message::DisplayMode))
            .open_pane_length(280.0)
            .pane_title("CatCPU")
            .is_settings_visible(false)
            .on_selected_tag_changed(context.callback(Message::Page))
            .slots([
                SlotView::collection(NavigationViewSlot::MenuItems, items),
                SlotView::new(NavigationViewSlot::Content, content),
            ])
    }
}

#[derive(Clone)]
enum QuickMessage {
    Theme(Option<usize>),
    Speed(Option<f64>),
    Size(Option<f64>),
    Pause(bool),
    Startup(bool),
    Sleep(bool),
    OpenFull,
    Exit,
}

struct TrayQuickApp {
    model: SettingsModel,
}

impl TrayQuickApp {
    fn persist(&self) {
        persist_model(self.model);
    }

    fn row(title: &str, action: impl Into<View>) -> View {
        Border::new()
            .background(ThemeBrush::CardBackground)
            .border_brush(ThemeBrush::CardStroke)
            .border_thickness(1.0)
            .corner_radius(4.0)
            .min_height(56.0)
            .padding(12.0)
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .content(
                Grid::new()
                    .columns([GridLength::STAR, GridLength::Auto])
                    .column_spacing(12.0)
                    .children((
                        TextBlock::new()
                            .text(title)
                            .font_size(14.0)
                            .vertical_alignment(VerticalAlignment::Center)
                            .grid_column(0),
                        Border::new()
                            .grid_column(1)
                            .vertical_alignment(VerticalAlignment::Center)
                            .horizontal_alignment(HorizontalAlignment::Right)
                            .content(action),
                    )),
            )
            .into()
    }
}

impl Component for TrayQuickApp {
    type Message = QuickMessage;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            model: SettingsModel::load(),
        }
    }

    fn update(&mut self, message: QuickMessage, _context: &ComponentContext<Self>) {
        let mut persist = true;
        match message {
            QuickMessage::Theme(Some(index)) if index <= 2 => self.model.theme = index,
            QuickMessage::Theme(_) => persist = false,
            QuickMessage::Speed(Some(value)) if (0.10..=5.0).contains(&value) => {
                self.model.speed_multiplier = value;
            }
            QuickMessage::Speed(_) => persist = false,
            QuickMessage::Size(Some(value)) if (12.0..=64.0).contains(&value) => {
                self.model.size_px = value.round() as u32;
            }
            QuickMessage::Size(_) => persist = false,
            QuickMessage::Pause(value) => self.model.manual_pause = value,
            QuickMessage::Startup(value) => {
                if set_startup_enabled(value) {
                    self.model.startup = value;
                }
                persist = false;
            }
            QuickMessage::Sleep(value) => self.model.sleep_idle = value,
            QuickMessage::OpenFull => {
                open_full_settings();
                persist = false;
            }
            QuickMessage::Exit => {
                exit_tray_app();
                persist = false;
            }
        }
        if persist {
            self.persist();
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("CatCPU");
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
                .client_size(420.0, 520.0)
                .constraints(WindowConstraints {
                    min_width: Some(360.0),
                    min_height: Some(440.0),
                    max_width: Some(520.0),
                    ..Default::default()
                }),
        );

        let theme = ComboBox::new()
            .width(180.0)
            .items_source(["Automatic", "Light", "Dark"])
            .selected_index(self.model.theme)
            .on_selection_changed(context.callback(QuickMessage::Theme));
        let speed = NumberBox::new()
            .width(120.0)
            .minimum(0.10)
            .maximum(5.0)
            .value(self.model.speed_multiplier)
            .on_value_changed(context.callback(QuickMessage::Speed));
        let size = NumberBox::new()
            .width(120.0)
            .minimum(12.0)
            .maximum(64.0)
            .value(self.model.size_px as f64)
            .on_value_changed(context.callback(QuickMessage::Size));
        let pause = ToggleSwitch::new()
            .is_on(self.model.manual_pause)
            .on_toggled(context.callback(QuickMessage::Pause));
        let startup = ToggleSwitch::new()
            .is_on(self.model.startup)
            .on_toggled(context.callback(QuickMessage::Startup));
        let sleep = ToggleSwitch::new()
            .is_on(self.model.sleep_idle)
            .on_toggled(context.callback(QuickMessage::Sleep));

        ScrollViewer::new()
            .horizontal_scroll_bar_visibility(ScrollBarVisibility::Disabled)
            .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
            .content(
                StackPanel::new()
                    .margin(Thickness::uniform(16.0))
                    .spacing(8.0)
                    .horizontal_alignment(HorizontalAlignment::Stretch)
                    .children((
                        TextBlock::new()
                            .text("CatCPU")
                            .font_size(24.0)
                            .font_weight(FontWeight::SEMI_BOLD),
                        TextBlock::new()
                            .text("Quick settings")
                            .font_size(12.0)
                            .opacity(0.7),
                        Self::row("Pause animation", pause),
                        Self::row("Start with Windows", startup),
                        Self::row("Cat theme", theme),
                        Self::row("Speed", speed),
                        Self::row("Cat size", size),
                        Self::row("Sleeping cat when idle", sleep),
                        Grid::new()
                            .columns([GridLength::STAR, GridLength::Auto])
                            .column_spacing(8.0)
                            .margin(Thickness::uniform(4.0))
                            .children((
                                Button::new()
                                    .style(ButtonStyle::Accent)
                                    .horizontal_alignment(HorizontalAlignment::Left)
                                    .grid_column(0)
                                    .on_click(context.callback(|_| QuickMessage::OpenFull))
                                    .content("Open Settings"),
                                Button::new()
                                    .grid_column(1)
                                    .on_click(context.callback(|_| QuickMessage::Exit))
                                    .content("Exit CatCPU"),
                            )),
                    )),
            )
    }
}

fn run_single_instance(name: &str) -> Option<OwnedHandle> {
    let mutex_name = wide(name);
    let mutex = unsafe { CreateMutexW(null(), 0, mutex_name.as_ptr()) };
    if mutex == 0 {
        return None;
    }
    let guard = OwnedHandle(mutex);
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        return None;
    }
    Some(guard)
}

fn main() {
    let tray_quick = std::env::args().any(|arg| arg == "--tray");
    let mutex = if tray_quick {
        run_single_instance("Local\\CatCPU.TrayQuick.Singleton")
    } else {
        run_single_instance("Local\\CatCPU.Settings.Singleton")
    };
    let Some(_guard) = mutex else {
        return;
    };

    if tray_quick {
        let _ = App::run_component::<TrayQuickApp>(());
    } else {
        let _ = App::run_component::<SettingsApp>(());
    }
}
