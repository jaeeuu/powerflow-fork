import type { ChargingHistory } from '@/bindings'
import { commands } from '@/bindings'

export function useAsyncData<T>(promiseFn: () => Promise<{ status: 'ok', data: T } | { status: 'error', error: string }>) {
  const data = ref<T | null>(null)
  const isLoading = ref(true)
  const err = ref('')

  const load = async () => {
    try {
      const r = await promiseFn()
      if (r.status === 'ok') {
        data.value = r.data
      }
      else {
        err.value = r.error
      }
    }
    catch (error) {
      err.value = String(error)
    }
    finally {
      isLoading.value = false
    }
  }

  const update = () => {
    isLoading.value = true
    data.value = null
    err.value = ''
    load()
  }

  load()

  return {
    data,
    isLoading,
    err,
    update,
  }
}

const selectedItem = ref(null as ChargingHistory | null)
const history = useAsyncData<ChargingHistory[]>(() => commands.getAllChargingHistory())

export function useHistory() {
  return {
    selectedItem,
    history,
  }
}
