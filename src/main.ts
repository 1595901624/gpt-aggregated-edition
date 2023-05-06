import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import Mirror from "./components/Mirror.vue";
import MultiTabPage from "./components/tab/MultiTabPage.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import { createRouter, createWebHashHistory } from 'vue-router'

import * as ElementPlusIconsVue from '@element-plus/icons-vue'

const routes = [
  { path: '/', component: MultiTabPage },
  { path: '/mirror', component: Mirror }
]
const router = createRouter({
  history: createWebHashHistory(),
  routes
})

const app = createApp(App)
app.use(ElementPlus);
app.use(router);

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
app.mount("#app");
