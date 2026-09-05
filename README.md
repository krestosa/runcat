# CatCPU

CatCPU is a native Windows tray cat written in Rust. Its running speed follows total CPU usage, and it can curl up into a dedicated sleeping state when activity falls below a configurable threshold.

The tray/runtime remains a lightweight Win32 Rust process. Settings is a separate Rust executable rendered by real WinUI 3 through Microsoft's `windows-reactor`, rather than a GDI imitation of Fluent UI. No C++, C#, .NET, Electron, or WebView application code is used.

## UI architecture

`catcpu.exe`

- Owns the notification-area icon, CPU sampling, animation, overlay and quick menu.
- Uses Win32/GDI+ only where needed for the tray icon and sprite rendering.
- Stays lightweight when Settings is closed.

`catcpu-settings.exe`

- Uses `windows-reactor` `0.100`, backed by actual WinUI 3 controls.
- Uses the Windows App SDK XAML rendering pipeline, including DirectWrite/Direct2D/Windows Composition used by WinUI.
- Uses real `NavigationView`, `ToggleSwitch`, `ComboBox`, `NumberBox`, `Button`, `TextBlock`, `Grid`, `StackPanel` and `Border` controls.
- Uses real WinUI theme resources such as `CardBackgroundFillColorDefaultBrush` and `CardStrokeColorDefaultBrush` rather than hard-coded fake Fluent colors.
- Uses the real WinUI visual states, focus visuals, animations and control rendering.
- Requests the WinUI Mica window backdrop and follows the Windows light/dark theme.
- Writes the same `%APPDATA%\CatCPU\settings.ini` file and notifies the tray process immediately when a setting changes.

Keeping Settings in its own process also means the Windows App SDK/XAML runtime is not initialized inside the always-running tray process.

## Features

- Five-frame CPU-driven running animation.
- Dedicated sleeping-cat idle state with configurable threshold and hysteresis.
- Automatic cat contrast against the Windows taskbar theme.
- Cat size from `12` to `64` px.
- Optional click-through overlay for physical `33–64` px rendering above the taskbar.
- Running and sleeping sprites normalized to the same visual scale.
- Speed multiplier from `0.10×` to `5.00×`.
- Smooth / Linear / Reactive CPU response curves.
- Optional smooth speed transitions.
- Optional inverted CPU/speed behavior.
- Optional manual animation pause.
- Optional pause on battery.
- Optional CPU, RAM and battery data in the tray tooltip.
- Start with Windows.
- Single-instance tray process and single-instance Settings process.
- Tray recovery after Explorer restarts.
- Atomic settings writes.

## Settings pages

The WinUI Settings app uses a real `NavigationView` with four focused pages so controls are not compressed into a dense desktop dialog.

### Appearance

- Cat theme: Automatic / Light / Dark.
- Cat size: `12–64 px`.
- Large overlay.
- Start with Windows.
- Reset defaults.

### Animation

- Speed multiplier.
- Speed curve.
- Smooth speed transitions.
- Invert CPU / speed.
- Pause animation.

### Idle & power

- Sleep threshold: `0–100%`.
- Wake hysteresis: `0–25%`.
- CPU sampling: `250–5000 ms`.
- Sleeping cat when idle.
- Pause animation on battery.

### Tray

- CPU in tooltip.
- RAM in tooltip.
- Battery in tooltip.

## Build on Windows

Install the Rust MSVC toolchain, then run:

```powershell
Set-ExecutionPolicy -Scope Process Bypass
.\build.ps1
```

The two application executables are produced at:

```text
target\release\catcpu.exe
target\release\catcpu-settings.exe
```

`windows-reactor-setup` stages a private Windows App SDK runtime into the same profile directory. Keep the two executables together with those staged runtime files/directories. This makes Settings self-contained instead of requiring a separately installed Windows App Runtime framework package.

The first self-contained build needs network access for the pinned Microsoft Windows App SDK runtime packages. The setup crate caches them for subsequent builds.

## CI

GitHub Actions runs on Windows and performs:

- `cargo check --all-targets`
- `cargo test`
- `cargo clippy --all-targets`
- `cargo build --release`

CI then stages `catcpu.exe`, `catcpu-settings.exe`, the required Windows App SDK DLL/PRI/XAML files and runtime resource directories into a single self-contained artifact.

## Performance behavior

CPU is sampled with `GetSystemTimes`. RAM and power data are sampled only when enabled features need them. The tray icon and tooltip are only sent back to Explorer when their visible content changes. When idle, manually paused, or paused by battery policy, the animation timer is stopped entirely.

The WinUI process exists only while Settings is open, so the heavier XAML/composition stack does not remain resident with the tray cat.

## Assets

Running frames are pinned to `runcat-dev/RunCat365@03b6e2b288c2df5df2433398f5547857bb4d0e2f`.

The sleeping cat is pinned to `runcat-dev/RunCatNeo@b3b1543049ea0a051ecb78654a45f144724ea737`.

The reused image assets remain subject to their upstream Apache-2.0 licensing terms. The license text is included under `LICENSES/Apache-2.0.txt`.
