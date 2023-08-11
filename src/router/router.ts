import Main from "../Page/Main.vue";
import {createRouter, createWebHistory,} from "vue-router"
import Preview from "../Page/Preview.vue";
const routes = [{path: "/", component: Main}, {path: "/preview", component: Preview}]

const router = createRouter({
    routes, history: createWebHistory()
})

export default router