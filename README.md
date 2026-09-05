# CatCPU

A lightweight native Windows tray cat written in Rust. The cat runs faster as total CPU usage rises and curls up to sleep when activity falls below the configured threshold.

The application is implemented with Win32 APIs and the Rust standard library. It does not require .NET, Electron, WebView, a background service, or third-party Rust crates.

## Highlights

- Five-frame CPU-driven running animation.
- Dedicated sleeping-cat idle state with configurable threshold and hysteresis.
- Automatic cat contrast against the Windows taskbar theme.
- Settings window that automatically follows the Windows app light/dark theme, including the title bar and native controls.
- DPI-aware native UI.
- Left-click the tray cat to open Settings; right-click for quick controls.
- Configurable speed multiplier and Smooth / Linear / Reactive CPU curves.
- Cat size from `12` to `64` px.
- Optional click-through large overlay for physical `33–64` px rendering above the taskbar.
- Running and sleeping sprites normalized to the same visual scale.
- Optional manual animation pause.
- Optional pause on battery.
- Optional CPU, RAM, and battery information in the tray tooltip.
- Start with Windows.
- Single-instance guard.
- Tray recovery after Explorer restarts.
- Settings written atomically to `%APPDATA%\CatCPU\settings.ini`.

## Performance behavior

CPU is sampled with `GetSystemTimes`. RAM and power information are only sampled when a feature currently needs them, such as the Settings window, a related tooltip option, or battery-aware behavior.

The tray icon and tooltip are only sent back to Explorer when their visible content changes. Windows theme registry reads are event-driven rather than performed on every CPU sample.

When idle, manually paused, or paused by battery policy, the animation timer is stopped entirely.

## Settings

The Settings window is split into two compact sections.

**Appearance & animation**

- Cat theme: Automatic / Light / Dark.
- Start with Windows.
- Speed multiplier: `0.10×–5.00×`.
- Speed curve: Smooth / Linear / Reactive.
- Cat size: `12–64 px`.
- Smooth speed transitions.
- Invert CPU / speed.
- Manual pause.

**Idle, power & tray**

- Sleep threshold: `0–100%`.
- Wake hysteresis: `0–25%`.
- CPU sampling: `250–5000 ms`.
- Sleeping cat when idle.
- Pause animation on battery.
- Tooltip CPU / RAM / battery.
- Large overlay mode.

Resetting app settings does not silently change the Windows startup preference.

## Build on Windows

Install the Rust MSVC toolchain, then run:

```powershell
Set-ExecutionPolicy -Scope Process Bypass
.\build.ps1
```

The executable is produced at:

```text
target\release\catcpu.exe
```

GitHub Actions runs `cargo check`, unit tests, Clippy, and a release build on Windows, then uploads the executable as an artifact.

## Assets

Running frames are pinned to `runcat-dev/RunCat365@03b6e2b288c2df5df2433398f5547857bb4d0e2f`.

The sleeping cat is pinned to `runcat-dev/RunCatNeo@b3b1543049ea0a051ecb78654a45f144724ea737`.

The reused image assets remain subject to their upstream Apache-2.0 licensing terms. The license text is included under `LICENSES/Apache-2.0.txt`.
