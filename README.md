# CatCPU

Minimal native Windows tray cat written in Rust. The cat runs faster as total CPU load increases and curls up to sleep when CPU usage is at or below the configured idle threshold.

CatCPU is a clean-room implementation. It does not copy source code from RunCat365, RunCatNeo, or GNOME RunCat. Only explicitly attributed cat image assets are reused under Apache-2.0. Asset provenance is documented below and the Apache-2.0 text is included under `LICENSES/`.

## Core behavior

- Native Windows notification-area application.
- CPU usage is sampled with `GetSystemTimes`.
- Five-frame running animation.
- Dedicated sleeping-cat state while idle.
- Running and sleeping sprites are alpha-bounds normalized to the same visual scale while preserving aspect ratio.
- Automatic contrast against the Windows taskbar theme:
  - light taskbar -> black cat
  - dark taskbar -> white cat
- Manual theme override is available.
- No .NET, Electron, WebView, telemetry, background service, or third-party Rust crates.
- Settings are stored in `%APPDATA%\CatCPU\settings.ini`.
- Single-instance guard prevents duplicate tray cats.
- Tray icon is restored after Explorer restarts.

## Settings UI

Right-click the cat and choose **Settings...**. The settings window uses native Win32 controls and applies changes without restarting CatCPU.

Editable ranges are intentionally continuous rather than preset-only:

- Theme: Automatic / Light / Dark.
- Start with Windows: on/off.
- Speed multiplier: `0.10x` to `5.00x`.
- Cat size: `12` to `32` px inside the Windows tray icon canvas.
- Idle / sleep threshold: `0` to `100` percent CPU.
- CPU sample interval: `250` to `5000` ms.
- Smooth speed changes: on/off.
- Invert CPU / speed: on/off.
- Show sleeping cat when idle: on/off.
- Live CPU / running / sleeping status.

The right-click submenus still expose a few common values as quick shortcuts; custom values configured in the settings window remain valid even when they do not match a shortcut.

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

## Asset provenance

Running frames are pinned to `runcat-dev/RunCat365@03b6e2b288c2df5df2433398f5547857bb4d0e2f`.

The sleeping cat is pinned to `runcat-dev/RunCatNeo@b3b1543049ea0a051ecb78654a45f144724ea737`.

The reused image assets remain subject to their upstream Apache-2.0 licensing terms; see `LICENSES/Apache-2.0.txt`.
