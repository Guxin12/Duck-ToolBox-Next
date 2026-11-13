import { Shield } from "@lucide/vue"
import { defineFeature } from "@/core/features"
import { localeFiles } from "@/core/i18n"

export default defineFeature({
  id: "susfs",
  namespace: "susfs",
  icon: Shield,
  order: 16,
  placement: "tool",
  contract: 1,
  routes: [
    {
      path: "",
      name: "susfs",
      component: () => import("./pages/SusfsPage.vue"),
      meta: { titleKey: "susfs.meta.title", backTo: "/" },
    },
  ],
  messages: localeFiles(import.meta.glob("./locales/*.json")),
})
