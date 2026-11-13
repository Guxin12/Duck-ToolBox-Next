//! Apply config the same way BRENE's post-fs-data / boot-completed scripts do.
//! Stage selects which subset runs (early vs after storage is ready).

use serde::Serialize;

use crate::config::SusfsConfig;
use crate::custom::CustomLists;
use crate::error::fail;
use crate::susfs::{self, SusfsTool};
use duck_core::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyStage {
    /// post-fs-data style: susfs paths, avc spoof, uname, early hides.
    Early,
    /// boot-completed style: props, settings, storage-dependent work.
    Late,
    /// Both stages in order.
    All,
}

#[derive(Debug, Serialize)]
pub struct ApplyReport {
    pub stage: String,
    pub steps: Vec<String>,
    pub errors: Vec<String>,
}

pub fn apply(ctx: &Context, stage: ApplyStage) -> anyhow::Result<ApplyReport> {
    let cfg = SusfsConfig::load(ctx)?;
    let custom = CustomLists::load(ctx)?;
    let tool = SusfsTool::from_env();
    if !tool.available() {
        return Err(fail(
            "susfs_missing",
            "susfs binary not found at /data/adb/ksu/bin/susfs",
        ));
    }

    let mut steps = Vec::new();
    let mut errors = Vec::new();

    let run_early = matches!(stage, ApplyStage::Early | ApplyStage::All);
    let run_late = matches!(stage, ApplyStage::Late | ApplyStage::All);

    if run_early {
        apply_early(&tool, &cfg, &custom, &mut steps, &mut errors);
    }
    if run_late {
        apply_late(&tool, &cfg, &custom, &mut steps, &mut errors);
    }

    Ok(ApplyReport {
        stage: match stage {
            ApplyStage::Early => "early",
            ApplyStage::Late => "late",
            ApplyStage::All => "all",
        }
        .into(),
        steps,
        errors,
    })
}

fn note(steps: &mut Vec<String>, msg: impl Into<String>) {
    steps.push(msg.into());
}

fn catch(errors: &mut Vec<String>, steps: &mut Vec<String>, label: &str, r: anyhow::Result<()>) {
    match r {
        Ok(()) => note(steps, label),
        Err(e) => errors.push(format!("{label}: {e}")),
    }
}

fn apply_early(
    tool: &SusfsTool,
    cfg: &SusfsConfig,
    custom: &CustomLists,
    steps: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    // Built-in path hiding (same intent as BRENE post-fs-data).
    if cfg.paths_hiding_data_local_tmp {
        catch(
            errors,
            steps,
            "add_sus_path /data/local/tmp",
            tool.add_sus_path("/data/local/tmp"),
        );
    }
    if cfg.paths_hiding_non_standard_sdcard_android {
        for p in [
            "/sdcard/Android/data",
            "/sdcard/Android/media",
            "/sdcard/Android/obb",
            "/storage/emulated/0/Android/data",
            "/storage/emulated/0/Android/media",
            "/storage/emulated/0/Android/obb",
        ] {
            let _ = tool.add_sus_path(p);
        }
        note(steps, "paths_hiding non-standard sdcard Android");
    }

    // Custom lists
    for p in &custom.sus_path {
        catch(errors, steps, &format!("sus_path {p}"), tool.add_sus_path(p));
    }
    for p in &custom.sus_path_loop {
        catch(
            errors,
            steps,
            &format!("sus_path_loop {p}"),
            tool.add_sus_path_loop(p),
        );
    }
    for p in &custom.sus_map {
        catch(errors, steps, &format!("sus_map {p}"), tool.add_sus_map(p));
    }
    for line in &custom.open_redirect {
        // format: src|dst or src|dst|flags
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 2 {
            let flags = parts.get(2).copied().unwrap_or("");
            catch(
                errors,
                steps,
                &format!("open_redirect {}", parts[0]),
                tool.add_open_redirect(parts[0], parts[1], flags),
            );
        }
    }

    if cfg.enable_avc_log_spoofing {
        catch(
            errors,
            steps,
            "enable_avc_log_spoofing",
            tool.enable_avc_log_spoofing(true),
        );
    }
    if cfg.hide_sus_mnts_for_non_su_procs {
        catch(
            errors,
            steps,
            "hide_sus_mnts_for_non_su_procs",
            tool.hide_sus_mnts_for_non_su_procs(true),
        );
    }

    // Uname spoof (simplified; full BRENE builds release string from KMI + random)
    if cfg.spoof_uname {
        let release = if cfg.custom_spoof_uname
            && cfg.custom_uname_kernel_release != "default"
        {
            cfg.custom_uname_kernel_release.as_str()
        } else {
            "default"
        };
        let version = if cfg.custom_spoof_uname
            && cfg.custom_uname_kernel_version != "default"
        {
            cfg.custom_uname_kernel_version.as_str()
        } else {
            "default"
        };
        catch(
            errors,
            steps,
            "set_uname",
            tool.set_uname(release, version),
        );
    }
}

fn apply_late(
    _tool: &SusfsTool,
    cfg: &SusfsConfig,
    _custom: &CustomLists,
    steps: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    // KSU features
    catch(
        errors,
        steps,
        "ksud su_compat",
        susfs::ksud_feature_set("su_compat", cfg.su_compat),
    );
    catch(
        errors,
        steps,
        "ksud kernel_umount",
        susfs::ksud_feature_set("kernel_umount", cfg.kernel_umount),
    );
    catch(
        errors,
        steps,
        "ksud selinux_hide",
        susfs::ksud_feature_set("selinux_hide", cfg.selinux_hide),
    );

    // Developer / debugging settings
    catch(
        errors,
        steps,
        "developer_options",
        susfs::settings_put_global(
            "development_settings_enabled",
            if cfg.developer_options { "1" } else { "0" },
        ),
    );
    catch(
        errors,
        steps,
        "usb_debugging",
        susfs::settings_put_global("adb_enabled", if cfg.usb_debugging { "1" } else { "0" }),
    );
    catch(
        errors,
        steps,
        "wireless_debugging",
        susfs::settings_put_global(
            "adb_wifi_enabled",
            if cfg.wireless_debugging { "1" } else { "0" },
        ),
    );

    if cfg.spoof_system_properties {
        // Core set from BRENE utils.sh spoof_system_properties
        for (k, v) in [
            ("ro.secure", "1"),
            ("ro.debuggable", "0"),
            ("ro.adb.secure", "1"),
            ("ro.boot.verifiedbootstate", "green"),
            ("ro.boot.flash.locked", "1"),
            ("ro.boot.veritymode", "enforcing"),
            ("ro.build.type", "user"),
            ("ro.build.tags", "release-keys"),
        ] {
            let _ = susfs::resetprop_n(k, v);
        }
        note(steps, "spoof_system_properties");
    }

    if !cfg.spoof_verified_boot_hash.is_empty() {
        catch(
            errors,
            steps,
            "spoof_verified_boot_hash",
            susfs::resetprop_n("ro.boot.vbmeta.digest", &cfg.spoof_verified_boot_hash),
        );
    }
}
