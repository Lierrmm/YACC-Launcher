//! This module deals with handling things around YACC such as
//! - getting version number
pub mod install;

use crate::{get_host_os, GameInstall, InstallType};
use anyhow::anyhow;

/// Launches YACC
#[tauri::command]
pub fn launch_yacc(game_install: GameInstall) -> Result<String, String> {
    dbg!(game_install.clone());

    let host_os = get_host_os();

    // Explicitly fail early certain (currently) unsupported install setups
    if host_os != "windows" || !(matches!(game_install.install_type, InstallType::STEAM)) {
        return Err(format!(
            "Not yet implemented for \"{}\" with COD4 installed via \"{:?}\"",
            get_host_os(),
            game_install.install_type
        ));
    }

    // Switch to COD4 directory for launching
    // yacc.exe expects to be run from that folder
    if std::env::set_current_dir(game_install.game_path.clone()).is_err() {
        // We failed to get to COD4 directory
        return Err(anyhow!("Couldn't access COD4 directory").to_string());
    }

    // Only Windows with Steam or Origin are supported at the moment
    if host_os == "windows" && (matches!(game_install.install_type, InstallType::STEAM)) {
        let ns_exe_path = format!("{}/YACC.exe", game_install.game_path);
        let _output = std::process::Command::new("C:\\Windows\\System32\\cmd.exe")
            .args(["/C", "start", "", &ns_exe_path])
            .spawn()
            .expect("failed to execute process");
        return Ok("Launched game".to_string());
    }

    Err(format!(
        "Not yet implemented for {:?} on {}",
        game_install.install_type,
        get_host_os()
    ))
}
