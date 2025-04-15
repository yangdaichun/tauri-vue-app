import { createApp } from 'vue'
import App from './App.vue'
import router from './router/index'
import plugins from './plugins/index'
import tray_init from "./tray.js"

tray_init()
createApp(App).use(router).use(plugins).mount('#app')
