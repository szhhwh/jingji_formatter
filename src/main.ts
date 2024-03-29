import { createApp } from "vue";
import "./styles.css";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from "./App.vue";
import router from "./router/router";

const app = createApp(App)
app.use(ElementPlus)
app.use(router)
app.mount("#app");
