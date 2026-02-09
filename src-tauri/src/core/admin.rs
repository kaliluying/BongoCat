use std::{env, process::Command};

use tauri::{AppHandle, Runtime, command};

#[cfg(target_os = "windows")]
#[command]
pub async fn restart_as_admin<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    let current_exe = env::current_exe()
        .map_err(|err| format!("Failed to locate current executable: {err}"))?;

    let exe_path = current_exe.to_string_lossy().replace('\'', "''");

    let script = format!("Start-Process -FilePath '{exe_path}' -Verb RunAs");

    let status = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .status()
        .map_err(|err| format!("Failed to start elevated process: {err}"))?;

    if !status.success() {
        return Err("Failed to request administrator privileges".into());
    }

    app_handle.exit(0);

    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[command]
pub async fn restart_as_admin<R: Runtime>(_app_handle: AppHandle<R>) -> Result<(), String> {
    Err("Run as administrator is only supported on Windows".into())
}
