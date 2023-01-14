import router from '@/router'

export default {
  install: () => {
    router.beforeEach(async (to, from, next) => {
        if (to.path == '/') {
          next({path:'/home'})
        } else {
          next()
        }
    })
  },
}
