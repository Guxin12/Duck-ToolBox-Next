import { useFeatureI18n } from "@/core/i18n"
import type messages from "./locales/en.json"
export const useSuSFSI18n = () => useSuSFSI18n<typeof messages>("susfs")
