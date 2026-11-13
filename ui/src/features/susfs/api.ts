import { duckd } from "@/core/duckd"

export type SusfsConfig = {
  paths_hiding_data_local_tmp: boolean
  paths_hiding_non_standard_sdcard: boolean
  paths_hiding_non_standard_sdcard_android: boolean
  hide_custom_recovery: boolean
  hide_ksu_directories: boolean
  hide_third_party_apps: boolean
  hide_suspicious_pty: boolean
  hide_addon_d: boolean
  hide_custom_rom_paths: boolean
  hide_custom_rom_paths_2: boolean
  hide_framework_res_apk: boolean
  hide_lineage_strings: boolean
  hide_sus_mnts_for_non_su_procs: boolean
  enable_avc_log_spoofing: boolean
  su_compat: boolean
  kernel_umount: boolean
  selinux_hide: boolean
  shrink_buffer_size: boolean
  fix_data_local_tmp_inconsistencies: boolean
  spoof_uname: boolean
  custom_spoof_uname: boolean
  custom_uname_kernel_release: string
  custom_uname_kernel_version: string
  spoof_system_properties: boolean
  spoof_os_security_patch_level_property: boolean
  spoof_vendor_security_patch_level_property: boolean
  spoof_cmdline_or_bootconfig: boolean
  spoof_hosts: boolean
  spoof_libstagefright: boolean
  spoof_verified_boot_hash: string
  developer_options: boolean
  usb_debugging: boolean
  wireless_debugging: boolean
  saturation: boolean
  show_refresh_rate: boolean
  disable_child_process_restrictions: boolean
  pif_props: boolean
  rom_props: boolean
  brene_logs: boolean
  enable_log: boolean
}

export type CustomLists = {
  sus_path: string[]
  sus_path_loop: string[]
  sus_map: string[]
  open_redirect: string[]
  kernel_umount: string[]
}

export type SusfsStatus = {
  susfs_available: boolean
  susfs_version: string | null
  susfs_variant: string | null
  enabled_features: string[]
  enabled_features_count: number
  susfs_total_features: number
  config: SusfsConfig
  custom: CustomLists
  device_model: string | null
  kernel_release: string | null
  android_release: string | null
  android_sdk: string | null
  custom_rom_hint: boolean | null
}

export type ApplyReport = {
  stage: string
  steps: string[]
  errors: string[]
}

export const susfsApi = {
  status: () => duckd<SusfsStatus>(["susfs", "status"]),
  getConfig: () => duckd<SusfsConfig>(["susfs", "get-config"]),
  setConfig: (config: SusfsConfig) =>
    duckd<SusfsConfig>(["susfs", "set-config"], { input: config }),
  getCustom: () => duckd<CustomLists>(["susfs", "get-custom"]),
  setCustom: (kind: string, lines: string[]) =>
    duckd<CustomLists>(["susfs", "set-custom"], { input: { kind, lines } }),
  apply: (stage: "early" | "late" | "all" = "all") =>
    duckd<ApplyReport>(["susfs", "apply", "--stage", stage]),
}
