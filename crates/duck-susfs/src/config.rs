//! Configuration model. Field names and defaults match BRENE v0.0.75 `config.sh`.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::fail;
use crate::paths::{self, ensure_data_dir};
use duck_core::Context;

/// All toggles and string options that BRENE stores in config.sh.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "snake_case")]
pub struct SusfsConfig {
    // --- Path hiding ---
    pub paths_hiding_data_local_tmp: bool,
    pub paths_hiding_non_standard_sdcard: bool,
    pub paths_hiding_non_standard_sdcard_android: bool,

    // --- Hiding ---
    pub hide_custom_recovery: bool,
    pub hide_ksu_directories: bool,
    pub hide_third_party_apps: bool,
    pub hide_suspicious_pty: bool,
    pub hide_addon_d: bool,
    pub hide_custom_rom_paths: bool,
    pub hide_custom_rom_paths_2: bool,
    pub hide_framework_res_apk: bool,
    pub hide_lineage_strings: bool,
    pub hide_sus_mnts_for_non_su_procs: bool,

    // --- SuSFS / KSU ---
    pub enable_avc_log_spoofing: bool,
    pub su_compat: bool,
    pub kernel_umount: bool,
    pub selinux_hide: bool,
    pub shrink_buffer_size: bool,
    pub fix_data_local_tmp_inconsistencies: bool,

    // --- Spoofing ---
    pub spoof_uname: bool,
    pub custom_spoof_uname: bool,
    pub custom_uname_kernel_release: String,
    pub custom_uname_kernel_version: String,
    pub spoof_system_properties: bool,
    pub spoof_os_security_patch_level_property: bool,
    pub spoof_vendor_security_patch_level_property: bool,
    pub spoof_cmdline_or_bootconfig: bool,
    pub spoof_hosts: bool,
    pub spoof_libstagefright: bool,
    pub spoof_verified_boot_hash: String,

    // --- Android / system ---
    pub developer_options: bool,
    pub usb_debugging: bool,
    pub wireless_debugging: bool,
    pub saturation: bool,
    pub show_refresh_rate: bool,
    pub disable_child_process_restrictions: bool,

    // --- ROM / misc ---
    pub pif_props: bool,
    pub rom_props: bool,
    pub brene_logs: bool,
    pub enable_log: bool,
}

impl Default for SusfsConfig {
    fn default() -> Self {
        // Defaults from BRENE v0.0.75 module/config.sh
        Self {
            paths_hiding_data_local_tmp: true,
            paths_hiding_non_standard_sdcard: false,
            paths_hiding_non_standard_sdcard_android: true,
            hide_custom_recovery: true,
            hide_ksu_directories: true,
            hide_third_party_apps: true,
            hide_suspicious_pty: false,
            hide_addon_d: false,
            hide_custom_rom_paths: false,
            hide_custom_rom_paths_2: false,
            hide_framework_res_apk: false,
            hide_lineage_strings: false,
            hide_sus_mnts_for_non_su_procs: true,
            enable_avc_log_spoofing: true,
            su_compat: true,
            kernel_umount: true,
            selinux_hide: true,
            shrink_buffer_size: true,
            fix_data_local_tmp_inconsistencies: true,
            spoof_uname: true,
            custom_spoof_uname: false,
            custom_uname_kernel_release: "default".into(),
            custom_uname_kernel_version: "default".into(),
            spoof_system_properties: true,
            spoof_os_security_patch_level_property: true,
            spoof_vendor_security_patch_level_property: true,
            spoof_cmdline_or_bootconfig: false,
            spoof_hosts: false,
            spoof_libstagefright: false,
            spoof_verified_boot_hash: String::new(),
            developer_options: false,
            usb_debugging: false,
            wireless_debugging: false,
            saturation: false,
            show_refresh_rate: false,
            disable_child_process_restrictions: true,
            pif_props: false,
            rom_props: false,
            brene_logs: false,
            enable_log: false,
        }
    }
}

impl SusfsConfig {
    pub fn load(ctx: &Context) -> anyhow::Result<Self> {
        ensure_data_dir(ctx)?;
        let path = paths::config_path(ctx);
        if path.is_file() {
            let text = std::fs::read_to_string(&path)?;
            let cfg: SusfsConfig = toml::from_str(&text)
                .map_err(|e| fail("config_parse", format!("config.toml: {e}")))?;
            return Ok(cfg);
        }
        // One-time migration from legacy config.sh if present.
        let legacy = paths::legacy_config_path(ctx);
        if legacy.is_file() {
            let cfg = migrate_from_shell(&legacy)?;
            cfg.save(ctx)?;
            return Ok(cfg);
        }
        let cfg = Self::default();
        cfg.save(ctx)?;
        Ok(cfg)
    }

    pub fn save(&self, ctx: &Context) -> anyhow::Result<()> {
        ensure_data_dir(ctx)?;
        let path = paths::config_path(ctx);
        let text = toml::to_string_pretty(self)
            .map_err(|e| fail("config_serialize", e.to_string()))?;
        std::fs::write(&path, text)?;
        Ok(())
    }
}

/// Best-effort migration from BRENE `config.sh` key=value lines.
fn migrate_from_shell(path: &Path) -> anyhow::Result<SusfsConfig> {
    let text = std::fs::read_to_string(path)?;
    let mut cfg = SusfsConfig::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().strip_prefix("config_").unwrap_or(key.trim());
        let value = value.trim().trim_matches('\'').trim_matches('"');
        let b = value == "1" || value.eq_ignore_ascii_case("true");
        match key {
            "paths_hiding__data_local_tmp" => cfg.paths_hiding_data_local_tmp = b,
            "paths_hiding__non_standard_sdcard" => cfg.paths_hiding_non_standard_sdcard = b,
            "paths_hiding__non_standard_sdcard_android" => {
                cfg.paths_hiding_non_standard_sdcard_android = b
            }
            "hide_custom_recovery" => cfg.hide_custom_recovery = b,
            "hide_ksu_directories" => cfg.hide_ksu_directories = b,
            "hide_third_party_apps" => cfg.hide_third_party_apps = b,
            "hide_suspicious_pty" => cfg.hide_suspicious_pty = b,
            "hide_addon_d" => cfg.hide_addon_d = b,
            "hide_custom_rom_paths" => cfg.hide_custom_rom_paths = b,
            "hide_custom_rom_paths_2" => cfg.hide_custom_rom_paths_2 = b,
            "hide_framework_res_apk" => cfg.hide_framework_res_apk = b,
            "hide_lineage_strings" => cfg.hide_lineage_strings = b,
            "hide_sus_mnts_for_non_su_procs" => cfg.hide_sus_mnts_for_non_su_procs = b,
            "enable_avc_log_spoofing" => cfg.enable_avc_log_spoofing = b,
            "su_compat" => cfg.su_compat = b,
            "kernel_umount" => cfg.kernel_umount = b,
            "selinux_hide" => cfg.selinux_hide = b,
            "shrink_buffer_size" => cfg.shrink_buffer_size = b,
            "fix_data_local_tmp_inconsistencies" => cfg.fix_data_local_tmp_inconsistencies = b,
            "spoof_uname" => cfg.spoof_uname = b,
            "custom_spoof_uname" => cfg.custom_spoof_uname = b,
            "custom_uname_kernel_release" => cfg.custom_uname_kernel_release = value.into(),
            "custom_uname_kernel_version" => cfg.custom_uname_kernel_version = value.into(),
            "spoof_system_properties" => cfg.spoof_system_properties = b,
            "spoof_os_security_patch_level_property" => {
                cfg.spoof_os_security_patch_level_property = b
            }
            "spoof_vendor_security_patch_level_property" => {
                cfg.spoof_vendor_security_patch_level_property = b
            }
            "spoof_cmdline_or_bootconfig" => cfg.spoof_cmdline_or_bootconfig = b,
            "spoof_hosts" => cfg.spoof_hosts = b,
            "spoof_libstagefright" => cfg.spoof_libstagefright = b,
            "spoof_verified_boot_hash" => cfg.spoof_verified_boot_hash = value.into(),
            "developer_options" => cfg.developer_options = b,
            "usb_debugging" => cfg.usb_debugging = b,
            "wireless_debugging" => cfg.wireless_debugging = b,
            "saturation" => cfg.saturation = b,
            "show_refresh_rate" => cfg.show_refresh_rate = b,
            "disable_child_process_restrictions" => cfg.disable_child_process_restrictions = b,
            "pif_props" => cfg.pif_props = b,
            "rom_props" => cfg.rom_props = b,
            "brene_logs" => cfg.brene_logs = b,
            "enable_log" => cfg.enable_log = b,
            _ => {}
        }
    }
    Ok(cfg)
}
