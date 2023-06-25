// Enumerates the way COD4 could be installed (Steam)
// Needs to be synced with `pub enum InstallType` in /src-tauri/src/lib.rs
export enum InstallType {
    STEAM = 'STEAM',
    UNKNOWN = 'UNKNOWN', // used when the install location was manually selected
}