use crate::error::{AppError, Result};
use crate::models::AuthMethod;
use russh::client::Handle;
use russh::keys::{PrivateKey, PrivateKeyWithHashAlg};
use std::sync::Arc;

use super::session::ClientHandler;

/// 对 SSH 会话执行认证
pub async fn authenticate(
    session: &mut Handle<ClientHandler>,
    username: &str,
    auth_method: &AuthMethod,
) -> Result<()> {
    match auth_method {
        AuthMethod::Password { password } => {
            let auth_result = session
                .authenticate_password(username, password)
                .await
                .map_err(|e| AppError::Ssh(format!("Password auth failed: {}", e)))?;

            if !auth_result.success() {
                return Err(AppError::Ssh("Password authentication failed".to_string()));
            }
        }
        AuthMethod::KeyFile { key_path, passphrase } => {
            // 使用 russh::keys::PrivateKey 读取密钥文件
            // 展开路径中的 ~ 符号
            let expanded_path = if key_path.starts_with('~') {
                if let Some(home_dir) = dirs::home_dir() {
                    home_dir.join(key_path.strip_prefix("~").unwrap())
                } else {
                    std::path::PathBuf::from(key_path)
                }
            } else {
                std::path::PathBuf::from(key_path)
            };

            let key_data = std::fs::read(&expanded_path)
                .map_err(|e| AppError::Ssh(format!("Failed to read key file '{}': {}", expanded_path.display(), e)))?;

            // 尝试多种方式解析密钥，提高兼容性
            let key_pair = match PrivateKey::from_openssh(&key_data) {
                Ok(key) => key,
                Err(openssh_err) => {
                    // 尝试从PEM格式解析
                    match PrivateKey::from_bytes(&key_data) {
                        Ok(key) => key,
                        Err(bytes_err) => {
                            // 尝试PPK格式（PuTTY密钥）
                            if let Ok(key_str) = String::from_utf8(key_data.clone()) {
                                match PrivateKey::from_ppk(&key_str, passphrase.clone()) {
                                    Ok(key) => key,
                                    Err(ppk_err) => {
                                        // 所有解析方式都失败
                                        return Err(AppError::Ssh(format!(
                                            "Failed to parse key file: \n- OpenSSH format: {}\n- Raw bytes: {}\n- PPK format: {}",
                                            openssh_err, bytes_err, ppk_err
                                        )));
                                    }
                                }
                            } else {
                                return Err(AppError::Ssh(format!(
                                    "Failed to parse key file: \n- OpenSSH format: {}\n- Raw bytes: {}",
                                    openssh_err, bytes_err
                                )));
                            }
                        }
                    }
                }
            };

            // 包装成 PrivateKeyWithHashAlg
            let key_with_alg = PrivateKeyWithHashAlg::new(Arc::new(key_pair), None);

            let auth_result = session
                .authenticate_publickey(username, key_with_alg)
                .await
                .map_err(|e| AppError::Ssh(format!("Key file authentication failed: {}", e)))?;

            if !auth_result.success() {
                return Err(AppError::Ssh(
                    "Key file authentication failed. Please check:\n\
                    1. The public key is added to ~/.ssh/authorized_keys on the server\n\
                    2. The username is correct\n\
                    3. The key file has correct permissions (should be readable only by you)\n\
                    4. The key format is supported (OpenSSH, PEM, PPK)".to_string()
                ));
            }
        }
        AuthMethod::SshAgent => {
            // SSH Agent 认证暂不支持，返回错误提示用户使用密钥文件认证
            return Err(AppError::Ssh("SSH agent authentication is not yet supported. Please use password or key file authentication.".to_string()));
        }
    }

    Ok(())
}
