use crate::db::Database;
use std::path::PathBuf;
use std::process::exit;
use tauri::AppHandle;

/// A simple helper function to make connecting to the database easier.
///
/// # Arguments
///
/// * `app` - The Tauri AppHandle.
///
/// # Returns
///
/// A new `Database` struct.
pub fn connect_to_db(app: AppHandle) -> Database {
    let app_data_dir = get_app_data_dir(app);

    let mut database = Database::default();
    if let Err(e) = database.init(app_data_dir, false) {
        println!("{e:?}");
        exit(1);
    }

    database
}

/// A simple helper function to get the application data directory.
///
/// # Arguments
///
/// * `app` - The Tauri AppHandle.
///
/// # Returns
///
/// The application data directory as a `PathBuf`.
pub fn get_app_data_dir(app: AppHandle) -> PathBuf {
    let app_data_dir = app.path_resolver().app_data_dir();
    match app_data_dir {
        Some(d) => d,
        None => panic!("Failed to get application data directory!"),
    }
}
