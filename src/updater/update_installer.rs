use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "macos")]
pub fn install_update(update_file: PathBuf) -> Result<(), Box<dyn Error>> {
    eprintln!("[UPDATE] Installing macOS update from: {:?}", update_file);

    let extract_dir = std::env::temp_dir().join("calm_update_extract");
    if extract_dir.exists() {
        std::fs::remove_dir_all(&extract_dir)?;
    }
    std::fs::create_dir_all(&extract_dir)?;

    let status = Command::new("tar")
        .args([
            "-xzf",
            update_file.to_str().unwrap(),
            "-C",
            extract_dir.to_str().unwrap(),
        ])
        .status()?;

    if !status.success() {
        return Err("Failed to extract update archive".into());
    }

    let app_bundle = extract_dir.join("Calm.app");
    if !app_bundle.exists() {
        return Err("Calm.app not found in update archive".into());
    }

    let target_app = PathBuf::from("/Applications/Calm.app");

    if target_app.exists() {
        let backup = PathBuf::from("/Applications/Calm.app.bak");
        if backup.exists() {
            std::fs::remove_dir_all(&backup)?;
        }
        std::fs::rename(&target_app, &backup)?;
    }

    let status = Command::new("cp")
        .args(["-R", app_bundle.to_str().unwrap(), "/Applications/"])
        .status()?;

    if !status.success() {
        return Err("Failed to install update to /Applications".into());
    }

    eprintln!("[UPDATE] Update installed successfully. Restart Calm to use the new version.");

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn install_update(update_file: PathBuf) -> Result<(), Box<dyn Error>> {
    eprintln!("[UPDATE] Installing Linux update from: {:?}", update_file);

    let extract_dir = std::env::temp_dir().join("calm_update_extract");
    if extract_dir.exists() {
        std::fs::remove_dir_all(&extract_dir)?;
    }
    std::fs::create_dir_all(&extract_dir)?;

    let status = Command::new("tar")
        .args([
            "-xzf",
            update_file.to_str().unwrap(),
            "-C",
            extract_dir.to_str().unwrap(),
        ])
        .status()?;

    if !status.success() {
        return Err("Failed to extract update archive".into());
    }

    let new_binary = extract_dir.join("calm-linux/calm");
    if !new_binary.exists() {
        return Err("calm binary not found in update archive".into());
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let target_binary = PathBuf::from(home).join(".local/bin/calm");

    if target_binary.exists() {
        let backup = target_binary.with_extension("bak");
        std::fs::copy(&target_binary, &backup)?;
    }

    std::fs::copy(&new_binary, &target_binary)?;

    let _ = Command::new("chmod")
        .args(["+x", target_binary.to_str().unwrap()])
        .status();

    eprintln!("[UPDATE] Update installed successfully. Restart Calm to use the new version.");

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn install_update(update_file: PathBuf) -> Result<(), Box<dyn Error>> {
    eprintln!("[UPDATE] Installing Windows update from: {:?}", update_file);

    let extract_dir = std::env::temp_dir().join("calm_update_extract");
    if extract_dir.exists() {
        std::fs::remove_dir_all(&extract_dir)?;
    }
    std::fs::create_dir_all(&extract_dir)?;

    let status = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Expand-Archive -Path '{}' -DestinationPath '{}'",
                update_file.display(),
                extract_dir.display()
            ),
        ])
        .status()?;

    if !status.success() {
        return Err("Failed to extract update archive".into());
    }

    let new_binary = extract_dir.join("calm-windows/calm.exe");
    if !new_binary.exists() {
        return Err("calm.exe not found in update archive".into());
    }

    let current_exe = std::env::current_exe()?;
    let backup = current_exe.with_extension("exe.bak");

    std::fs::copy(&current_exe, &backup)?;

    std::fs::copy(&new_binary, &current_exe)?;

    eprintln!("[UPDATE] Update installed successfully. Restart Calm to use the new version.");

    Ok(())
}
