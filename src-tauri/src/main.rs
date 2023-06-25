// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};

#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_OK, MB_USERICON};

use crate::constants::REFRESH_DELAY;

mod utils;
mod yacc;

use semver::Version;
use serde::{Deserialize, Serialize};
use tauri::{Manager, Runtime};
use tokio::time::sleep;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct YACCThunderstoreRelease {
    package: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct YACCThunderstoreReleaseWrapper {
    label: String,
    value: YACCThunderstoreRelease,
}

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

fn main() {
    // Setup logger
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters("info");
    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    // Only enable Sentry crash logs on release
    #[cfg(not(debug_assertions))]
    let _guard = sentry::init((
        "https://08b31609761a4cfcaed44126c3ad8877@o1007591.ingest.sentry.io/4505418582851584",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            attach_stacktrace: true,
            ..Default::default()
        },
    ));

    match tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_millis(2000)).await;
                    app_handle.emit_all("backend-ping", "ping").unwrap();
                }
            });
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_millis(2000)).await;
                    app_handle
                        .emit_all("yacc-running-ping", utils::check_yacc_running())
                        .unwrap();
                }
            });

            //Emit updated player and server count to GUI
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(REFRESH_DELAY).await;
                    app_handle
                        .emit_all("yacc-statistics", utils::get_server_player_count().await)
                        .unwrap();
                }
            });

            Ok(())
        })
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![
            utils::force_panic,
            yacc::install::find_game_install_location,
            // get_yacc_launcher_version_number,
            // yacc::get_yacc_version_number,
            // check_is_yacc_outdated,
            verify_install_location,
            get_host_os,
            // install_yacc_caller,
            // update_yacc,
            yacc::launch_yacc,
            // github::release_notes::check_is_yacc_launcher_outdated,
            utils::is_debug_mode,
            // github::release_notes::get_yacc_release_notes,
            // install_mod_caller,
            // github::release_notes::get_newest_yacc_launcher_version,
            utils::get_server_player_count,
            // github::get_list_of_tags,
            // github::compare_tags,
            // github::pull_requests::get_pull_requests_wrapper,
            // github::pull_requests::apply_launcher_pr,
            // github::pull_requests::apply_mods_pr,
            // github::pull_requests::get_launcher_download_link,
        ])
        .run(tauri::generate_context!())
    {
        Ok(()) => (),
        Err(err) => {
            // Failed to launch system native web view

            // Log error on Linux
            #[cfg(not(target_os = "windows"))]
            {
                log::error!("{err}");
            }

            // On Windows we can show an error window using Windows API to show how to install WebView2
            #[cfg(target_os = "windows")]
            {
                log::error!("WebView2 not installed: {err}");
                // Display a message box to the user with a button to open the installation instructions
                let title = "WebView2 not found"
                    .encode_utf16()
                    .chain(Some(0))
                    .collect::<Vec<_>>();
                let message = "YACC-Launcher requires WebView2 to run.\n\nClick OK to open installation instructions.".encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                unsafe {
                    let result = MessageBoxW(
                        null_mut(),
                        message.as_ptr(),
                        title.as_ptr(),
                        MB_OK | MB_ICONERROR | MB_USERICON,
                    );
                    if result == 1 {
                        // Open the installation instructions URL in the user's default web browser
                        open::that("https://github.com/Lierrmm/YACC-Launcher").unwrap();
                    }
                }
            }
        }
    };
}

/// Returns the current version number as a string
#[tauri::command]
async fn get_launcher_version_number() -> String {
    let version = env!("CARGO_PKG_VERSION");
    if cfg!(debug_assertions) {
        // Debugging enabled
        format!("v{} (debug mode)", version)
    } else {
        // Debugging disabled
        format!("v{}", version)
    }
}

/// Helps with converting release candidate numbers
pub fn convert_release_candidate_number(version_number: String) -> String {
    // This simply converts `-rc` to `0`
    // Works as intended for RCs < 10, e.g.  `v1.9.2-rc1`  -> `v1.9.201`
    // Doesn't work for larger numbers, e.g. `v1.9.2-rc11` -> `v1.9.2011` (should be `v1.9.211`)
    version_number.replace("-rc", "0").replace("00", "")
}

/// Checks if is valid Titanfall2 install based on certain conditions
#[tauri::command]
async fn verify_install_location(game_path: String) -> bool {
    match check_is_valid_game_path(&game_path) {
        Ok(()) => true,
        Err(err) => {
            log::warn!("{}", err);
            false
        }
    }
}

/// Installs YACC to the given path
// #[tauri::command]
// async fn install_yacc_caller(
//     window: tauri::Window,
//     game_path: String,
//     northstar_package_name: Option<String>,
//     version_number: Option<String>,
// ) -> Result<bool, String> {
//     log::info!("Running YACC install");

//     // Get Northstar package name (`Northstar` vs `NorthstarReleaseCandidate`)
//     let northstar_package_name = northstar_package_name
//         .map(|name| {
//             if name.len() <= 1 {
//                 "YACC".to_string()
//             } else {
//                 name
//             }
//         })
//         .unwrap_or("YACC".to_string());

//     match yacc::install::install_northstar(
//         window,
//         &game_path,
//         northstar_package_name,
//         version_number,
//     )
//     .await
//     {
//         Ok(_) => Ok(true),
//         Err(err) => {
//             log::error!("{}", err);
//             Err(err)
//         }
//     }
// }

// The remaining below was originally in `lib.rs`.
// As this was causing issues it was moved into `main.rs` until being later moved into dedicated modules
use anyhow::Result;

pub mod constants;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InstallType {
    STEAM,
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInstall {
    pub game_path: String,
    pub install_type: InstallType,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct YACCMod {
    pub name: String,
    pub version: Option<String>,
    pub enabled: bool,
    pub directory: String,
}

/// Checks whether the provided path is a valid iw3mp gamepath by checking against a certain set of criteria
pub fn check_is_valid_game_path(game_install_path: &str) -> Result<(), String> {
    let path_to_iw3mp_exe = format!("{game_install_path}/iw3mp.exe");
    let is_correct_game_path = std::path::Path::new(&path_to_iw3mp_exe).exists();
    log::info!("iw3mp.exe exists in path? {}", is_correct_game_path);

    // Exit early if wrong game path
    if !is_correct_game_path {
        return Err(format!("Incorrect game path \"{game_install_path}\"")); // Return error cause wrong game path
    }
    Ok(())
}

/// Returns identifier of host OS YACC-Launcher is running on
#[tauri::command]
fn get_host_os() -> String {
    env::consts::OS.to_string()
}
