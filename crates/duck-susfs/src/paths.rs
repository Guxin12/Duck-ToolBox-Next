//! On-device paths used by BRENE and by this feature.
//! Data stays under `/data/adb/brene` for compatibility with existing installs.

use std::path::{Path, PathBuf};

use duck_core::Context;

/// Persistent data directory (BRENE-compatible).
pub fn data_dir(_ctx: &Context) -> PathBuf {
    PathBuf::from("/data/adb/brene")
}

pub fn config_path(ctx: &Context) -> PathBuf {
    data_dir(ctx).join("config.toml")
}

/// Legacy shell config (read for migration only).
pub fn legacy_config_path(ctx: &Context) -> PathBuf {
    data_dir(ctx).join("config.sh")
}

pub fn custom_file(ctx: &Context, name: &str) -> PathBuf {
    data_dir(ctx).join(name)
}

pub fn susfs_bin() -> PathBuf {
    PathBuf::from("/data/adb/ksu/bin/susfs")
}

pub fn ksud_bin() -> PathBuf {
    PathBuf::from("/data/adb/ksud")
}

pub fn ensure_data_dir(ctx: &Context) -> anyhow::Result<()> {
    let dir = data_dir(ctx);
    if !dir.is_dir() {
        std::fs::create_dir_all(&dir)?;
        // Owner-only, same idea as BRENE.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
        }
    }
    Ok(())
}

pub fn read_lines(path: &Path) -> anyhow::Result<Vec<String>> {
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path)?;
    Ok(text
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect())
}

pub fn write_lines(path: &Path, lines: &[String]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut body = lines.join("\n");
    if !body.is_empty() {
        body.push('\n');
    }
    std::fs::write(path, body)?;
    Ok(())
}
