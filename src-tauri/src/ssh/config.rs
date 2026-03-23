use crate::error::Result;
use crate::models::SshConfigEntry;
use std::path::Path;

/// 解析 SSH Config 文件
pub fn parse_ssh_config_file(path: &Path) -> Result<Vec<SshConfigEntry>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(path)?;
    let mut entries: Vec<SshConfigEntry> = Vec::new();
    let mut current_entry: Option<SshConfigEntry> = None;

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].to_lowercase();
        let value = parts[1].trim();

        match key.as_str() {
            "host" => {
                if let Some(entry) = current_entry.take() {
                    entries.push(entry);
                }
                current_entry = Some(SshConfigEntry {
                    host: value.to_string(),
                    hostname: None,
                    port: None,
                    user: None,
                    identity_file: None,
                });
            }
            "hostname" => {
                if let Some(ref mut entry) = current_entry {
                    entry.hostname = Some(value.to_string());
                }
            }
            "port" => {
                if let Some(ref mut entry) = current_entry {
                    entry.port = value.parse().ok();
                }
            }
            "user" => {
                if let Some(ref mut entry) = current_entry {
                    entry.user = Some(value.to_string());
                }
            }
            "identityfile" => {
                if let Some(ref mut entry) = current_entry {
                    entry.identity_file = Some(value.to_string());
                }
            }
            _ => {}
        }
    }

    if let Some(entry) = current_entry {
        entries.push(entry);
    }

    Ok(entries)
}

/// 获取默认 SSH Config 路径
pub fn get_default_ssh_config_path() -> std::path::PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".ssh").join("config"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.ssh/config"))
}
