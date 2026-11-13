<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue"
import { useI18n } from "vue-i18n"
import { useQuery, useMutation, useQueryClient } from "@tanstack/vue-query"

import {
  susfsApi,
  type SusfsConfig,
  type SusfsStatus,
  type CustomLists,
} from "../api"

const { t } = useI18n({ useScope: "global" })
const qc = useQueryClient()

const TABS = [
  "home",
  "android",
  "hiding",
  "spoofing",
  "susfs",
  "ksu",
  "rom",
  "advanced",
  "info",
] as const
type TabId = (typeof TABS)[number]
const tab = ref<TabId>("home")

const statusQuery = useQuery({
  queryKey: ["susfs", "status"],
  queryFn: () => susfsApi.status(),
})

const config = ref<SusfsConfig | null>(null)
const custom = ref<CustomLists | null>(null)
const message = ref("")
const messageError = ref(false)

watch(
  () => statusQuery.data.value,
  (s) => {
    if (s) {
      config.value = { ...s.config }
      custom.value = {
        sus_path: [...s.custom.sus_path],
        sus_path_loop: [...s.custom.sus_path_loop],
        sus_map: [...s.custom.sus_map],
        open_redirect: [...s.custom.open_redirect],
        kernel_umount: [...s.custom.kernel_umount],
      }
    }
  },
  { immediate: true },
)

const saveConfig = useMutation({
  mutationFn: (c: SusfsConfig) => susfsApi.setConfig(c),
  onSuccess: async () => {
    messageError.value = false
    message.value = t("susfs.common.save")
    await qc.invalidateQueries({ queryKey: ["susfs", "status"] })
  },
  onError: (e: Error) => {
    messageError.value = true
    message.value = e.message
  },
})

const applyMut = useMutation({
  mutationFn: () => susfsApi.apply("all"),
  onSuccess: (r) => {
    messageError.value = r.errors.length > 0
    message.value =
      r.errors.length > 0
        ? `${t("susfs.home.applyFail")}: ${r.errors[0]}`
        : t("susfs.home.applyOk")
  },
  onError: (e: Error) => {
    messageError.value = true
    message.value = e.message
  },
})

function toggle<K extends keyof SusfsConfig>(key: K) {
  if (!config.value) return
  const cur = config.value[key]
  if (typeof cur === "boolean") {
    ;(config.value as Record<string, unknown>)[key as string] = !cur
    saveConfig.mutate({ ...config.value })
  }
}

function setString<K extends keyof SusfsConfig>(key: K, value: string) {
  if (!config.value) return
  ;(config.value as Record<string, unknown>)[key as string] = value
  saveConfig.mutate({ ...config.value })
}

async function saveCustomList(
  kind: "sus_path" | "sus_path_loop" | "sus_map" | "open_redirect" | "kernel_umount",
  text: string,
) {
  const lines = text
    .split("\n")
    .map((l) => l.trim())
    .filter(Boolean)
  try {
    const next = await susfsApi.setCustom(kind, lines)
    custom.value = next
    messageError.value = false
    message.value = t("susfs.susfsTab.save")
  } catch (e) {
    messageError.value = true
    message.value = e instanceof Error ? e.message : String(e)
  }
}

const customText = computed(() => ({
  sus_path: (custom.value?.sus_path ?? []).join("\n"),
  sus_path_loop: (custom.value?.sus_path_loop ?? []).join("\n"),
  sus_map: (custom.value?.sus_map ?? []).join("\n"),
  open_redirect: (custom.value?.open_redirect ?? []).join("\n"),
  kernel_umount: (custom.value?.kernel_umount ?? []).join("\n"),
}))

const status = computed(() => statusQuery.data.value as SusfsStatus | undefined)
const loading = computed(() => statusQuery.isLoading.value)
const loadError = computed(() => statusQuery.error.value as Error | null)

onMounted(() => {
  void statusQuery.refetch()
})

function yesNo(v: boolean | null | undefined) {
  if (v === true) return t("susfs.common.yes")
  if (v === false) return t("susfs.common.no")
  return "—"
}
</script>

<template>
  <div class="flex h-full min-h-0 flex-col gap-3 p-3">
    <!-- Tab bar: same 9 tabs as BRENE -->
    <nav class="flex flex-wrap gap-1 border-b pb-2">
      <button
        v-for="id in TABS"
        :key="id"
        type="button"
        class="rounded-full px-3 py-1 text-sm"
        :class="
          tab === id ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'
        "
        @click="tab = id"
      >
        {{ t(`susfs.tabs.${id}`) }}
      </button>
    </nav>

    <p
      v-if="message"
      class="text-sm"
      :class="messageError ? 'text-destructive' : 'text-muted-foreground'"
    >
      {{ message }}
    </p>
    <p v-if="loading" class="text-sm text-muted-foreground">{{ t("susfs.common.loading") }}</p>
    <p v-if="loadError" class="text-sm text-destructive">{{ loadError.message }}</p>

    <div v-if="config && status" class="min-h-0 flex-1 overflow-y-auto">
      <!-- HOME -->
      <section v-show="tab === 'home'" class="space-y-3">
        <h2 class="text-base font-medium">{{ t("susfs.home.general") }}</h2>
        <div class="grid gap-2 text-sm">
          <div class="flex justify-between rounded-lg border p-3">
            <span>{{ t("susfs.home.deviceModel") }}</span>
            <span class="text-muted-foreground">{{ status.device_model ?? "—" }}</span>
          </div>
          <div class="flex justify-between rounded-lg border p-3">
            <span>{{ t("susfs.home.kernelVersion") }}</span>
            <span class="text-muted-foreground">{{ status.kernel_release ?? "—" }}</span>
          </div>
          <div class="flex justify-between rounded-lg border p-3">
            <span>{{ t("susfs.home.androidVersion") }}</span>
            <span class="text-muted-foreground">
              {{ status.android_release ?? "—" }}
              <template v-if="status.android_sdk">(API {{ status.android_sdk }})</template>
            </span>
          </div>
          <div class="flex justify-between rounded-lg border p-3">
            <span>{{ t("susfs.home.customRom") }}</span>
            <span class="text-muted-foreground">{{ yesNo(status.custom_rom_hint) }}</span>
          </div>
          <div class="flex justify-between rounded-lg border p-3">
            <span>{{ t("susfs.home.susfsVersion") }}</span>
            <span class="text-muted-foreground">{{ status.susfs_version ?? "—" }}</span>
          </div>
          <div class="flex justify-between rounded-lg border p-3">
            <span>{{ t("susfs.home.susfsVariant") }}</span>
            <span class="text-muted-foreground">{{ status.susfs_variant ?? "—" }}</span>
          </div>
          <div class="flex justify-between rounded-lg border p-3">
            <span>{{ t("susfs.home.features") }}</span>
            <span class="text-muted-foreground">
              {{ status.enabled_features_count }}/{{ status.susfs_total_features }}
            </span>
          </div>
        </div>
        <button
          type="button"
          class="rounded-lg bg-primary px-4 py-2 text-sm text-primary-foreground"
          :disabled="applyMut.isPending.value"
          @click="applyMut.mutate()"
        >
          {{ t("susfs.home.apply") }}
        </button>
      </section>

      <!-- ANDROID -->
      <section v-show="tab === 'android'" class="space-y-2">
        <h2 class="text-base font-medium">{{ t("susfs.android.title") }}</h2>
        <label
          v-for="item in [
            ['developer_options', 'developerOptions'],
            ['usb_debugging', 'usbDebugging'],
            ['wireless_debugging', 'wirelessDebugging'],
            ['show_refresh_rate', 'showRefreshRate'],
            ['saturation', 'saturation'],
            ['disable_child_process_restrictions', 'disableChildProcessRestrictions'],
          ] as const"
          :key="item[0]"
          class="flex items-center justify-between rounded-lg border p-3 text-sm"
        >
          <span>{{ t(`susfs.android.${item[1]}`) }}</span>
          <input type="checkbox" :checked="!!config[item[0]]" @change="toggle(item[0])" />
        </label>
      </section>

      <!-- HIDING -->
      <section v-show="tab === 'hiding'" class="space-y-2">
        <h2 class="text-base font-medium">{{ t("susfs.hiding.title") }}</h2>
        <label
          v-for="item in [
            ['paths_hiding_data_local_tmp', 'dataLocalTmp'],
            ['paths_hiding_non_standard_sdcard', 'nonStandardSdcard'],
            ['paths_hiding_non_standard_sdcard_android', 'nonStandardSdcardAndroid'],
            ['hide_custom_recovery', 'customRecovery'],
            ['hide_ksu_directories', 'ksuDirectories'],
            ['hide_third_party_apps', 'thirdPartyApps'],
            ['hide_suspicious_pty', 'suspiciousPty'],
            ['hide_addon_d', 'addonD'],
            ['hide_custom_rom_paths', 'customRomPaths'],
            ['hide_custom_rom_paths_2', 'customRomPaths2'],
            ['hide_framework_res_apk', 'frameworkRes'],
            ['hide_lineage_strings', 'lineageStrings'],
            ['hide_sus_mnts_for_non_su_procs', 'susMntsNonSu'],
          ] as const"
          :key="item[0]"
          class="flex items-center justify-between rounded-lg border p-3 text-sm"
        >
          <span>{{ t(`susfs.hiding.${item[1]}`) }}</span>
          <input type="checkbox" :checked="!!config[item[0]]" @change="toggle(item[0])" />
        </label>
      </section>

      <!-- SPOOFING -->
      <section v-show="tab === 'spoofing'" class="space-y-2">
        <h2 class="text-base font-medium">{{ t("susfs.spoofing.title") }}</h2>
        <label
          v-for="item in [
            ['spoof_system_properties', 'systemProperties'],
            ['spoof_os_security_patch_level_property', 'osPatch'],
            ['spoof_vendor_security_patch_level_property', 'vendorPatch'],
            ['spoof_uname', 'uname'],
            ['custom_spoof_uname', 'customUname'],
            ['spoof_cmdline_or_bootconfig', 'cmdline'],
            ['spoof_hosts', 'hosts'],
            ['spoof_libstagefright', 'libstagefright'],
          ] as const"
          :key="item[0]"
          class="flex items-center justify-between rounded-lg border p-3 text-sm"
        >
          <span>{{ t(`susfs.spoofing.${item[1]}`) }}</span>
          <input type="checkbox" :checked="!!config[item[0]]" @change="toggle(item[0])" />
        </label>
        <div class="space-y-1 rounded-lg border p-3 text-sm">
          <label class="block">{{ t("susfs.spoofing.unameRelease") }}</label>
          <input
            class="w-full rounded border bg-background px-2 py-1"
            :value="config.custom_uname_kernel_release"
            @change="
              setString('custom_uname_kernel_release', ($event.target as HTMLInputElement).value)
            "
          />
          <label class="mt-2 block">{{ t("susfs.spoofing.unameVersion") }}</label>
          <input
            class="w-full rounded border bg-background px-2 py-1"
            :value="config.custom_uname_kernel_version"
            @change="
              setString('custom_uname_kernel_version', ($event.target as HTMLInputElement).value)
            "
          />
          <label class="mt-2 block">{{ t("susfs.spoofing.vbmetaHash") }}</label>
          <input
            class="w-full rounded border bg-background px-2 py-1 font-mono text-xs"
            :value="config.spoof_verified_boot_hash"
            @change="
              setString('spoof_verified_boot_hash', ($event.target as HTMLInputElement).value)
            "
          />
        </div>
      </section>

      <!-- SUSFS custom lists -->
      <section v-show="tab === 'susfs'" class="space-y-3">
        <h2 class="text-base font-medium">{{ t("susfs.susfsTab.title") }}</h2>
        <div
          v-for="kind in [
            'sus_path',
            'sus_path_loop',
            'sus_map',
            'open_redirect',
            'kernel_umount',
          ] as const"
          :key="kind"
          class="space-y-1"
        >
          <label class="text-sm font-medium">{{
            t(
              kind === "sus_path"
                ? "susfs.susfsTab.susPath"
                : kind === "sus_path_loop"
                  ? "susfs.susfsTab.susPathLoop"
                  : kind === "sus_map"
                    ? "susfs.susfsTab.susMap"
                    : kind === "open_redirect"
                      ? "susfs.susfsTab.openRedirect"
                      : "susfs.susfsTab.kernelUmount",
            )
          }}</label>
          <textarea
            class="min-h-24 w-full rounded border bg-background p-2 font-mono text-xs"
            :value="customText[kind]"
            @change="saveCustomList(kind, ($event.target as HTMLTextAreaElement).value)"
          />
        </div>
      </section>

      <!-- KSU -->
      <section v-show="tab === 'ksu'" class="space-y-2">
        <h2 class="text-base font-medium">{{ t("susfs.ksu.title") }}</h2>
        <label
          v-for="item in [
            ['su_compat', 'suCompat'],
            ['kernel_umount', 'kernelUmount'],
            ['selinux_hide', 'selinuxHide'],
          ] as const"
          :key="item[0]"
          class="flex items-center justify-between rounded-lg border p-3 text-sm"
        >
          <span>{{ t(`susfs.ksu.${item[1]}`) }}</span>
          <input type="checkbox" :checked="!!config[item[0]]" @change="toggle(item[0])" />
        </label>
      </section>

      <!-- ROM -->
      <section v-show="tab === 'rom'" class="space-y-2">
        <h2 class="text-base font-medium">{{ t("susfs.rom.title") }}</h2>
        <label
          v-for="item in [
            ['pif_props', 'pifProps'],
            ['rom_props', 'romProps'],
          ] as const"
          :key="item[0]"
          class="flex items-center justify-between rounded-lg border p-3 text-sm"
        >
          <span>{{ t(`susfs.rom.${item[1]}`) }}</span>
          <input type="checkbox" :checked="!!config[item[0]]" @change="toggle(item[0])" />
        </label>
        <div class="rounded-lg border p-3 text-sm text-muted-foreground">
          {{ t("susfs.rom.lineageHint") }}: {{ yesNo(status.custom_rom_hint) }}
        </div>
      </section>

      <!-- ADVANCED -->
      <section v-show="tab === 'advanced'" class="space-y-2">
        <h2 class="text-base font-medium">{{ t("susfs.advanced.title") }}</h2>
        <label
          v-for="item in [
            ['enable_avc_log_spoofing', 'avcLogSpoofing'],
            ['shrink_buffer_size', 'shrinkBuffer'],
            ['fix_data_local_tmp_inconsistencies', 'fixDataLocalTmp'],
            ['brene_logs', 'breneLogs'],
            ['enable_log', 'enableLog'],
          ] as const"
          :key="item[0]"
          class="flex items-center justify-between rounded-lg border p-3 text-sm"
        >
          <span>{{ t(`susfs.advanced.${item[1]}`) }}</span>
          <input type="checkbox" :checked="!!config[item[0]]" @change="toggle(item[0])" />
        </label>
      </section>

      <!-- INFO -->
      <section v-show="tab === 'info'" class="space-y-2">
        <h2 class="text-base font-medium">{{ t("susfs.info.title") }}</h2>
        <p v-if="!status.susfs_available" class="text-sm text-destructive">
          {{ t("susfs.info.unavailable") }}
        </p>
        <div class="rounded-lg border p-3 text-sm">
          <div class="mb-2 font-medium">{{ t("susfs.info.enabledFeatures") }}</div>
          <ul class="list-inside list-disc text-muted-foreground">
            <li v-for="f in status.enabled_features" :key="f">{{ f }}</li>
            <li v-if="!status.enabled_features.length">—</li>
          </ul>
        </div>
      </section>
    </div>
  </div>
</template>
