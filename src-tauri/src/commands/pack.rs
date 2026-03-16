use crate::git::{get_changed_file_paths, open_repo};
use crate::models::{FileChange, PackOptions, PackResult};
use crate::pack::{collect_files, create_tar_gz, create_zip};
use std::path::Path;
use tauri::{command, AppHandle, Emitter};

/// 打包本地变更
#[command]
pub async fn pack_local_changes(
    repo_path: String,
    changes: Vec<FileChange>,
    options: PackOptions,
    app_handle: AppHandle,
) -> Result<PackResult, String> {
    let repo = open_repo(&repo_path).map_err(|e| e.to_string())?;

    // 获取变更文件的绝对路径
    let file_paths = get_changed_file_paths(&repo, &changes).map_err(|e| e.to_string())?;

    // 收集所有文件
    let all_files = collect_files(&file_paths);

    if all_files.is_empty() {
        return Err("No files to pack".to_string());
    }

    let base_dir = Path::new(&repo_path);
    let output_path = Path::new(&options.output_path);

    // 确保输出目录存在
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    let progress_callback = |progress: crate::models::PackProgress| {
        let _ = app_handle.emit("pack-progress", progress);
    };

    let result = match options.format {
        crate::models::PackFormat::Zip => {
            create_zip(output_path, &all_files, base_dir, &options, Some(progress_callback))
        }
        crate::models::PackFormat::TarGz => {
            create_tar_gz(output_path, &all_files, base_dir, &options, Some(progress_callback))
        }
    };

    result.map_err(|e| e.to_string())
}

/// 选择输出目录
#[command]
pub async fn select_output_directory() -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;

    // 返回默认路径，实际选择由前端调用 dialog 插件完成
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());

    Ok(home)
}

/// 在文件资源管理器中打开路径
#[command]
pub async fn open_in_explorer(path: String) -> Result<(), String> {
    let path = Path::new(&path);
    
    if !path.exists() {
        return Err(format!("路径不存在: {}", path.display()));
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer.exe")
            .arg(path)
            .spawn()
            .map_err(|e| format!("无法打开文件资源管理器: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("无法打开 Finder: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("无法打开文件管理器: {}", e))?;
    }
    
    Ok(())
}
