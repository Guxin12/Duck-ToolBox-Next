//! Thin wrapper around the `ksu_susfs` / `susfs` userspace binary.

use std::path::PathBuf;
use std::process::Command;

use crate::error::fail;
use crate::paths;

pub struct SusfsTool {
    bin: PathBuf,
}

impl SusfsTool {
    pub fn from_env() -> Self {
        Self {
            bin: paths::susfs_bin(),
        }
    }

    pub fn available(&self) -> bool {
        self.bin.is_file()
    }

    fn run(&self, args: &[&str]) -> anyhow::Result<String> {
        if !self.bin.is_file() {
            return Err(fail(
                "susfs_missing",
                format!("{} not found", self.bin.display()),
            ));
        }
        let out = Command::new(&self.bin)
            .args(args)
            .output()
            .map_err(|e| fail("susfs_spawn", e.to_string()))?;
        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            return Err(fail(
                "susfs_failed",
                format!("susfs {:?}: {}", args, stderr.trim()),
            ));
        }
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }

    pub fn show_version(&self) -> anyhow::Result<String> {
        self.run(&["show", "version"])
    }

    pub fn show_variant(&self) -> anyhow::Result<String> {
        self.run(&["show", "variant"])
    }

    pub fn show_enabled_features(&self) -> anyhow::Result<Vec<String>> {
        let s = self.run(&["show", "enabled_features"])?;
        Ok(s.lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect())
    }

    pub fn add_sus_path(&self, path: &str) -> anyhow::Result<()> {
        self.run(&["add_sus_path", path]).map(|_| ())
    }

    pub fn add_sus_path_loop(&self, path: &str) -> anyhow::Result<()> {
        self.run(&["add_sus_path_loop", path]).map(|_| ())
    }

    pub fn add_sus_map(&self, path: &str) -> anyhow::Result<()> {
        self.run(&["add_sus_map", path]).map(|_| ())
    }

    pub fn add_open_redirect(&self, from: &str, to: &str, flags: &str) -> anyhow::Result<()> {
        if flags.is_empty() {
            self.run(&["add_open_redirect", from, to]).map(|_| ())
        } else {
            self.run(&["add_open_redirect", from, to, flags]).map(|_| ())
        }
    }

    pub fn set_uname(&self, release: &str, version: &str) -> anyhow::Result<()> {
        self.run(&["set_uname", release, version]).map(|_| ())
    }

    pub fn enable_avc_log_spoofing(&self, on: bool) -> anyhow::Result<()> {
        self.run(&["enable_avc_log_spoofing", if on { "1" } else { "0" }])
            .map(|_| ())
    }

    pub fn hide_sus_mnts_for_non_su_procs(&self, on: bool) -> anyhow::Result<()> {
        self.run(&[
            "hide_sus_mnts_for_non_su_procs",
            if on { "1" } else { "0" },
        ])
        .map(|_| ())
    }

    pub fn set_cmdline_or_bootconfig(&self, path: &str) -> anyhow::Result<()> {
        self.run(&["set_cmdline_or_bootconfig", path]).map(|_| ())
    }
}

/// Run `ksud feature set <name> <0|1>` and optionally `ksud feature save`.
pub fn ksud_feature_set(name: &str, on: bool) -> anyhow::Result<()> {
    let bin = paths::ksud_bin();
    let status = Command::new(&bin)
        .args(["feature", "set", name, if on { "1" } else { "0" }])
        .status()
        .map_err(|e| fail("ksud_spawn", e.to_string()))?;
    if !status.success() {
        return Err(fail("ksud_failed", format!("feature set {name}")));
    }
    let status = Command::new(&bin)
        .args(["feature", "save"])
        .status()
        .map_err(|e| fail("ksud_spawn", e.to_string()))?;
    if !status.success() {
        return Err(fail("ksud_failed", "feature save"));
    }
    Ok(())
}

pub fn resetprop_n(key: &str, value: &str) -> anyhow::Result<()> {
    let status = Command::new("resetprop")
        .args(["-n", key, value])
        .status()
        .map_err(|e| fail("resetprop_spawn", e.to_string()))?;
    if !status.success() {
        return Err(fail(
            "resetprop_failed",
            format!("resetprop -n {key} {value}"),
        ));
    }
    Ok(())
}

pub fn settings_put_global(key: &str, value: &str) -> anyhow::Result<()> {
    let status = Command::new("settings")
        .args(["put", "global", key, value])
        .status()
        .map_err(|e| fail("settings_spawn", e.to_string()))?;
    if !status.success() {
        return Err(fail(
            "settings_failed",
            format!("settings put global {key} {value}"),
        ));
    }
    Ok(())
}
