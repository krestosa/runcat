#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("CatCPU is Windows-only.");
}

#[cfg(target_os = "windows")]
mod windows_app {
    use std::ffi::c_void;
    use std::fs;
    use std::mem::{size_of, zeroed};
    use std::os::windows::ffi::OsStrExt;
    use std::path::{Path, PathBuf};
    use std::ptr::{copy_nonoverlapping, null, null_mut};
    use std::sync::{Mutex, OnceLock};

    include!("windows_types.rs");
    include!("windows_ffi.rs");
    include!("windows_system.rs");
    include!("windows_render.rs");
    include!("windows_settings.rs");
    include!("windows_ui.rs");
    include!("windows_run.rs");
}

#[cfg(target_os = "windows")]
fn main() {
    windows_app::main();
}
