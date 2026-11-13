import { ShieldCheck } from "@lucide/vue"
import { defineFeature } from "@/core/features"
import { localeFiles } from "@/core/i18n"

export default defineFeature({
  id: "susfs",
  namespace: "susfs",
  backend: null,
  icon: ShieldCheck,
  order: 16,
  routes: ["", "kernel", "diagnostics", "about"].map((section) => ({
    path: section,
    component: () => import("./pages/SusfsPage.vue"),
    props: { section: section || "status" },
    meta: { titleKey: `susfs.nav.${section || "status"}`, backTo: "/" },
  })),
  messages: localeFiles(import.meta.glob("./locales/*.json")),
})
