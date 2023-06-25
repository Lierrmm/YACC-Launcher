use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{cell::RefCell, time::Instant};
use ts_rs::TS;

use crate::constants::COD4_STEAM_ID;
use crate::{utils::extract, GameInstall, InstallType};

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
enum InstallState {
    Downloading,
    Extracting,
    Done,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct InstallProgress {
    current_downloaded: u64,
    total_size: u64,
    state: InstallState,
}

/// Attempts to find the game install location
#[tauri::command]
pub fn find_game_install_location() -> Result<GameInstall, String> {
    // Attempt parsing Steam library directly
    match steamlocate::SteamDir::locate() {
        Some(mut steamdir) => {
            let cod4_steamid = COD4_STEAM_ID.parse().unwrap();
            match steamdir.app(&cod4_steamid) {
                Some(app) => {
                    let game_install = GameInstall {
                        game_path: app.path.to_str().unwrap().to_string(),
                        install_type: InstallType::STEAM,
                    };
                    return Ok(game_install);
                }
                None => log::info!("Couldn't locate COD4 Steam install"),
            }
        }
        None => log::info!("Couldn't locate Steam on this computer!"),
    }

    Err("Could not auto-detect game install location! Please enter it manually.".to_string())
}
