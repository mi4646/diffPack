use crate::error::Result;
use crate::models::{PackOptions, PackProgress, PackResult};
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use tar::Builder;

/// 创建 tar.gz 打包
pub fn create_tar_gz(
    output_path: &Path,
    source_files: &[std::path::PathBuf],
    base_dir: &Path,
    options: &PackOptions,
    progress_callback: Option<impl Fn(PackProgress) + Send + Sync>,
) -> Result<PackResult> {
    let start = Instant::now();
    let total = source_files.len();
    let file = File::create(output_path)?;
    let enc = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut tar = Builder::new(enc);

    let mut processed = 0;

    for file_path in source_files {
        if file_path.is_file() {
            let relative_path = file_path.strip_prefix(base_dir)
                .unwrap_or(file_path);

            if let Some(cb) = &progress_callback {
                cb(PackProgress {
                    current_file: relative_path.to_string_lossy().to_string(),
                    processed,
                    total,
                    percentage: (processed as f32 / total as f32) * 100.0,
                });
            }

            tar.append_path_with_name(file_path, relative_path)?;
        }

        processed += 1;
    }

    tar.finish()?;

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
