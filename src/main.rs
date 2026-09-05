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
    use std::path::PathBuf;
    use std::ptr::{copy_nonoverlapping, null, null_mut};
    use std::sync::{Mutex, OnceLock};

    include!("windows_ffi_a.rs");
    include!("windows_ffi_b.rs");
    include!("windows_ffi_c.rs");
    include!("windows_features.rs");
    include!("windows_core_a.rs");
    include!("windows_core_b.rs");
    include!("windows_core_c.rs");
    include!("windows_settings_a.rs");
    include!("windows_settings_b.rs");
    include!("windows_ui_a.rs");
    include!("windows_ui_b.rs");
    include!("windows_run.rs");
}

#[cfg(target_os = "windows")]
fn main() {
    windows_app::main();
}
