import { computed, nextTick, ref, watch } from "vue"
import { Cpu, FileText, Info, ShieldCheck } from "@lucide/vue"
import { useI18n } from "vue-i18n"
import { RouterLink } from "vue-router"

import SectionCard from "@/core/components/SectionCard.vue"
import { useFeatureI18n } from "@/core/i18n/feature"
import type messages from "../locales/en.json"

const props = withDefaults(defineProps<{ section?: string }>(), { section: "status" })
const { t } = useFeatureI18n<typeof messages>("susfs")
const global = useI18n()
const tabNav = ref<HTMLElement>()
const tabs = ["status", "kernel", "diagnostics", "about"] as const
const activeSection = computed(() =>
  tabs.includes(props.section as (typeof tabs)[number])
    ? (props.section as (typeof tabs)[number])
    : "status",
)

watch(
  activeSection,
  async () => {
    await nextTick()
    tabNav.value
      ?.querySelector('[aria-current="page"]')
      ?.scrollIntoView({ block: "nearest", inline: "nearest" })
  },
  { immediate: true },
)
</script>

<template>
  <div class="mx-auto flex min-h-full w-full max-w-4xl flex-col">
    <nav
      ref="tabNav"
      :aria-label="t('meta.title')"
      class="mx-4 mt-4 flex min-w-0 gap-1 overflow-x-auto rounded-2xl bg-muted/70 p-1.5"
    >
      <RouterLink
        v-for="tab in tabs"
        :key="tab"
        replace
        :to="tab === 'status' ? '/susfs' : `/susfs/${tab}`"
        class="state-layer focus-visible:ring-ring/50 relative inline-flex min-h-12 shrink-0 items-center justify-center rounded-xl px-4 text-sm font-medium whitespace-nowrap outline-none transition-[color,background-color,box-shadow] focus-visible:ring-2"
        :class="
          activeSection === tab
            ? 'bg-card text-foreground shadow-sm ring-1 ring-border/60'
            : 'text-muted-foreground hover:text-foreground'
        "
        :aria-current="activeSection === tab ? 'page' : undefined"
        >{{ t(`nav.${tab}`) }}</RouterLink
      >
    </nav>

    <main class="flex min-w-0 flex-1 flex-col gap-5 px-4 pt-5 pb-6">
      <section
        class="flex items-start gap-4 rounded-2xl border bg-card p-4"
        aria-live="polite"
      >
        <div class="bg-primary/10 text-primary flex size-11 shrink-0 items-center justify-center rounded-xl">
          <ShieldCheck v-if="activeSection === 'status'" class="size-5" aria-hidden="true" />
          <Cpu v-else-if="activeSection === 'kernel'" class="size-5" aria-hidden="true" />
          <FileText v-else-if="activeSection === 'diagnostics'" class="size-5" aria-hidden="true" />
          <Info v-else class="size-5" aria-hidden="true" />
        </div>
        <div class="min-w-0">
          <p class="text-muted-foreground text-sm">{{ t("prototype.label") }}</p>
          <p class="mt-1 text-lg leading-7 font-semibold">{{ t("prototype.notConnected") }}</p>
          <p class="text-muted-foreground mt-1 text-sm leading-6">
            {{ t("prototype.description") }}
          </p>
        </div>
      </section>

      <template v-if="activeSection === 'status'">
        <SectionCard :title="t('status.title')" :description="t('status.description')">
          <dl class="grid gap-3 sm:grid-cols-2">
            <div v-for="item in ['binary', 'variant', 'version', 'kernel']" :key="item" class="rounded-xl bg-muted/70 p-4">
              <dt class="text-muted-foreground text-sm">{{ t(`status.fields.${item}`) }}</dt>
              <dd class="mt-1 font-medium">{{ t("prototype.pending") }}</dd>
            </div>
          </dl>
        </SectionCard>
        <SectionCard :title="t('status.scopeTitle')">
          <p class="text-muted-foreground text-sm leading-6">{{ t("status.scopeDescription") }}</p>
        </SectionCard>
      </template>

      <template v-else-if="activeSection === 'kernel'">
        <SectionCard :title="t('kernel.title')" :description="t('kernel.description')">
          <dl class="divide-y">
            <div v-for="item in ['release', 'variant', 'support']" :key="item" class="flex flex-wrap justify-between gap-2 py-3 first:pt-0 last:pb-0">
              <dt class="text-muted-foreground">{{ t(`kernel.fields.${item}`) }}</dt>
              <dd class="font-medium">{{ t("prototype.pending") }}</dd>
            </div>
          </dl>
        </SectionCard>
      </template>

      <template v-else-if="activeSection === 'diagnostics'">
        <SectionCard :title="t('diagnostics.title')" :description="t('diagnostics.description')">
          <div class="rounded-xl bg-muted/70 p-4">
            <p class="font-medium">{{ t("diagnostics.logs") }}</p>
            <p class="text-muted-foreground mt-2 text-sm leading-6">{{ t("diagnostics.empty") }}</p>
          </div>
        </SectionCard>
      </template>

      <template v-else>
        <SectionCard :title="t('about.title')">
          <p class="text-muted-foreground text-sm leading-6">{{ t("about.description") }}</p>
          <p class="text-muted-foreground mt-3 text-sm leading-6">
            {{ global.t("core.home.noHost") }}
          </p>
        </SectionCard>
      </template>
    </main>
  </div>
</template>
