use serde::{Deserialize, Serialize};

/// 打包格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PackFormat {
    Zip,
    TarGz,
}

/// 打包选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackOptions {
    pub format: PackFormat,
    pub output_path: String,
    pub include_deleted: bool,
    pub preserve_structure: bool,
    pub base_dir: Option<String>,
}

/// 打包进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackProgress {
    pub current_file: String,
    pub processed: usize,
    pub total: usize,
    pub percentage: f32,
}

/// 打包结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackResult {
    pub success: bool,
    pub output_path: String,
    pub file_count: usize,
    pub total_size: u64,
    pub duration_ms: u64,
    pub error: Option<String>,
}
