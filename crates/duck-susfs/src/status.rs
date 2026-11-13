//! Status payload for Home / Info tabs.

use serde::Serialize;

use crate::config::SusfsConfig;
use crate::custom::CustomLists;
use crate::susfs::SusfsTool;
use duck_core::Context;

#[derive(Debug, Serialize)]
pub struct SusfsStatus {
    pub susfs_available: bool,
    pub susfs_version: Option<String>,
    pub susfs_variant: Option<String>,
    pub enabled_features: Vec<String>,
    pub enabled_features_count: usize,
    /// BRENE reports total kernel features as 10 (including priority UID filter).
    pub susfs_total_features: u32,
    pub config: SusfsConfig,
    pub custom: CustomLists,
    pub device_model: Option<String>,
    pub kernel_release: Option<String>,
    pub android_release: Option<String>,
    pub android_sdk: Option<String>,
    pub custom_rom_hint: Option<bool>,
}

pub fn collect(ctx: &Context) -> anyhow::Result<SusfsStatus> {
    let tool = SusfsTool::from_env();
    let available = tool.available();
    let (version, variant, features) = if available {
        (
            tool.show_version().ok(),
            tool.show_variant().ok(),
            tool.show_enabled_features().unwrap_or_default(),
        )
    } else {
        (None, None, Vec::new())
    };
    let count = features.len();
    let config = SusfsConfig::load(ctx)?;
    let custom = CustomLists::load(ctx)?;

    Ok(SusfsStatus {
        susfs_available: available,
        susfs_version: version,
        susfs_variant: variant,
        enabled_features_count: count,
        enabled_features: features,
        susfs_total_features: 10,
        config,
        custom,
        device_model: prop("ro.product.model"),
        kernel_release: read_kernel_release(),
        android_release: prop("ro.build.version.release"),
        android_sdk: prop("ro.build.version.sdk"),
        custom_rom_hint: detect_custom_rom(),
    })
}

fn prop(key: &str) -> Option<String> {
    let out = std::process::Command::new("resetprop")
        .arg(key)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

fn read_kernel_release() -> Option<String> {
    let text = std::fs::read_to_string("/proc/version").ok()?;
    // First token that looks like x.y.z
    text.split_whitespace()
        .find_map(|t| {
            let mut parts = t.split('.');
            if parts.next()?.chars().all(|c| c.is_ascii_digit())
                && parts.next()?.chars().all(|c| c.is_ascii_digit())
            {
                Some(t.to_string())
            } else {
                None
            }
        })
        .or_else(|| {
            std::process::Command::new("uname")
                .arg("-r")
                .output()
                .ok()
                .and_then(|o| {
                    let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                    if s.is_empty() { None } else { Some(s) }
                })
        })
}

fn detect_custom_rom() -> Option<bool> {
    // Light check used by BRENE WebUI (lineage string under /system).
    let out = std::process::Command::new("sh")
        .args(["-c", r#"find /system -iname '*lineage*' 2>/dev/null | head -1"#])
        .output()
        .ok()?;
    Some(!String::from_utf8_lossy(&out.stdout).trim().is_empty())
}
