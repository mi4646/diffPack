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
            let privkey_path = std::path::Path::new(key_path);
            let key_data = std::fs::read(privkey_path)?;

            // 尝试解析密钥，如果是加密的且有密码短语则使用密码
            let key_pair = match PrivateKey::from_openssh(&key_data) {
                Ok(key) => key,
                Err(e) if e.to_string().contains("encrypted") && passphrase.is_some() => {
                    // 对于加密密钥，russh 0.58 的 ssh-key 版本需要使用 with_passphrase
                    // 临时实现：先返回错误提示用户暂时使用无密码密钥或密码认证
                    return Err(AppError::Ssh("Encrypted private keys are not yet supported in this version. Please use an unencrypted key or password authentication.".to_string()));
                }
                Err(e) => {
                    return Err(AppError::Ssh(format!("Failed to read key file: {}", e)));
                }
            };

            // 包装成 PrivateKeyWithHashAlg
            let key_with_alg = PrivateKeyWithHashAlg::new(Arc::new(key_pair), None);

            let auth_result = session
                .authenticate_publickey(username, key_with_alg)
                .await
                .map_err(|e| AppError::Ssh(format!("Key file auth failed: {}", e)))?;

            if !auth_result.success() {
                return Err(AppError::Ssh("Key file authentication failed".to_string()));
            }
        }
        AuthMethod::SshAgent => {
            // SSH Agent 认证暂不支持，返回错误提示用户使用密钥文件认证
            return Err(AppError::Ssh("SSH agent authentication is not yet supported. Please use password or key file authentication.".to_string()));
        }
    }

    Ok(())
}
