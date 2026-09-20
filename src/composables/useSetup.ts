import { useI18n } from 'vue-i18n'
import { commands, events } from '@/bindings'
import { initializePreference } from '@/stores/preference'

const SUPPORTED_LOCALES = ['en', 'zh-CN'] as const

export function resolveLocale(preferred: string | undefined, fallback: string) {
  if (!preferred)
    return fallback
  if ((SUPPORTED_LOCALES as readonly string[]).includes(preferred))
    return preferred
  if (preferred.startsWith('zh'))
    return 'zh-CN'
  if (preferred.startsWith('en'))
    return 'en'
  return fallback
}

export function useSetup() {
  const i18n = useI18n()
  const preferedLang = usePreferredLanguages()
  const preference = usePreference()
  const preferDark = usePreferredDark()

  i18n.locale.value = resolveLocale(preferedLang.value[0], 'en')

  const applyTheme = async (theme = preference.theme) => {
    document.documentElement.classList.toggle(
      'dark',
      theme === 'system' ? preferDark.value : theme === 'dark',
    )
    try {
      await commands.switchTheme(theme)
    }
    catch (err) {
      console.warn('[theme] switchTheme failed', err)
    }
  }

  initializePreference(preference).then(async () => {
    applyTheme()
    i18n.locale.value = resolveLocale(preference.language, i18n.locale.value)
    await events.preferenceEvent.emit({ updateInterval: preference.updateInterval })
    await events.preferenceEvent.emit({ statusBarItem: preference.statusBarItem })
    await events.preferenceEvent.emit({ statusBarShowCharging: preference.statusBarShowCharging })
  }).catch(error => console.error('[preference]', error))

  watch([preferDark, () => preference.theme], () => applyTheme())

  events.preferenceEvent.listen(({ payload }) => {
    if ('theme' in payload) {
      applyTheme(payload.theme)
    }
    if ('language' in payload) {
      i18n.locale.value = resolveLocale(payload.language, i18n.locale.value)
    }
  })

  // notify rust to get a instant update
  onMounted(() => {
    events.windowLoadedEvent.emit()
  })
}
