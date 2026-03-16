pub mod commands;
pub mod error;
pub mod git;
pub mod models;
pub mod pack;
pub mod ssh;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Git commands
            commands::git::open_local_repo,
            commands::git::get_commits,
            commands::git::get_diff_between_commits,
            commands::git::get_commits_by_date_range,
            // Pack commands
            commands::pack::pack_local_changes,
            commands::pack::select_output_directory,
            commands::pack::open_in_explorer,
            // SSH commands
            commands::ssh::parse_ssh_config,
            commands::ssh::test_ssh_connection,
            commands::ssh::connect_ssh,
            commands::ssh::disconnect_ssh,
            commands::ssh::get_remote_commits,
            commands::ssh::get_remote_diff,
            commands::ssh::pack_remote_changes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
