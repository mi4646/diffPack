use crate::error::{AppError, Result};
use crate::models::AuthMethod;
use std::path::Path;

/// 对 SSH 会话执行认证
pub fn authenticate(session: &ssh2::Session, username: &str, auth_method: &AuthMethod) -> Result<()> {
    match auth_method {
        AuthMethod::Password { password } => {
            session
                .userauth_password(username, password)
                .map_err(|e| AppError::Ssh(format!("Password auth failed: {}", e)))?;
        }
        AuthMethod::KeyFile { key_path, passphrase } => {
            let privkey_path = Path::new(key_path);
            let passphrase_str = passphrase.as_deref();
            session
                .userauth_pubkey_file(username, None, privkey_path, passphrase_str)
                .map_err(|e| AppError::Ssh(format!("Key file auth failed: {}", e)))?;
        }
        AuthMethod::SshAgent => {
            session
                .userauth_agent(username)
                .map_err(|e| AppError::Ssh(format!("SSH agent auth failed: {}", e)))?;
        }
    }
    Ok(())
}
