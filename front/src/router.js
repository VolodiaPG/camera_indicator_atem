import { writable } from 'svelte/store'
import router from 'page'

export const page = writable({
  component: null,
  props: {}
})

router('/', () =>
  import(/* webpackChunkName: "index" */ './pages/Main.svelte').then(module =>
    page.set({ component: module.default })
  )
)

router('/messages', () =>
  import(/* webpackChunkName: "messages" */ './pages/Messages.svelte').then(
    module => page.set({ component: module.default })
  )
)


export default router