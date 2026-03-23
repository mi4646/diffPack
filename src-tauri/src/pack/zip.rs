use crate::error::{AppError, Result};
use crate::models::{PackOptions, PackProgress, PackResult};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

/// 创建 ZIP 打包
pub fn create_zip(
    output_path: &Path,
    source_files: &[std::path::PathBuf],
    base_dir: &Path,
    _options: &PackOptions,
    progress_callback: Option<impl Fn(PackProgress) + Send + Sync>,
) -> Result<PackResult> {
    let start = Instant::now();
    let total = source_files.len();
    let file = File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options: zip::write::FileOptions<'_, ()> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut processed = 0;
    let mut _total_size: u64 = 0;

    for file_path in source_files {
        if file_path.is_file() {
            let relative_path = file_path
                .strip_prefix(base_dir)
                .unwrap_or(file_path)
                .to_string_lossy()
                .replace('\\', "/");

            if let Some(cb) = &progress_callback {
                cb(PackProgress {
                    current_file: relative_path.clone(),
                    processed,
                    total,
                    percentage: (processed as f32 / total as f32) * 100.0,
                });
            }

            zip.start_file(&relative_path, options)
                .map_err(|e| AppError::Pack(e.to_string()))?;
            let data = std::fs::read(file_path)?;
            zip.write_all(&data)?;
            _total_size += data.len() as u64;
        }

        processed += 1;
    }

    // 发送 100% 完成事件
    if let Some(cb) = &progress_callback {
        cb(PackProgress {
            current_file: String::new(),
            processed: total,
            total,
            percentage: 100.0,
        });
    }

    zip.finish().map_err(|e| AppError::Pack(e.to_string()))?;

    let duration = start.elapsed();
    let output_metadata = std::fs::metadata(output_path)?;

    Ok(PackResult {
        success: true,
        output_path: output_path.to_string_lossy().to_string(),
        file_count: processed,
        total_size: output_metadata.len(),
        duration_ms: duration.as_millis() as u64,
        error: None,
    })
}

/// 收集目录下所有文件
pub fn collect_files(paths: &[std::path::PathBuf]) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    for path in paths {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    files
}
