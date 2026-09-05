# CatCPU

Minimal native Windows tray cat written in Rust. The cat runs faster as total CPU load increases and curls up to sleep when CPU usage is at or below the configured idle threshold.

CatCPU is a clean-room implementation. It does not copy source code from RunCat365, RunCatNeo, or GNOME RunCat. Only explicitly attributed cat image assets are reused under Apache-2.0. Asset provenance is documented below and the Apache-2.0 text is included under `LICENSES/`.

## Core behavior

- Native Windows notification-area application.
- CPU usage is sampled with `GetSystemTimes`.
- Five-frame running animation plus a dedicated sleeping-cat state.
- Running and sleeping sprites are normalized to the same apparent scale.
- Automatic contrast against the Windows taskbar theme.
- Manual light/dark theme override.
- No .NET, Electron, WebView, telemetry, background service, or third-party Rust crates.
- Settings are stored in `%APPDATA%\CatCPU\settings.ini`.
- Single-instance guard prevents duplicate tray cats.
- Tray icon is restored after Explorer restarts.

## Settings

Right-click the cat and choose **Settings...**. Changes apply without restarting CatCPU.

- Theme: Automatic / Light / Dark.
- Start with Windows: on/off.
- Speed multiplier: `0.10x` to `5.00x`.
- Speed curve: Smooth / Linear / Reactive.
- Cat size: `12` to `64` px.
- Idle / sleep threshold: `0` to `100` percent CPU.
- Idle hysteresis: `0` to `25` percentage points, to prevent rapid sleep/wake switching around the threshold.
- CPU sample interval: `250` to `5000` ms.
- Smooth speed changes: on/off.
- Invert CPU / speed: on/off.
- Sleeping cat when idle: on/off.
- Tooltip CPU: on/off.
- Tooltip RAM: on/off.
- Pause animation on battery: on/off.
- Large overlay mode: on/off.
- Live CPU, RAM, power and running/sleeping status.

Windows controls the physical size of notification-area icons. Values above 32 px therefore keep the tray icon at the largest size Windows accepts. Enable **Large overlay** to render the cat physically at 33-64 px just above the taskbar while keeping the tray icon available for controls.

The right-click menu includes common speed, size and threshold presets; custom values configured in the settings window remain valid.

## Build on Windows

Install the Rust MSVC toolchain, then run PowerShell from this directory:

```powershell
Set-ExecutionPolicy -Scope Process Bypass
.\build.ps1
```

`build.ps1` downloads only the required cat assets from pinned upstream commits, validates each one against its Git blob SHA, then runs:

```powershell
cargo build --release
```

The executable is produced at:

```text
target\release\catcpu.exe
```

GitHub Actions also builds the Windows release on pushes and pull requests and uploads `catcpu.exe` as a workflow artifact.

## Asset provenance

Running frames are pinned to `runcat-dev/RunCat365@03b6e2b288c2df5df2433398f5547857bb4d0e2f`.

The sleeping cat is pinned to `runcat-dev/RunCatNeo@b3b1543049ea0a051ecb78654a45f144724ea737`.

The reused image assets remain subject to their upstream Apache-2.0 licensing terms; see `LICENSES/Apache-2.0.txt`.
