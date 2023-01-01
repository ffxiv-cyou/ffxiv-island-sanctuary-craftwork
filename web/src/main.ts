import { createApp } from 'vue'
import VueGtag from "vue-gtag";
import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(router)
app.use(VueGtag, { config: { id: "G-55CXMHCLVY" } });
app.mount('#app')
